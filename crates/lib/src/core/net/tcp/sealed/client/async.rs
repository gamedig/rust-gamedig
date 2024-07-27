use std::{net::SocketAddr, time::Duration};

use async_std::{
    future::timeout as timer,
    io::{ReadExt, WriteExt},
    net::TcpStream,
};

use crate::{
    error::{NetworkError, Report, Result, _metadata::NetworkProtocol},
    settings::Timeout,
};

use log::{debug, trace};

#[derive(Debug)]
pub(crate) struct AsyncTcpClient {
    addr: SocketAddr,
    stream: TcpStream,
    read_timeout: Duration,
    write_timeout: Duration,
}

#[maybe_async::async_impl]
impl super::Tcp for AsyncTcpClient {
    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self> {
        trace!("TCP::<Async>::New: Creating new TCP client for {addr} with timeout: {timeout:?}");

        let stream = match timer(timeout.connect, TcpStream::connect(addr)).await {
            Ok(Ok(stream)) => stream,
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

        Ok(Self {
            addr: *addr,
            stream,
            read_timeout: timeout.read,
            write_timeout: timeout.write,
        })
    }

    async fn read(&mut self, size: Option<u16>) -> Result<Vec<u8>> {
        trace!(
            "TCP::<Async>::Read: Reading data from {} with size: {:?}",
            self.addr,
            size
        );

        let validated_size = match size {
            Some(size) => size.min(Self::MAX_TCP_PACKET_SIZE) as usize,
            None => Self::MAX_TCP_PACKET_SIZE as usize,
        };

        let mut vec = Vec::with_capacity(validated_size);

        match timer(self.read_timeout, self.stream.read_to_end(&mut vec)).await {
            Ok(Ok(len)) => {
                if validated_size < len {
                    debug!(
                        "TCP::<Async>::Read: More data than expected. Realloc was required. \
                         Expected: {validated_size} bytes, Read: {len} bytes",
                    );
                }

                let capacity = vec.capacity();
                if capacity * (Self::VEC_CAPACITY_SHRINK_MARGIN as usize)
                    > (len * Self::VEC_CAPACITY_BASE_UNIT as usize)
                {
                    debug!(
                        "TCP::<Async>::Read: Shrink threshold exceeded. Shrinking vec to fit. \
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

        match timer(self.write_timeout, self.stream.write_all(data)).await {
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
