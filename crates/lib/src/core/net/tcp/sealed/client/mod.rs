use std::net::SocketAddr;

use std::fmt::{self, Display, Formatter};

use error_stack::{Context, Report, Result, ResultExt};

use crate::settings::Timeout;

#[cfg(feature = "client_async_std")]
mod async_std;
#[cfg(feature = "client_std")]
mod sync_std;
#[cfg(feature = "client_tokio")]
mod tokio;

#[maybe_async::maybe_async]
pub(crate) trait Tcp {
    type Error: Context;

    const DEFAULT_PACKET_SIZE: u16 = 1024;

    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self, Self::Error>
    where Self: Sized;

    async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, Self::Error>;
    async fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub(crate) struct Inner {
    #[cfg(feature = "async-tokio-client")]
    pub(crate) inner: tokio::AsyncTokioTcpClient,

    #[cfg(feature = "sync-std-client")]
    pub(crate) inner: sync_std::SyncStdTcpClient,

    #[cfg(feature = "async-std-client")]
    pub(crate) inner: async_std::AsyncStdTcpClient,
}

#[maybe_async::maybe_async]
impl Inner {
    pub(crate) async fn new(
        addr: &SocketAddr,
        timeout: Option<&Timeout>,
    ) -> Result<Self, TCPClientInnerError> {
        let timeout = timeout.unwrap_or(&Timeout::DEFAULT);

        Ok(Self {
            #[cfg(feature = "async-tokio-client")]
            inner: tokio::AsyncTokioTcpClient::new(addr, timeout)
                .await
                .map_err(Report::from)
                .attach_printable("Unable to create a tokio TCP client")
                .change_context(TCPClientInnerError)?,

            #[cfg(feature = "sync-std-client")]
            inner: sync_std::SyncStdTcpClient::new(addr, timeout)
                .map_err(Report::from)
                .attach_printable("Unable to create a sync std TCP client")
                .change_context(TCPClientInnerError)?,

            #[cfg(feature = "async-std-client")]
            inner: async_std::AsyncStdTcpClient::new(addr, timeout)
                .await
                .map_err(Report::from)
                .attach_printable("Unable to create an async std TCP client")
                .change_context(TCPClientInnerError)?,
        })
    }
}

#[derive(Debug)]
pub struct TCPClientInnerError;

impl Context for TCPClientInnerError {}

impl Display for TCPClientInnerError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "GameDig Core Net Runtime Error (tcp_client_inner)")
    }
}
