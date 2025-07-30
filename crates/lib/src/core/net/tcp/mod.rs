use {
    crate::error::Result,
    sealed::client::AbstractTcp,
    std::{net::SocketAddr, time::Duration},
};

mod sealed;

#[derive(Debug)]
pub(crate) struct TcpClient {
    client: sealed::client::Inner,
    read_timeout: Duration,
    write_timeout: Duration,
}

#[maybe_async::maybe_async]
impl TcpClient {
    /// Creates a new TCP client instance.
    ///
    /// # Arguments
    ///
    /// * `addr` - The `SocketAddr` of the server to connect to.
    /// * `connect_timeout` - Optional timeout for establishing the connection.
    /// * `read_timeout` - Optional timeout for reading from the stream.
    /// * `write_timeout` - Optional timeout for writing to the stream.
    pub(crate) async fn new(
        addr: SocketAddr,
        connect_timeout: Option<Duration>,
        read_timeout: Option<Duration>,
        write_timeout: Option<Duration>,
    ) -> Result<Self> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::CLIENT::<NEW>: [addr: {:?}, connect_timeout: {:?}, \
             read_timeout: {:?}, write_timeout: {:?}]",
        );

        let [
            valid_connect_timeout,
            valid_read_timeout,
            valid_write_timeout,
        ] = [connect_timeout, read_timeout, write_timeout].map(|opt| {
            opt.filter(|d| !d.is_zero())
                .unwrap_or(Duration::from_secs(5))
        });

        Ok(Self {
            client: sealed::client::Inner::new(addr, valid_connect_timeout).await?,
            read_timeout: valid_read_timeout,
            write_timeout: valid_write_timeout,
        })
    }

    /// Reads a exact number of bytes from the TCP stream.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable slice of bytes to be filled with data read from the TCP stream.
    pub(crate) async fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::CLIENT::<READ_EXACT>: [buf: len({:?})]",
            buf.len()
        );

        self.client.inner.read_exact(buf, self.read_timeout).await
    }

    /// Reads data from the TCP stream until EOF.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable vector of bytes to be filled with data read from the TCP stream.
    pub(crate) async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::CLIENT::<READ_TO_END>: [buf: cap({:?})]",
            buf.capacity()
        );

        self.client.inner.read_to_end(buf, self.read_timeout).await
    }

    /// Writes data to the TCP stream.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to be written to the TCP stream.
    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<()> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::CLIENT::<WRITE>: [data: len({:?})]",
            data.len()
        );

        self.client.inner.write(data, self.write_timeout).await
    }
}
