use {
    crate::core::error::{Report, ResultExt},
    client::{AbstractUdp, InnerUdpClient},
    std::{net::SocketAddr, time::Duration},
};

mod client;

#[derive(Debug, thiserror::Error)]
pub enum UdpClientError {
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
    /// Creates a new UDP client instance.
    ///
    /// # Arguments
    ///
    /// * `addr` - The `SocketAddr` of the server to connect to.
    /// * `read_timeout` - Optional timeout for reading from the socket.
    /// * `write_timeout` - Optional timeout for writing to the socket.
    pub(crate) async fn new(
        addr: SocketAddr,
        read_timeout: Option<Duration>,
        write_timeout: Option<Duration>,
    ) -> Result<Self, Report<UdpClientError>> {
        dev_trace_fmt!("GAMEDIG::CORE::UDP::<NEW>: {:?}", |f| {
            f.debug_struct("Args")
                .field("addr", &addr)
                .field("read_timeout", &read_timeout)
                .field("write_timeout", &write_timeout)
                .finish()
        });

        let [valid_read_timeout, valid_write_timeout] = [read_timeout, write_timeout].map(|opt| {
            opt.filter(|d| !d.is_zero())
                .unwrap_or(Duration::from_secs(5))
        });

        Ok(Self {
            client: InnerUdpClient::new(addr)
                .await
                .change_context(UdpClientError::Init)?,

            read_timeout: valid_read_timeout,
            write_timeout: valid_write_timeout,
        })
    }

    /// Sends data to the remote address.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to be written to the UDP socket.
    pub(crate) async fn send(&mut self, data: &[u8]) -> Result<(), Report<UdpClientError>> {
        dev_trace_fmt!("GAMEDIG::CORE::NET::UDP::<SEND>: {:?}", |f| {
            f.debug_struct("Args")
                .field("data", format_args!("len({:?})", data.len()))
                .finish()
        });

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
    pub(crate) async fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Report<UdpClientError>> {
        // If the caller passed a Vec, the slice (buf) length will be 0.
        // The original type is erased at this point, so the capacity meta is unknown here.
        dev_trace_fmt!("GAMEDIG::CORE::NET::UDP::<RECV>: {:?}", |f| {
            f.debug_struct("Args")
                .field("buf", format_args!("len({})", buf.len()))
                .finish()
        });

        self.client
            .recv(buf, self.read_timeout)
            .await
            .change_context(UdpClientError::Recv)
    }
}
