use crate::error::ResultConstrustor;
use error_stack::{Context, Report, ResultExt};
use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
    sync::Arc,
};
use tokio::net::UdpSocket;

/// Error type representing failures within the `Udp` module.
#[derive(Debug)]
pub(crate) struct UdpError;

impl Display for UdpError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result { fmt.write_str("GameDig Core Net Runtime Error: Udp") }
}

impl Context for UdpError {}

/// A specialized `Result` type for `Udp` operations.
type Result<T> = ResultConstrustor<T, UdpError>;

/// Represents a UDP client that encapsulates the functionality to send and
/// receive datagrams.
///
/// This struct uses an `Arc` to manage the shared ownership of the underlying
/// `UdpSocket`, allowing it to be safely shared across multiple asynchronous
/// tasks.
pub(crate) struct Udp {
    socket: Arc<UdpSocket>,
}

impl Udp {
    /// Creates a new `Udp` instance by binding to the specified socket address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The socket address to bind to.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `Udp` instance if successful, or a
    /// `UdpError` otherwise.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to bind to the specified
    /// address.
    pub(crate) async fn new(addr: &SocketAddr) -> Result<Self> {
        Ok(Udp {
            socket: Arc::new(
                UdpSocket::bind(addr)
                    .await
                    .map_err(Report::from)
                    .attach_printable("Failed to bind a UDP socket")
                    .change_context(UdpError)?,
            ),
        })
    }

    /// Sends data through the UDP socket to the specified address.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer containing the data to send.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of bytes sent, or a `UdpError` if the
    /// send operation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to send the data.
    pub(crate) async fn send(&self, buf: &[u8]) -> Result<usize> {
        Ok(Arc::clone(&self.socket)
            .send(buf)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to send data through the UDP socket")
            .change_context(UdpError)?)
    }

    /// Receives data from the UDP socket into the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer to store the received data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of bytes received, or a `UdpError` if
    /// the receive operation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to receive the data.
    pub(crate) async fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        Ok(Arc::clone(&self.socket)
            .recv(buf)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to receive data through the UDP socket")
            .change_context(UdpError)?)
    }
}
