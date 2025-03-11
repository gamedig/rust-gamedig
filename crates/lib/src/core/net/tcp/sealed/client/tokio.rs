use {
    crate::error::{
        NetworkError,
        Report,
        Result,
        diagnostic::{FailureReason, Recommendation},
    },
    std::{net::SocketAddr, time::Duration},
    tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::{
            TcpStream,
            tcp::{OwnedReadHalf, OwnedWriteHalf},
        },
        sync::Mutex,
        time::timeout as timer,
    },
};

#[derive(Debug)]
pub(crate) struct TokioTcpClient {
    peer_addr: SocketAddr,
    read_stream: Mutex<OwnedReadHalf>,
    write_stream: Mutex<OwnedWriteHalf>,
}

#[maybe_async::async_impl]
impl super::AbstractTcp for TokioTcpClient {
    async fn new(addr: &SocketAddr, timeout: Option<&Duration>) -> Result<Self> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Tokio>::New: Creating new TCP client for {addr}"
        );

        let valid_timeout = match timeout {
            Some(timeout) => {
                match timeout.is_zero() {
                    true => Duration::from_secs(5),
                    false => *timeout,
                }
            }

            None => Duration::from_secs(5),
        };

        #[cfg(feature = "_DEV_LOG")]
        log::debug!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Tokio>::New: Attempting to connect to {addr:?} with a timeout of {valid_timeout:?}",
        );

        let (orh, owh) = match timer(valid_timeout, TcpStream::connect(*addr)).await {
            // Connection established successfully
            Ok(Ok(stream)) => {
                #[cfg(feature = "_DEV_LOG")]
                log::debug!(
                    target: crate::log::EventTarget::GAMEDIG_DEV,
                    "TCP::<Tokio>::New: Successfully connected to {addr:?}",
                );

                stream.into_split()
            }

            // Error during the connection attempt
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(NetworkError::TcpConnectionError { peer_addr: *addr }.into())
                    .attach_printable(FailureReason::new(
                        "Failed to establish a TCP connection due to an underlying I/O error.",
                    ))
                    .attach_printable(Recommendation::new(format!(
                        "Verify the server address ({addr:?}) is reachable, ensure the server is \
                         running, and that no firewall or network restrictions are blocking the \
                         connection."
                    ))));
            }

            // Connection attempt timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpTimeoutElapsedError { peer_addr: *addr }.into(),
                    )
                    .attach_printable(FailureReason::new(
                        "The connection attempt exceeded the specified timeout duration.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Check the server's status for high traffic or downtime, and consider \
                         increasing the timeout duration for distant or busy servers.",
                    )));
            }
        };

        Ok(TokioTcpClient {
            peer_addr: *addr,
            read_stream: Mutex::new(orh),
            write_stream: Mutex::new(owh),
        })
    }

    async fn read_exact(&mut self, buf: &mut [u8], timeout: Option<&Duration>) -> Result<()> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Tokio>::Read: Reading data from {}",
            &self.peer_addr,
        );

        // Await the read stream lock
        let mut orh_mg = self.read_stream.lock().await;
        let orh = &mut *orh_mg;

        // Validate the timeout duration
        let timeout = match timeout {
            Some(timeout) => {
                match timeout.is_zero() {
                    true => Duration::from_secs(5),
                    false => *timeout,
                }
            }

            None => Duration::from_secs(5),
        };

        match timer(timeout, orh.read_exact(buf)).await {
            // Data read successfully
            Ok(Ok(_)) => Ok(()),

            // Error during the read operation
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpReadError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "An underlying IO error occurred during socket read operation.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Ensure the socket connection is stable and there are no issues with the \
                         network or server.",
                    )));
            }

            // Read operation timed out
            Err(e) => {
                let report = Report::from(e)
                    .change_context(
                        NetworkError::TcpTimeoutElapsedError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "The read operation exceeded the specified timeout duration.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Check for network latency issues and consider increasing the timeout \
                         duration if the server response is expected to be slow.",
                    ));

                return Err(report);
            }
        }
    }

    async fn read_to_end(&mut self, buf: &mut Vec<u8>, timeout: Option<&Duration>) -> Result<()> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Tokio>::Read: Reading data from {}",
            &self.peer_addr,
        );

        // Await the read stream lock
        let mut orh_mg = self.read_stream.lock().await;
        let orh = &mut *orh_mg;

        // Validate the timeout duration
        let timeout = match timeout {
            Some(timeout) => {
                match timeout.is_zero() {
                    true => Duration::from_secs(5),
                    false => *timeout,
                }
            }

            None => Duration::from_secs(5),
        };

        match timer(timeout, orh.read_to_end(buf)).await {
            // Data read successfully
            Ok(Ok(_)) => Ok(()),

            // Error during the read operation
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpReadError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "An underlying IO error occurred during socket read operation.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Ensure the socket connection is stable and there are no issues with the \
                         network or server.",
                    )));
            }

            // Read operation timed out
            Err(e) => {
                let report = Report::from(e)
                    .change_context(
                        NetworkError::TcpTimeoutElapsedError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "The read operation exceeded the specified timeout duration.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Check for network latency issues and consider increasing the timeout \
                         duration if the server response is expected to be slow.",
                    ));

                return Err(report);
            }
        }
    }

    async fn write(&mut self, data: &[u8], timeout: Option<&Duration>) -> Result<()> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Tokio>::Write: Writing data to {}",
            &self.peer_addr,

        );

        // Await the write stream lock
        let mut owh_mg = self.write_stream.lock().await;
        let owh = &mut *owh_mg;

        // Validate the timeout duration
        let timeout = match timeout {
            Some(timeout) => {
                match timeout.is_zero() {
                    true => Duration::from_secs(5),
                    false => *timeout,
                }
            }

            None => Duration::from_secs(5),
        };

        match timer(timeout, owh.write_all(data)).await {
            // Data written successfully
            Ok(Ok(_)) => Ok(()),

            // Error during the write operation
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpWriteError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "An underlying IO error occurred during socket write operation.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Check if the server is accepting data correctly and there are no issues \
                         with network stability.",
                    )));
            }

            // Write operation timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpTimeoutElapsedError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "The write operation exceeded the specified timeout duration.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Consider increasing the timeout duration or check for network congestion.",
                    )));
            }
        }
    }
}
