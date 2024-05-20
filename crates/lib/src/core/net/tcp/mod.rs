use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
    sync::Arc,
};

mod stream;

use error_stack::{Context, Report, ResultExt};

use crate::error::ResultConstrustor;

use self::stream::TcpSplitStream;

/// Error type representing failures within the `Tcp` module.
#[derive(Debug)]
pub(crate) struct TcpError;

impl Display for TcpError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result { fmt.write_str("GameDig Core Net Runtime Error: Tcp") }
}

impl Context for TcpError {}

/// A specialized `Result` type for `Tcp` operations.
type Result<T> = ResultConstrustor<T, TcpError>;

/// Represents a TCP client that encapsulates the functionality to read from and
/// write to a TCP stream.
///
/// This struct uses an `Arc` to manage the shared ownership of the underlying
/// `TcpSplitStream`.
pub(crate) struct Tcp {
    stream: Arc<TcpSplitStream>,
}

impl Tcp {
    /// Creates a new `Tcp` instance by establishing a TCP connection to the
    /// specified address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The socket address to connect to.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `Tcp` instance if successful, or a
    /// `TcpError` otherwise.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to establish a TCP
    /// connection.
    pub(crate) async fn new(addr: &SocketAddr) -> Result<Self> {
        Ok(Tcp {
            stream: Arc::new(
                TcpSplitStream::new(addr)
                    .await
                    .map_err(Report::from)
                    .attach_printable("Unable to initialize a new TCP split stream")
                    .change_context(TcpError)?,
            ),
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
    /// A `Result` containing the number of bytes read, or a `TcpError` if the
    /// read operation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to read from the TCP
    /// stream.
    pub(crate) async fn read(&self, buf: &mut [u8]) -> Result<usize> {
        Arc::clone(&self.stream)
            .read(buf)
            .await
            .map_err(Report::from)
            .attach_printable("Unable to read from the TCP stream")
            .change_context(TcpError)
    }

    /// Writes data to the TCP stream from the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer containing the data to write.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of bytes written, or a `TcpError` if
    /// the write operation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to write to the TCP
    /// stream.
    pub(crate) async fn write(&self, buf: &[u8]) -> Result<usize> {
        Arc::clone(&self.stream)
            .write(buf)
            .await
            .map_err(Report::from)
            .attach_printable("Unable to write to the TCP stream")
            .change_context(TcpError)
    }
}
