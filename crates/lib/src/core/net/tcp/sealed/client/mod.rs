use ::std::net::SocketAddr;

use crate::{error::Result, settings::Timeout};

#[cfg(feature = "client_std")]
mod std;
#[cfg(feature = "client_tokio")]
mod tokio;

#[maybe_async::maybe_async]
pub(crate) trait Tcp {
    // The margin to shrink the buffer by
    const BUF_SHRINK_MARGIN: u8 = 32;
    // Default capacity for the buffer
    const DEFAULT_BUF_CAPACITY: u16 = 1024;

    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self>
    where Self: Sized;

    async fn read(&mut self, size: Option<usize>) -> Result<(Vec<u8>, usize)>;
    async fn write(&mut self, data: &[u8]) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct Inner {
    #[cfg(feature = "client_std")]
    pub(crate) inner: std::StdTcpClient,

    #[cfg(feature = "client_tokio")]
    pub(crate) inner: tokio::TokioTcpClient,
}

#[maybe_async::maybe_async]
impl Inner {
    pub(crate) async fn new(addr: &SocketAddr, timeout: Option<&Timeout>) -> Result<Self> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Inner>::New: Creating new TCP client for {addr} with timeout: {timeout:?}"
        );

        let timeout = timeout.unwrap_or(&Timeout::DEFAULT);

        Ok(Self {
            #[cfg(feature = "client_std")]
            inner: std::StdTcpClient::new(addr, timeout)?,

            #[cfg(feature = "client_tokio")]
            inner: tokio::TokioTcpClient::new(addr, timeout).await?,
        })
    }
}
