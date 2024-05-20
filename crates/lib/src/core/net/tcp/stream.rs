use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
    sync::Arc,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
};

use error_stack::{Context, Report, ResultExt};

use crate::error::ResultConstrustor;

/// Error type representing failures within the `TcpSplitStream` module.
#[derive(Debug)]
pub(super) struct TcpSplitStreamError;

impl Display for TcpSplitStreamError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str("GameDig Core Net Runtime Error: TcpSplitStream")
    }
}

impl Context for TcpSplitStreamError {}

/// A specialized `Result` type for `TcpSplitStream` operations.
type Result<T> = ResultConstrustor<T, TcpSplitStreamError>;

/// Represents a TCP connection split into separate read and write halves.
///
/// This struct utilizes `Arc` (Atomic Reference Counting) and `Mutex` for
/// thread-safe concurrent access, making it suitable for asynchronous
/// operations in a multi-threaded context.
pub(super) struct TcpSplitStream {
    read: Arc<Mutex<OwnedReadHalf>>,
    write: Arc<Mutex<OwnedWriteHalf>>,
}

impl TcpSplitStream {
    /// Establishes a new TCP connection to the specified address and splits the
    /// stream into read and write halves.
    ///
    /// # Arguments
    ///
    /// * `addr` - The socket address to connect to.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `TcpSplitStream` if successful, or
    /// a `TcpSplitStreamError` otherwise.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to establish a TCP
    /// connection to the provided address.
    pub(super) async fn new(addr: &SocketAddr) -> Result<Self> {
        // Attempt to connect to the specified socket address.
        let (orh, owh) = TcpStream::connect(addr)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to establish a TCP connection")
            .attach_printable(format!("Attempted to connect to address: {addr:?}."))
            .change_context(TcpSplitStreamError)?
            // Split the stream into read and write halves.
            .into_split();

        Ok(TcpSplitStream {
            read: Arc::new(Mutex::new(orh)),
            write: Arc::new(Mutex::new(owh)),
        })
    }

    /// Reads data from the TCP stream into the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer to store the read data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of bytes read, or a
    /// `TcpSplitStreamError` if the read operation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to read from the TCP
    /// stream.
    pub(super) async fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let read_half = Arc::clone(&self.read);
        let mut orh = read_half.lock().await;

        // Read data from the TCP stream into the provided buffer.
        orh.read(buf)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to read data from its half of the TCP split stream")
            .change_context(TcpSplitStreamError)
    }

    /// Writes data to the TCP stream from the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer containing the data to write.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of bytes written, or a
    /// `TcpSplitStreamError` if the write operation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to write to the TCP
    /// stream.
    pub(super) async fn write(&self, buf: &[u8]) -> Result<usize> {
        let write_half = Arc::clone(&self.write);
        let mut owh = write_half.lock().await;

        // Write the data from the provided buffer to the TCP stream.
        let n = owh
            .write(buf)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to write data to its half of the TCP split stream")
            .change_context(TcpSplitStreamError)?;

        // Flush the stream to ensure all data is sent.
        owh.flush()
            .await
            .map_err(Report::from)
            .attach_printable("Failed to flush the TCP write half after writing data")
            .change_context(TcpSplitStreamError)?;

        Ok(n)
    }
}
