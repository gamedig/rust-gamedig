use {
    crate::error::Result,

    // Keep `::` at the beginning of the
    // path to avoid module resolution conflict
    ::std::{net::SocketAddr, time::Duration},
};

#[cfg(feature = "socket_std")]
mod std;
#[cfg(feature = "socket_tokio")]
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

    async fn read(&mut self, size: Option<usize>, timeout: Option<&Duration>) -> Result<Vec<u8>>;

    async fn write(&mut self, data: &[u8], timeout: Option<&Duration>) -> Result<()>;
}

/// An internal TCP client
///
/// This struct is used to wrap the actual TCP client implementation
/// and provide a conditional interface for the client.
#[derive(Debug)]
pub(crate) struct Inner {
    /// The standard library (blocking) TCP client
    #[cfg(feature = "socket_std")]
    pub(crate) inner: std::StdTcpClient,

    /// The Tokio (asynchronous) TCP client
    #[cfg(feature = "socket_tokio")]
    pub(crate) inner: tokio::TokioTcpClient,
}

#[maybe_async::maybe_async]
impl Inner {
    /// Creates a new instance of `Inner`, which internally holds either a `std` or `tokio` TCP client,
    /// depending on the enabled feature flag.
    ///
    /// # Arguments
    /// * `addr` - The socket address of the server you want to connect to.
    /// * `timeout` - An optional timeout value for establishing the connection.
    pub(crate) async fn new(addr: &SocketAddr, timeout: Option<&Duration>) -> Result<Self> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Inner>::New: Get new sealed TCP client for {addr}"
        );

        Ok(Self {
            #[cfg(feature = "socket_std")]
            inner: std::StdTcpClient::new(addr, timeout)?,

            #[cfg(feature = "socket_tokio")]
            inner: tokio::TokioTcpClient::new(addr, timeout).await?,
        })
    }
}
