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

#[allow(dead_code)]
#[maybe_async::maybe_async]
pub(crate) trait AbstractUdp {
    async fn new(addr: SocketAddr) -> Result<Self>
    where Self: Sized;

    async fn send(&mut self, data: &[u8], timeout: Duration) -> Result<()>;

    async fn recv(&mut self, buf: &mut [u8], timeout: Duration) -> Result<()>;
}

pub(crate) struct Inner {
    #[cfg(feature = "socket_std")]
    pub(crate) inner: std::StdUdpClient,

    #[cfg(feature = "socket_tokio")]
    pub(crate) inner: tokio::TokioUdpClient,
}

#[maybe_async::maybe_async]
impl Inner {
    pub(crate) async fn new(addr: SocketAddr) -> Result<Self> {
        dev_trace!("GAMEDIG::CORE::NET::UDP::SEALED::CLIENT::INNER::<NEW>: [addr: {addr:?}]",);

        Ok(Self {
            #[cfg(feature = "socket_std")]
            inner: std::StdUdpClient::new(addr).await?,

            #[cfg(feature = "socket_tokio")]
            inner: tokio::TokioUdpClient::new(addr).await?,
        })
    }
}
