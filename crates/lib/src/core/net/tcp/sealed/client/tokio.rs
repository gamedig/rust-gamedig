use std::{net::SocketAddr, sync::Arc, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
    time::timeout as timer,
};

use crate::{
    error::{NetworkError, Report, Result, _metadata::NetworkProtocol},
    settings::Timeout,
};

use log::{debug, trace};

#[derive(Debug)]
pub(crate) struct TokioTcpClient {
    addr: SocketAddr,
    read_timeout: Duration,
    write_timeout: Duration,
    read_stream: Arc<Mutex<OwnedReadHalf>>,
    write_stream: Arc<Mutex<OwnedWriteHalf>>,
}

#[maybe_async::async_impl]
impl super::Tcp for TokioTcpClient {
    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self> {
        trace!("TCP::<Tokio>::New: Creating new TCP client for {addr} with timeout: {timeout:?}");

        let (orh, owh) = match timer(timeout.connect, TcpStream::connect(addr)).await {
            Ok(Ok(stream)) => stream.into_split(),
            Ok(Err(e)) => {
                return Err(Report::from(e).change_context(
                    NetworkError::ConnectionError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: *addr,
                    }
                    .into(),
                ));
            }
            Err(e) => {
                return Err(Report::from(e).change_context(
                    NetworkError::TimeoutElapsedError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: *addr,
                    }
                    .into(),
                ));
            }
        };

        Ok(TokioTcpClient {
            addr: *addr,
            read_timeout: timeout.read,
            write_timeout: timeout.write,
            read_stream: Arc::new(Mutex::new(orh)),
            write_stream: Arc::new(Mutex::new(owh)),
        })
    }

    async fn read(&mut self, size: Option<u16>) -> Result<Vec<u8>> {
        trace!(
            "TCP::<Std>::Read: Reading data from {} with size: {:?}",
            self.addr,
            size
        );

        let read_half = Arc::clone(&self.read_stream);
        let mut orh = read_half.lock().await;

        let validated_size = match size {
            Some(size) => size.min(Self::MAX_TCP_PACKET_SIZE) as usize,
            None => Self::MAX_TCP_PACKET_SIZE as usize,
        };

        let mut vec = Vec::with_capacity(validated_size);

        match timer(self.read_timeout, orh.read_to_end(&mut vec)).await {
            Ok(Ok(len)) => {
                if validated_size < len {
                    debug!(
                        "TCP::<Std>::Read: More data than expected. Realloc was required. \
                         Expected: {validated_size} bytes, Read: {len} bytes",
                    );
                }

                let capacity = vec.capacity();
                if capacity * (Self::VEC_CAPACITY_SHRINK_MARGIN as usize)
                    > (len * Self::VEC_CAPACITY_BASE_UNIT as usize)
                {
                    debug!(
                        "TCP::<Std>::Read: Shrink threshold exceeded. Shrinking vec to fit. \
                         Capacity: {capacity}, Read: {len} bytes",
                    );

                    vec.shrink_to_fit();
                }

                Ok(vec)
            }
            Ok(Err(e)) => {
                Err(Report::from(e).change_context(
                    NetworkError::ReadError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: self.addr,
                    }
                    .into(),
                ))
            }
            Err(e) => {
                Err(Report::from(e).change_context(
                    NetworkError::TimeoutElapsedError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: self.addr,
                    }
                    .into(),
                ))
            }
        }
    }

    async fn write(&mut self, data: &[u8]) -> Result<()> {
        trace!(
            "TCP::<Async>::Write: Writing data to {} with size: {}",
            self.addr,
            data.len()
        );

        let write_half = Arc::clone(&self.write_stream);
        let mut owh = write_half.lock().await;

        match timer(self.write_timeout, owh.write_all(data)).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => {
                Err(Report::from(e).change_context(
                    NetworkError::WriteError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: self.addr,
                    }
                    .into(),
                ))
            }
            Err(e) => {
                Err(Report::from(e).change_context(
                    NetworkError::TimeoutElapsedError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: self.addr,
                    }
                    .into(),
                ))
            }
        }
    }
}
