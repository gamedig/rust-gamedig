use {
    crate::core::error::{Report, ResultExt},
    client::{AbstractTcp, InnerTcpClient},
    std::{net::SocketAddr, time::Duration},
};

mod client;

#[derive(Debug, thiserror::Error)]
pub enum TcpClientError {
    #[error("[GameDig]::[TCP::INIT]: Failed to initialize the TCP client")]
    Init,

    #[error("[GameDig]::[TCP::READ_EXACT]: Failed to read exact number of bytes from TCP stream")]
    ReadExact,

    #[error("[GameDig]::[TCP::READ_TO_END]: Failed to read to end from TCP stream")]
    ReadToEnd,

    #[error("[GameDig]::[TCP::WRITE]: Failed to write data to TCP stream")]
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
    /// * `addr` - The `SocketAddr` of the server to connect to.
    /// * `connect_timeout` - Optional timeout for establishing the connection.
    /// * `read_timeout` - Optional timeout for reading from the stream.
    /// * `write_timeout` - Optional timeout for writing to the stream.
    pub(crate) async fn new(
        addr: SocketAddr,
        connect_timeout: Option<Duration>,
        read_timeout: Option<Duration>,
        write_timeout: Option<Duration>,
    ) -> Result<Self, Report<TcpClientError>> {
        dev_trace_fmt!("GAMEDIG::CORE::TCP::<NEW>: {:?}", |f| {
            f.debug_struct("Args")
                .field("addr", &addr)
                .field("connect_timeout", &connect_timeout)
                .field("read_timeout", &read_timeout)
                .field("write_timeout", &write_timeout)
                .finish()
        });

        let [
            valid_connect_timeout,
            valid_read_timeout,
            valid_write_timeout,
        ] = [connect_timeout, read_timeout, write_timeout].map(|opt| {
            opt.filter(|d| !d.is_zero())
                .unwrap_or(Duration::from_secs(5))
        });

        Ok(Self {
            client: InnerTcpClient::new(addr, valid_connect_timeout)
                .await
                .change_context(TcpClientError::Init)?,

            read_timeout: valid_read_timeout,
            write_timeout: valid_write_timeout,
        })
    }

    /// Reads a exact number of bytes from the TCP stream.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable slice of bytes to be filled with data read from the TCP stream.
    pub(crate) async fn read_exact(
        &mut self,
        buf: &mut [u8],
    ) -> Result<(), Report<TcpClientError>> {
        dev_trace_fmt!("GAMEDIG::CORE::TCP::<READ_EXACT>: {:?}", |f| {
            f.debug_struct("Args")
                .field("buf", format_args!("len({})", buf.len()))
                .finish()
        });

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
    pub(crate) async fn read_to_end(
        &mut self,
        buf: &mut Vec<u8>,
    ) -> Result<(), Report<TcpClientError>> {
        dev_trace_fmt!("GAMEDIG::CORE::TCP::<READ_TO_END>: {:?}", |f| {
            f.debug_struct("Args")
                .field("buf", format_args!("cap({})", buf.capacity()))
                .finish()
        });

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
    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<(), Report<TcpClientError>> {
        dev_trace_fmt!("GAMEDIG::CORE::TCP::<WRITE>: {:?}", |f| {
            f.debug_struct("Args")
                .field("data", format_args!("len({})", data.len()))
                .finish()
        });

        self.client
            .write(data, self.write_timeout)
            .await
            .change_context(TcpClientError::Write)
    }
}
