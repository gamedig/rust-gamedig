use {
    // Keep `::` at the beginning of the
    // path to avoid module resolution conflict
    ::std::{net::SocketAddr, time::Duration},
};

#[cfg(feature = "rt_std")]
mod std;
#[cfg(feature = "rt_tokio")]
mod tokio;

#[cfg(feature = "rt_std")]
pub(crate) type InnerUdpClient = std::StdUdpClient;

#[cfg(feature = "rt_tokio")]
pub(crate) type InnerUdpClient = tokio::TokioUdpClient;

#[maybe_async::maybe_async]
pub(crate) trait AbstractUdp {
    type Error;

    async fn new(addr: SocketAddr) -> Result<Self, Self::Error>
    where Self: Sized;

    async fn send(&mut self, data: &[u8], timeout: Duration) -> Result<(), Self::Error>;

    async fn recv(&mut self, buf: &mut [u8], timeout: Duration) -> Result<usize, Self::Error>;
}
