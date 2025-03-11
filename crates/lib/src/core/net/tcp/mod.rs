use {
    crate::error::Result,
    sealed::client::AbstractTcp,
    std::{net::SocketAddr, time::Duration},
};

mod sealed;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct TcpClient {
    client: sealed::client::Inner,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
}

#[maybe_async::maybe_async]
impl TcpClient {
    /// Creates a new TCP client instance.
    ///
    /// # Arguments
    ///
    /// * `addr` - A reference to the `SocketAddr` of the server to connect to.
    /// * `timeout` - An optional reference to the `Timeout` setting.
    #[allow(dead_code)]
    pub(crate) async fn new(
        addr: &SocketAddr,
        connect_timeout: Option<&Duration>,
        read_timeout: Option<&Duration>,
        write_timeout: Option<&Duration>,
    ) -> Result<Self> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Client>::New: Creating new TCP client for {addr}"
        );

        Ok(Self {
            client: sealed::client::Inner::new(addr, connect_timeout).await?,
            read_timeout: read_timeout.copied(),
            write_timeout: write_timeout.copied(),
        })
    }

    /// Reads data from the TCP stream into a buffer.
    ///
    /// # Arguments
    ///
    /// * `size` - An optional size parameter indicating the number of bytes to
    ///   read. If `None`, it will default to reading the maximum packet size.
    #[allow(dead_code)]
    pub(crate) async fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Client>::Read: Reading data from inner client"
        );

        self.client
            .inner
            .read_exact(buf, self.read_timeout.as_ref())
            .await
    }

    #[allow(dead_code)]
    pub(crate) async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Client>::Read: Reading data from inner client"
        );

        self.client
            .inner
            .read_to_end(buf, self.read_timeout.as_ref())
            .await
    }

    /// Writes data to the TCP stream.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to be written to the TCP stream.
    #[allow(dead_code)]
    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<()> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Client>::Write: Writing data to inner client"
        );

        self.client
            .inner
            .write(data, self.write_timeout.as_ref())
            .await
    }
}
