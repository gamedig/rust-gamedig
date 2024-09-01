mod sealed;

use sealed::client::Tcp;
use std::net::SocketAddr;

use crate::{core::prelude::Buffer, error::Result, settings::Timeout};

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct TcpClient {
    client: sealed::client::Inner,
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
    pub(crate) async fn new(addr: &SocketAddr, timeout: Option<&Timeout>) -> Result<Self> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Client>::New: Creating new TCP client for {addr} with timeout: {timeout:?}"
        );

        Ok(Self {
            client: sealed::client::Inner::new(addr, timeout).await?,
        })
    }

    /// Reads data from the TCP stream into a buffer.
    ///
    /// # Arguments
    ///
    /// * `size` - An optional size parameter indicating the number of bytes to
    ///   read. If `None`, it will default to reading the maximum packet size.
    #[allow(dead_code)]
    pub(crate) async fn read(&mut self, size: Option<usize>) -> Result<Buffer> {
        #[cfg(feature = "attribute_log")]
        log::trace!("TCP::<Client>::Read: Reading data with size: {size:?}");

        Ok(Buffer::from_socket(self.client.inner.read(size).await?))
    }

    /// Writes data to the TCP stream.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to be written to the TCP stream.
    #[allow(dead_code)]
    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<()> {
        #[cfg(feature = "attribute_log")]
        log::trace!("TCP::<Client>::Write: Writing data to the stream");

        self.client.inner.write(data).await
    }
}
