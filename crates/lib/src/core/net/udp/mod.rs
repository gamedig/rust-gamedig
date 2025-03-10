use {
    crate::{core::Buffer, error::Result},
    sealed::client::AbstractUdp,
    std::time::Duration,
};

mod sealed;

pub(crate) struct UdpClient {
    client: sealed::client::Inner,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
}

#[maybe_async::maybe_async]
impl UdpClient {
    #[allow(dead_code)]
    async fn new(
        addr: &std::net::SocketAddr,
        read_timeout: Option<Duration>,
        write_timeout: Option<Duration>,
    ) -> Result<Self> {
        #[cfg(feature = "attribute_log")]
        log::trace!("UDP::<Client>::New: Creating new UDP client for {addr}");

        Ok(Self {
            client: sealed::client::Inner::new(addr).await?,
            read_timeout: read_timeout,
            write_timeout: write_timeout,
        })
    }

    #[allow(dead_code)]
    async fn send(&mut self, data: &[u8]) -> Result<()> {
        #[cfg(feature = "attribute_log")]
        log::trace!("UDP::<Client>::Send: Sending data to the server");

        self.client
            .inner
            .send(data, self.write_timeout.as_ref())
            .await
    }

    #[allow(dead_code)]
    async fn recv(&mut self, size: Option<usize>) -> Result<Buffer> {
        #[cfg(feature = "attribute_log")]
        log::trace!("UDP::<Client>::Recv: Receiving data from the server");

        Ok(Buffer::new(
            self.client
                .inner
                .recv(size, self.read_timeout.as_ref())
                .await?,
        ))
    }
}
