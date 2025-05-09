use {
    crate::error::{
        NetworkError,
        Report,
        Result,
        diagnostic::{CrateInfo, FailureReason, Recommendation, SystemInfo},
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
    async fn new(addr: SocketAddr, timeout: Duration) -> Result<Self> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<NEW>: [addr: {addr:?}, timeout: \
             {timeout:?}]",
        );

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<NEW>: Attempting to connect to \
             {addr:?} with a timeout of {timeout:?}",
        );

        let (orh, owh) = match timer(timeout, TcpStream::connect(addr)).await {
            // Connection established successfully
            Ok(Ok(stream)) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<NEW>: Successfully \
                     connected to {addr:?}",
                );

                stream.into_split()
            }

            // Error during the connection attempt
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(NetworkError::TcpConnectionError { peer_addr: addr }.into())
                    .attach_printable(FailureReason::new(
                        "Failed to establish a TCP connection due to an underlying RT or OS I/O \
                         error.",
                    ))
                    .attach_printable(Recommendation::new(format!(
                        "Verify the server address ({addr:?}) is reachable, ensure the server is \
                         running, and that no firewall or network restrictions are blocking the \
                         connection."
                    )))
                    .attach_printable(SystemInfo::new())
                    .attach_printable(CrateInfo::new()));
            }

            // Connection attempt timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(NetworkError::TcpTimeoutElapsedError { peer_addr: addr }.into())
                    .attach_printable(FailureReason::new(format!(
                        "The connection attempt exceeded the specified timeout duration of \
                         {timeout:?}."
                    )))
                    .attach_printable(Recommendation::new(
                        "Check the server's status for high traffic or downtime, and consider \
                         increasing the timeout duration for distant or busy servers.",
                    )));
            }
        };

        Ok(TokioTcpClient {
            peer_addr: addr,
            read_stream: Mutex::new(orh),
            write_stream: Mutex::new(owh),
        })
    }

    async fn read_exact(&mut self, buf: &mut [u8], timeout: Duration) -> Result<()> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<READ_EXACT>: [buf: len({:?}), \
             timeout: {timeout:?}]",
            buf.len()
        );

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<READ_EXACT>: Attempting to acquire \
             read lock for TCP stream to read from {}",
            &self.peer_addr,
        );

        // Await the read stream lock
        let mut orh_mg = self.read_stream.lock().await;
        let orh = &mut *orh_mg;

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<READ_EXACT>: Acquired read lock for \
             TCP stream and attempting to read from {} with a timeout of {timeout:?}",
            &self.peer_addr,
        );

        match timer(timeout, orh.read_exact(buf)).await {
            // Data read successfully
            Ok(Ok(_)) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<READ_EXACT>: Successfully \
                     read data from {}",
                    &self.peer_addr,
                );

                Ok(())
            }

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
                        "An underlying RT or OS I/O error occurred during socket read operation.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Ensure the socket connection is stable and there are no issues with the \
                         network or server.",
                    ))
                    .attach_printable(SystemInfo::new())
                    .attach_printable(CrateInfo::new()));
            }

            // Read operation timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpTimeoutElapsedError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(format!(
                        "The read operation exceeded the specified timeout duration of \
                         {timeout:?}."
                    )))
                    .attach_printable(Recommendation::new(
                        "Check for network latency issues and consider increasing the timeout \
                         duration if the server response is expected to be slow.",
                    )));
            }
        }
    }

    async fn read_to_end(&mut self, buf: &mut Vec<u8>, timeout: Duration) -> Result<()> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<READ_TO_END>: [buf: cap({:?}), \
             timeout: {timeout:?}]",
            buf.capacity()
        );

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<READ_TO_END>: Attempting to acquire \
             read lock for TCP stream to read from {}",
            &self.peer_addr,
        );

        // Await the read stream lock
        let mut orh_mg = self.read_stream.lock().await;
        let orh = &mut *orh_mg;

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<READ_TO_END>: Acquired read lock \
             for TCP stream and attempting to read from {} with a timeout of {timeout:?}",
            &self.peer_addr,
        );

        match timer(timeout, orh.read_to_end(buf)).await {
            // Data read successfully
            Ok(Ok(_num_bytes)) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<READ_TO_END>: Successfully \
                     read {} bytes from {}",
                    _num_bytes,
                    &self.peer_addr,
                );

                Ok(())
            }

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
                        "An underlying RT or OS I/O error occurred during socket read operation.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Ensure the socket connection is stable and there are no issues with the \
                         network or server.",
                    ))
                    .attach_printable(SystemInfo::new())
                    .attach_printable(CrateInfo::new()));
            }

            // Read operation timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpTimeoutElapsedError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(format!(
                        "The read operation exceeded the specified timeout duration of \
                         {timeout:?}."
                    )))
                    .attach_printable(Recommendation::new(
                        "Check for network latency issues and consider increasing the timeout \
                         duration if the server response is expected to be slow.",
                    )));
            }
        }
    }

    async fn write(&mut self, data: &[u8], timeout: Duration) -> Result<()> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<WRITE>: [data: len({:?}), timeout: \
             {timeout:?}]",
            data.len()
        );

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<WRITE>: Attempting to acquire write \
             lock for TCP stream to write to {}",
            &self.peer_addr,
        );

        // Await the write stream lock
        let mut owh_mg = self.write_stream.lock().await;
        let owh = &mut *owh_mg;

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<WRITE>: Acquired write lock for TCP \
             stream and attempting to write to {} with a timeout of {timeout:?}",
            &self.peer_addr,
        );

        match timer(timeout, owh.write_all(data)).await {
            // Data written successfully
            Ok(Ok(_)) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::TOKIO::<WRITE>: Successfully wrote \
                     data to {}",
                    &self.peer_addr,
                );

                Ok(())
            }

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
                        "An underlying RT or OS I/O error occurred during socket write operation.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Check if the server is accepting data correctly and there are no issues \
                         with network stability.",
                    ))
                    .attach_printable(SystemInfo::new())
                    .attach_printable(CrateInfo::new()));
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
                    .attach_printable(FailureReason::new(format!(
                        "The write operation exceeded the specified timeout duration of \
                         {timeout:?}."
                    )))
                    .attach_printable(Recommendation::new(
                        "Consider increasing the timeout duration or check for network congestion.",
                    )));
            }
        }
    }
}
