use {
    // Keep `::` at the beginning of the
    // path to avoid module resolution conflict
    ::std::{net::SocketAddr, time::Duration},
};

#[cfg(feature = "socket_std")]
mod std;
#[cfg(feature = "socket_tokio")]
mod tokio;

#[cfg(feature = "socket_std")]
pub(crate) type InnerTcpClient = std::StdTcpClient;

#[cfg(feature = "socket_tokio")]
pub(crate) type InnerTcpClient = tokio::TokioTcpClient;

/// Abstract layer for TCP operations
///
/// This trait is used to define the common interface for TCP operations,
/// which can be implemented across different runtime implementations.
#[maybe_async::maybe_async]
pub(crate) trait AbstractTcp {
    type Error;

    async fn new(addr: SocketAddr, timeout: Duration) -> Result<Self, Self::Error>
    where Self: Sized;

    async fn read_exact(&mut self, buf: &mut [u8], timeout: Duration) -> Result<(), Self::Error>;

    async fn read_to_end(
        &mut self,
        buf: &mut Vec<u8>,
        timeout: Duration,
    ) -> Result<usize, Self::Error>;

    async fn write(&mut self, data: &[u8], timeout: Duration) -> Result<(), Self::Error>;
}
