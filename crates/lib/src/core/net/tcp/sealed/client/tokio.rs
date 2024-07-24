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

#[derive(Debug)]
pub(crate) struct AsyncTokioTcpClient {
    addr: SocketAddr,
    read_timeout: Duration,
    write_timeout: Duration,
    read_stream: Arc<Mutex<OwnedReadHalf>>,
    write_stream: Arc<Mutex<OwnedWriteHalf>>,
}

#[maybe_async::async_impl]
impl super::Tcp for AsyncTokioTcpClient {
    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self> {
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

        Ok(AsyncTokioTcpClient {
            addr: *addr,
            read_timeout: timeout.read,
            write_timeout: timeout.write,
            read_stream: Arc::new(Mutex::new(orh)),
            write_stream: Arc::new(Mutex::new(owh)),
        })
    }

    async fn read(&mut self, size: Option<u16>) -> Result<Vec<u8>> {
        let read_half = Arc::clone(&self.read_stream);
        let mut orh = read_half.lock().await;

        let mut vec = Vec::with_capacity(match size {
            Some(size) => size.min(Self::MAX_TCP_PACKET_SIZE) as usize,
            None => Self::MAX_TCP_PACKET_SIZE as usize,
        });

        match timer(self.read_timeout, orh.read_to_end(&mut vec)).await {
            Ok(Ok(len)) => {
                if vec.capacity() * (Self::VEC_CAPACITY_SHRINK_MARGIN as usize) > (len << 7) {
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
