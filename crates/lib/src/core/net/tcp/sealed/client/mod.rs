use ::std::net::SocketAddr;

use crate::{error::Result, settings::Timeout};

use log::trace;

#[cfg(feature = "client_async_std")]
mod r#async;
#[cfg(feature = "client_std")]
mod std;
#[cfg(feature = "client_tokio")]
mod tokio;

#[maybe_async::maybe_async]
pub(crate) trait Tcp {
    // MAX_TCP_PACKET_SIZE = ETHERNET_MTU - (IP_HEADER_SIZE + TCP_HEADER_SIZE)
    const MAX_TCP_PACKET_SIZE: u16 = 1460;

    // 100% = 128 or 1 << 7
    const VEC_CAPACITY_BASE_UNIT: u8 = (1 << 7);
    // 120% = 128 * 1.2 = 153.6 = 153
    const VEC_CAPACITY_SHRINK_MARGIN: u8 = (Self::VEC_CAPACITY_BASE_UNIT as f32 * 1.2) as u8;

    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self>
    where Self: Sized;

    async fn read(&mut self, size: Option<u16>) -> Result<Vec<u8>>;
    async fn write(&mut self, data: &[u8]) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct Inner {
    #[cfg(feature = "client_async_std")]
    pub(crate) inner: r#async::AsyncTcpClient,

    #[cfg(feature = "client_std")]
    pub(crate) inner: std::StdTcpClient,

    #[cfg(feature = "client_tokio")]
    pub(crate) inner: tokio::TokioTcpClient,
}

#[maybe_async::maybe_async]
impl Inner {
    pub(crate) async fn new(addr: &SocketAddr, timeout: Option<&Timeout>) -> Result<Self> {
        trace!("TCP::<Inner>::New: Creating new TCP client for {addr} with timeout: {timeout:?}");

        let timeout = timeout.unwrap_or(&Timeout::DEFAULT);

        Ok(Self {
            #[cfg(feature = "client_async_std")]
            inner: r#async::AsyncTcpClient::new(addr, timeout).await?,

            #[cfg(feature = "client_std")]
            inner: std::StdTcpClient::new(addr, timeout)?,

            #[cfg(feature = "client_tokio")]
            inner: tokio::TokioTcpClient::new(addr, timeout).await?,
        })
    }
}
