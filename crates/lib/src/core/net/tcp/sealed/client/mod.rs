use std::net::SocketAddr;

use crate::{
    error::{
        NetworkError,
        Report,
        Result,
        ResultExt,
        _metadata::{NetworkInterface, NetworkProtocol},
    },
    settings::Timeout,
};

#[cfg(feature = "client_async_std")]
mod async_std;
#[cfg(feature = "client_std")]
mod sync_std;
#[cfg(feature = "client_tokio")]
mod tokio;

#[maybe_async::maybe_async]
pub(crate) trait Tcp {
    const DEFAULT_PACKET_SIZE: u16 = 1024;

    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self>
    where Self: Sized;

    async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>>;
    async fn write(&mut self, data: &[u8]) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct Inner {
    #[cfg(feature = "client_async_std")]
    pub(crate) inner: async_std::AsyncStdTcpClient,

    #[cfg(feature = "client_std")]
    pub(crate) inner: sync_std::SyncStdTcpClient,

    #[cfg(feature = "client_tokio")]
    pub(crate) inner: tokio::AsyncTokioTcpClient,
}

#[maybe_async::maybe_async]
impl Inner {
    pub(crate) async fn new(addr: &SocketAddr, timeout: Option<&Timeout>) -> Result<Self> {
        let timeout = timeout.unwrap_or(&Timeout::DEFAULT);

        Ok(Self {
            #[cfg(feature = "client_async_std")]
            inner: async_std::AsyncStdTcpClient::new(addr, timeout)
                .await
                .map_err(Report::from)
                .attach_printable("Unable to create an async std TCP client")
                .change_context(
                    NetworkError::ConnectionError {
                        _protocol: NetworkProtocol::Tcp,
                        _interface: NetworkInterface::SealedClientInner,
                    }
                    .into(),
                )?,

            #[cfg(feature = "client_std")]
            inner: sync_std::SyncStdTcpClient::new(addr, timeout)
                .map_err(Report::from)
                .attach_printable("Unable to create a sync std TCP client")
                .change_context(
                    NetworkError::ConnectionError {
                        _protocol: NetworkProtocol::Tcp,
                        _interface: NetworkInterface::SealedClientInner,
                    }
                    .into(),
                )?,

            #[cfg(feature = "client_tokio")]
            inner: tokio::AsyncTokioTcpClient::new(addr, timeout)
                .await
                .map_err(Report::from)
                .attach_printable("Unable to create an async tokio TCP client")
                .change_context(
                    NetworkError::ConnectionError {
                        _protocol: NetworkProtocol::Tcp,
                        _interface: NetworkInterface::SealedClientInner,
                    }
                    .into(),
                )?,
        })
    }
}
