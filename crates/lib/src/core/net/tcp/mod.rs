#[cfg(feature = "async-std-client")]
mod async_std;
#[cfg(feature = "sync-std-client")]
mod sync_std;
#[cfg(feature = "async-tokio-client")]
mod tokio;

use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
};

use error_stack::{Context, Report, Result, ResultExt};

pub(crate) struct TcpClient {
    #[cfg(feature = "async-tokio-client")]
    inner: tokio::AsyncTokioTcpClient,

    #[cfg(feature = "sync-std-client")]
    inner: sync_std::SyncStdTcpClient,

    #[cfg(feature = "async-std-client")]
    inner: async_std::AsyncStdTcpClient,
}

#[maybe_async::maybe_async]
impl TcpClient {
    pub(crate) async fn new(addr: &SocketAddr) -> Result<Self, TCPClientError> {
        Ok(Self {
            #[cfg(feature = "async-tokio-client")]
            inner: tokio::AsyncTokioTcpClient::new(addr)
                .await
                .map_err(Report::from)
                .attach_printable("Unable to create a tokio TCP client")
                .change_context(TCPClientError)?,

            #[cfg(feature = "sync-std-client")]
            inner: sync_std::SyncStdTcpClient::new(addr)
                .map_err(Report::from)
                .attach_printable("Unable to create a sync std TCP client")
                .change_context(TCPClientError)?,

            #[cfg(feature = "async-std-client")]
            inner: async_std::AsyncStdTcpClient::new(addr)
                .await
                .map_err(Report::from)
                .attach_printable("Unable to create an async std TCP client")
                .change_context(TCPClientError)?,
        })
    }

    pub(crate) async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, TCPClientError> {
        Ok(self
            .inner
            .read(size)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to read data from the TCP Client")
            .change_context(TCPClientError)?)
    }

    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<(), TCPClientError> {
        Ok(self
            .inner
            .write(data)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to write data to the TCP Client")
            .change_context(TCPClientError)?)
    }
}

#[derive(Debug)]
pub struct TCPClientError;

impl Context for TCPClientError {}

impl Display for TCPClientError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "GameDig Core Net Runtime Error (tcp_client)")
    }
}

#[maybe_async::maybe_async]
pub(super) trait Tcp {
    type Error: Context;

    const DEFAULT_PACKET_SIZE: u16 = 1024;

    async fn new(addr: &SocketAddr) -> Result<Self, Self::Error>
    where Self: Sized;

    async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, Self::Error>;
    async fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}
