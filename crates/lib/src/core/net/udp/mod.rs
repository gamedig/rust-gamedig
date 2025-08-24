use {
    crate::error::Result,
    sealed::client::AbstractUdp,
    std::{net::SocketAddr, time::Duration},
};

mod sealed;

pub(crate) struct UdpClient {
    client: sealed::client::Inner,
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
    ) -> Result<Self> {
        dev_trace!(
            "GAMEDIG::CORE::NET::UDP::CLIENT::<NEW>: [addr: {addr:?}, read_timeout: \
             {read_timeout:?}, write_timeout: {write_timeout:?}]",
        );

        let [valid_read_timeout, valid_write_timeout] = [read_timeout, write_timeout].map(|opt| {
            opt.filter(|d| !d.is_zero())
                .unwrap_or(Duration::from_secs(5))
        });

        Ok(Self {
            client: sealed::client::Inner::new(addr).await?,
            read_timeout: valid_read_timeout,
            write_timeout: valid_write_timeout,
        })
    }

    /// Sends data to the remote address.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to be written to the UDP socket.
    pub(crate) async fn send(&mut self, data: &[u8]) -> Result<()> {
        dev_trace!(
            "GAMEDIG::CORE::NET::UDP::CLIENT::<SEND>: [data: len({:?})]",
            data.len()
        );

        self.client.inner.send(data, self.write_timeout).await
    }

    /// Receives a single datagram message.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable slice of bytes to be filled with the received data.
    ///
    /// **Note**: If a message is too long to fit in the supplied buffer, excess bytes may be discarded.
    pub(crate) async fn recv(&mut self, buf: &mut [u8]) -> Result<()> {
        // If the caller passed a Vec, the slice (buf) length will be 0.
        // The original type is erased at this point, so the capacity meta is unknown here.
        dev_trace!(
            "GAMEDIG::CORE::NET::UDP::CLIENT::<RECV>: [buf: len({:?})]",
            buf.len()
        );

        self.client.inner.recv(buf, self.read_timeout).await
    }
}
