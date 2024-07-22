use std::net::SocketAddr;

use crate::{error::Result, settings::Timeout};

#[cfg(feature = "client_async_std")]
mod async_std;
#[cfg(feature = "client_std")]
mod sync_std;
#[cfg(feature = "client_tokio")]
mod tokio;

#[maybe_async::maybe_async]
pub(crate) trait Tcp {
    // MAX_TCP_PACKET_SIZE = ETHERNET_MTU - (IP_HEADER_SIZE + TCP_HEADER_SIZE)
    const MAX_TCP_PACKET_SIZE: u16 = 1500 - (20 + 20) as u16;

    // 128 represents the shrink factor as 100%
    // 256 represents the base unit (1 << 8)
    // Calculate 1% of the shrink factor as: (128 * 256) / 100 = 327
    // Calculate 20% of the shrink factor as: 20 * 327 = 6540
    // Convert this to a percentage of the base unit: 6540 / 256 = 25
    // Finally, calculate the margin: (((128 - 25) * 256) >> 7) = 206
    const VEC_CAPACITY_SHRINK_MARGIN: u8 =
        (((128_u16 - (20 * (128_u16 * 256 / 100) / 256)) * 256) >> 7) as u8;

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
            inner: async_std::AsyncStdTcpClient::new(addr, timeout).await?,

            #[cfg(feature = "client_std")]
            inner: sync_std::SyncStdTcpClient::new(addr, timeout)?,

            #[cfg(feature = "client_tokio")]
            inner: tokio::AsyncTokioTcpClient::new(addr, timeout).await?,
        })
    }
}
