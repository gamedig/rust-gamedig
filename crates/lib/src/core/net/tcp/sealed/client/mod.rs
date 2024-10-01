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

/// Abstract layer for TCP operations
///
/// This trait is used to define the common interface for TCP operations,
/// which can be implemented across different runtime implementations.
#[maybe_async::maybe_async]
pub(crate) trait AbstractTcp {
    /// The margin by which the buffer can be shrunk
    const BUF_SHRINK_MARGIN: u8 = 32;
    // Default capacity for the buffer
    const DEFAULT_BUF_CAPACITY: u16 = 1024;

    async fn new(addr: &SocketAddr, timeout: Option<&Duration>) -> Result<Self>
    where Self: Sized;

    async fn read(
        &mut self,
        size: Option<usize>,
        timeout: Option<&Duration>,
    ) -> Result<(Vec<u8>, usize)>;

    async fn write(&mut self, data: &[u8], timeout: Option<&Duration>) -> Result<()>;
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
    pub(crate) async fn new(addr: &SocketAddr, timeout: Option<&Duration>) -> Result<Self> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Inner>::New: Creating new TCP client for {addr} with timeout: {timeout:?}"
        );

        Ok(Self {
            #[cfg(feature = "client_std")]
            inner: std::StdTcpClient::new(addr, timeout)?,

            #[cfg(feature = "client_tokio")]
            inner: tokio::TokioTcpClient::new(addr, timeout).await?,
        })
    }
}
