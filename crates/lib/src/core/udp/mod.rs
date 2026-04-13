use {
    super::{
        ToSocketAddr,
        error::{Report, ResultExt},
    },
    client::{AbstractUdp, InnerUdpClient},
    std::time::Duration,
};

mod client;

#[derive(Debug, thiserror::Error)]
pub enum UdpClientError {
    #[error("[GameDig]::[UDP::DNS]: Failed to resolve socket address")]
    Dns,

    #[error("[GameDig]::[UDP::INIT]: Failed to initialize the client")]
    Init,

    #[error("[GameDig]::[UDP::SEND]: Failed to send data to the socket")]
    Send,

    #[error("[GameDig]::[UDP::RECV]: Failed to receive data from the socket")]
    Recv,
}

pub(crate) struct UdpClient {
    client: InnerUdpClient,
    read_timeout: Duration,
    write_timeout: Duration,
}

#[maybe_async::maybe_async]
impl UdpClient {
    const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

    /// Creates a new UDP client instance.
    ///
    /// # Arguments
    ///
    /// * `addr` - The socket address of the server to connect to.
    /// * `read_timeout` - Optional timeout for reading from the socket.
    /// * `write_timeout` - Optional timeout for writing to the socket.
    #[cfg_attr(
        feature = "ext_tracing",
        tracing::instrument(
            level = "trace",
            fields(
                addr = ?addr,
                read_timeout = ?read_timeout,
                write_timeout = ?write_timeout,
            )
        )
    )]
    pub(crate) async fn new<A: ToSocketAddr>(
        addr: A,
        read_timeout: Option<Duration>,
        write_timeout: Option<Duration>,
    ) -> Result<Self, Report<UdpClientError>> {
        let [read_timeout, write_timeout] = [read_timeout, write_timeout].map(|opt| {
            opt.filter(|d| !d.is_zero())
                .unwrap_or(Self::DEFAULT_TIMEOUT)
        });

        Ok(Self {
            client: InnerUdpClient::new(
                addr.to_socket_addr()
                    .await
                    .change_context(UdpClientError::Dns)?,
            )
            .await
            .change_context(UdpClientError::Init)?,

            read_timeout,
            write_timeout,
        })
    }

    /// Sends data to the remote address.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to be written to the UDP socket.
    #[cfg_attr(
        feature = "ext_tracing",
        tracing::instrument(
            level = "trace",
            skip(self),
            fields(
                data = data
            )
        )
    )]
    pub(crate) async fn send(&mut self, data: &[u8]) -> Result<(), Report<UdpClientError>> {
        self.client
            .send(data, self.write_timeout)
            .await
            .change_context(UdpClientError::Send)
    }

    /// Receives a single datagram message.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable slice of bytes to be filled with the received data.
    ///
    /// **Note**: If a message is too long to fit in the supplied buffer, excess bytes may be discarded.
    #[cfg_attr(feature = "ext_tracing", tracing::instrument(level = "trace", skip(self, buf)))]
    pub(crate) async fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Report<UdpClientError>> {
        self.client
            .recv(buf, self.read_timeout)
            .await
            .change_context(UdpClientError::Recv)
    }

    /// Sets the read timeout.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout duration for read operations. If zero, a default
    ///   of 5 seconds is used.
    ///
    /// **Note**: This updates the client configuration only. The new timeout may
    /// not be applied to the underlying socket until the next receive operation.
    #[cfg_attr(feature = "ext_tracing", tracing::instrument(level = "trace", skip(self)))]
    pub(crate) fn set_read_timeout(&mut self, timeout: Duration) {
        self.read_timeout = if timeout.is_zero() {
            Self::DEFAULT_TIMEOUT
        } else {
            timeout
        };
    }

    /// Sets the write timeout.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout duration for write operations. If zero, a default
    ///   of 5 seconds is used.
    ///
    /// **Note**: This updates the client configuration only. The new timeout may
    /// not be applied to the underlying socket until the next send operation.
    #[cfg_attr(feature = "ext_tracing", tracing::instrument(level = "trace", skip(self)))]
    pub(crate) fn set_write_timeout(&mut self, timeout: Duration) {
        self.write_timeout = if timeout.is_zero() {
            Self::DEFAULT_TIMEOUT
        } else {
            timeout
        };
    }
}
