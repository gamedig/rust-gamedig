use {
    super::{
        ToSocketAddr,
        error::{Report, ResultExt},
    },
    client::{AbstractTcp, InnerTcpClient},
    std::time::Duration,
};

mod client;

#[derive(Debug, thiserror::Error)]
pub enum TcpClientError {
    #[error("[GameDig]::[TCP::DNS]: Failed to resolve socket address")]
    Dns,

    #[error("[GameDig]::[TCP::INIT]: Underlying client failed during initialization")]
    Init,

    #[error("[GameDig]::[TCP::READ_EXACT]: Underlying client failed while reading exact")]
    ReadExact,

    #[error("[GameDig]::[TCP::READ_TO_END]: Underlying client failed while reading to end")]
    ReadToEnd,

    #[error("[GameDig]::[TCP::WRITE]: Underlying client failed while writing")]
    Write,
}

#[derive(Debug)]
pub(crate) struct TcpClient {
    client: InnerTcpClient,
    read_timeout: Duration,
    write_timeout: Duration,
}

#[maybe_async::maybe_async]
impl TcpClient {
    /// Creates a new TCP client instance.
    ///
    /// # Arguments
    ///
    /// * `addr` - The socket address of the server to connect to.
    /// * `connect_timeout` - Optional timeout for establishing the connection.
    /// * `read_timeout` - Optional timeout for reading from the stream.
    /// * `write_timeout` - Optional timeout for writing to the stream.
    #[cfg_attr(
        feature = "ext_tracing",
        tracing::instrument(
            level = "trace",
            fields(
                addr = ?addr,
                connect_timeout = ?connect_timeout,
                read_timeout = ?read_timeout,
                write_timeout = ?write_timeout,
            )
        )
    )]
    pub(crate) async fn new<A: ToSocketAddr>(
        addr: A,
        connect_timeout: Option<Duration>,
        read_timeout: Option<Duration>,
        write_timeout: Option<Duration>,
    ) -> Result<Self, Report<TcpClientError>> {
        let [connect_timeout, read_timeout, write_timeout] = [connect_timeout, read_timeout, write_timeout].map(|opt| {
            opt.filter(|d| !d.is_zero())
                .unwrap_or(Duration::from_secs(5))
        });

        Ok(Self {
            client: InnerTcpClient::new(
                addr.to_socket_addr()
                    .await
                    .change_context(TcpClientError::Dns)?,
                connect_timeout,
            )
            .await
            .change_context(TcpClientError::Init)?,

            read_timeout,
            write_timeout,
        })
    }

    /// Reads a exact number of bytes from the TCP stream.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable slice of bytes to be filled with data read from the TCP stream.
    #[cfg_attr(feature = "ext_tracing", tracing::instrument(level = "trace", skip(self, buf)))]
    pub(crate) async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Report<TcpClientError>> {
        self.client
            .read_exact(buf, self.read_timeout)
            .await
            .change_context(TcpClientError::ReadExact)
    }

    /// Reads data from the TCP stream until EOF.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable vector of bytes to be filled with data read from the TCP stream.
    #[cfg_attr(feature = "ext_tracing", tracing::instrument(level = "trace", skip(self, buf)))]
    pub(crate) async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Report<TcpClientError>> {
        self.client
            .read_to_end(buf, self.read_timeout)
            .await
            .change_context(TcpClientError::ReadToEnd)
    }

    /// Writes data to the TCP stream.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to be written to the TCP stream.
    #[cfg_attr(feature = "ext_tracing", tracing::instrument(level = "trace", skip(self, data)))]
    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<(), Report<TcpClientError>> {
        self.client
            .write(data, self.write_timeout)
            .await
            .change_context(TcpClientError::Write)
    }
}
