use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
};

use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
    sync::Arc,
};

use error_stack::{Context, Report, Result, ResultExt};

/// Represents a TCP connection split into separate read and write halves.
///
/// This struct utilizes `Arc` (Atomic Reference Counting) and `Mutex` for
/// thread-safe concurrent access, making it suitable for asynchronous
/// operations in a multi-threaded context.
pub(super) struct AsyncTokioTcpClient {
    read_stream: Arc<Mutex<OwnedReadHalf>>,
    write_stream: Arc<Mutex<OwnedWriteHalf>>,
}

#[maybe_async::async_impl]
impl super::Tcp for AsyncTokioTcpClient {
    type Error = AsyncTokioTcpClientError;

    async fn new(addr: &SocketAddr) -> Result<Self, AsyncTokioTcpClientError> {
        // Attempt to connect to the specified socket address.
        let (orh, owh) = TcpStream::connect(addr)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to establish a TCP connection")
            .attach_printable(format!("Attempted to connect to address: {addr:?}."))
            .change_context(AsyncTokioTcpClientError)?
            // Split the stream into read and write halves.
            .into_split();

        Ok(AsyncTokioTcpClient {
            read_stream: Arc::new(Mutex::new(orh)),
            write_stream: Arc::new(Mutex::new(owh)),
        })
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, AsyncTokioTcpClientError> {
        let read_half = Arc::clone(&self.read_stream);
        let mut orh = read_half.lock().await;

        // Read data from the TCP stream into the provided buffer.
        orh.read(buf)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to read data from its half of the TCP split stream")
            .change_context(AsyncTokioTcpClientError)
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize, AsyncTokioTcpClientError> {
        let write_half = Arc::clone(&self.write_stream);
        let mut owh = write_half.lock().await;

        // Write the data from the provided buffer to the TCP stream.
        let n = owh
            .write(buf)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to write data to its half of the TCP split stream")
            .change_context(AsyncTokioTcpClientError)?;

        // Flush the stream to ensure all data is sent.
        owh.flush()
            .await
            .map_err(Report::from)
            .attach_printable("Failed to flush the TCP write half after writing data")
            .change_context(AsyncTokioTcpClientError)?;

        Ok(n)
    }
}

#[derive(Debug)]
pub struct AsyncTokioTcpClientError;

impl Context for AsyncTokioTcpClientError {}

impl Display for AsyncTokioTcpClientError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "GameDig Core Net Async Tokio Runtime Error: AsyncTokioTcpClient"
        )
    }
}
