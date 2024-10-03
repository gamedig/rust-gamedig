use {
    crate::error::Result,

    // Keep `::` at the beginning of the
    // path to avoid module resolution conflict
    ::std::{net::SocketAddr, time::Duration},
};

#[cfg(feature = "client_std")]
mod std;
#[cfg(feature = "client_tokio")]
mod tokio;

#[allow(dead_code)]
#[maybe_async::maybe_async]
pub(crate) trait AbstractUdp {
    // The margin to shrink the buffer by
    const BUF_SHRINK_MARGIN: u8 = 32;
    // Default capacity for the buffer
    const DEFAULT_BUF_CAPACITY: u16 = 1024;

    async fn new(addr: &SocketAddr) -> Result<Self>
    where Self: Sized;

    async fn send(&mut self, data: &[u8], timeout: Option<&Duration>) -> Result<()>;
    async fn recv(
        &mut self,
        size: Option<usize>,
        timeout: Option<&Duration>,
    ) -> Result<(Vec<u8>, usize)>;
}

pub(crate) struct Inner {
    #[cfg(feature = "client_std")]
    pub(crate) inner: std::StdUdpClient,

    #[cfg(feature = "client_tokio")]
    pub(crate) inner: tokio::TokioUdpClient,
}

#[maybe_async::maybe_async]
impl Inner {
    pub(crate) async fn new(addr: &SocketAddr) -> Result<Self> {
        #[cfg(feature = "attribute_log")]
        log::trace!("UDP::<Inner>::New: Creating new UDP client for {addr}");

        Ok(Self {
            #[cfg(feature = "client_std")]
            inner: std::StdUdpClient::new(addr).await?,

            #[cfg(feature = "client_tokio")]
            inner: tokio::TokioUdpClient::new(addr).await?,
        })
    }
}
