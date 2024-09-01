use std::{net::SocketAddr, time::Duration};

use crate::{
    error::{
        diagnostic::{metadata::NetworkProtocol, FailureReason, Recommendation},
        NetworkError,
        Report,
        Result,
    },
    settings::Timeout,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
    time::timeout as timer,
};

#[derive(Debug)]
pub(crate) struct TokioTcpClient {
    addr: SocketAddr,
    read_timeout: Duration,
    write_timeout: Duration,
    read_stream: Mutex<OwnedReadHalf>,
    write_stream: Mutex<OwnedWriteHalf>,
}

#[maybe_async::async_impl]
impl super::Tcp for TokioTcpClient {
    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Tokio>::New: Creating new TCP client for {addr} with timeout: {timeout:?}"
        );

        let (orh, owh) = match timer(timeout.connect, TcpStream::connect(addr)).await {
            // Connection succeeded, split the stream into read and write halves.
            Ok(Ok(stream)) => stream.into_split(),

            // Connection failed due to an IO error
            Ok(Err(e)) => {
                let report = Report::from(e)
                    .change_context(
                        NetworkError::ConnectionError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: *addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "Failed to establish a TCP connection due to an underlying I/O error",
                    ))
                    .attach_printable(Recommendation::new(format!(
                        "Verify the server address ({addr:?}) is reachable.",
                    )));

                return Err(report);
            }

            // Connection failed due to a timeout
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::TimeoutElapsedError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: *addr,
                        }
                        .into(),
                    )
                    .attach(FailureReason::new(
                        "The connection attempt elapsed the timeout that was set.",
                    ))
                    .attach(Recommendation::new(
                        "Check if the server is currently experiencing high traffic or \
                         performance issues. Increase the timeout setting to accommodate delays, \
                         especially during peak usage times or if the server is geographically \
                         distant.",
                    )));
            }
        };

        Ok(TokioTcpClient {
            addr: *addr,
            read_timeout: timeout.read,
            write_timeout: timeout.write,
            read_stream: Mutex::new(orh),
            write_stream: Mutex::new(owh),
        })
    }

    async fn read(&mut self, size: Option<usize>) -> Result<(Vec<u8>, usize)> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Tokio>::Read: Reading data from {} with size: {size:?}",
            self.addr
        );

        // Acquire a lock on the read stream
        let mut orh_mg = self.read_stream.lock().await;
        let orh = &mut *orh_mg;

        let valid_size = match size {
            Some(size) => size,
            None => Self::DEFAULT_BUF_CAPACITY as usize,
        };

        let mut vec = Vec::with_capacity(valid_size);

        match timer(self.read_timeout, orh.read_to_end(&mut vec)).await {
            // Data read successfully
            Ok(Ok(len)) => {
                #[cfg(feature = "attribute_log")]
                if valid_size < len {
                    log::debug!(
                        "TCP::<Tokio>::Read: Realloc was required, Requested: {valid_size}, \
                         Received: {len} from {}",
                        self.addr
                    );
                }

                // Shrink the vector to fit the data if there's excess capacity
                if vec.capacity() > (len + Self::BUF_SHRINK_MARGIN as usize) {
                    vec.shrink_to_fit();
                }

                Ok((vec, len))
            }

            // Error during the read operation
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::ReadError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: self.addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "An underlying IO error occurred during socket read operation.",
                    )));
            }

            // Read operation timed out
            Err(e) => {
                let report = Report::from(e)
                    .change_context(
                        NetworkError::TimeoutElapsedError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: self.addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "The read operation had elapsed the timeout.",
                    ));

                // Needs to be chained as attaching moves the report
                let report = if self.read_timeout < Timeout::DEFAULT_DURATION {
                    report.attach(Recommendation::new(
                        "Possibly increase the read timeout duration as the current duration set \
                         is less than the default.",
                    ))
                } else {
                    report
                };

                return Err(report);
            }
        }
    }

    async fn write(&mut self, data: &[u8]) -> Result<()> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Tokio>::Write: Writing data to {} with size: {}",
            self.addr,
            data.len()
        );

        // Acquire a lock on the write stream
        let mut owh_mg = self.write_stream.lock().await;
        let owh = &mut *owh_mg;

        match timer(self.write_timeout, owh.write_all(data)).await {
            // Data written successfully
            Ok(Ok(_)) => Ok(()),

            // Error during the write operation
            Ok(Err(e)) => {
                Err(Report::from(e)
                    .change_context(
                        NetworkError::WriteError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: self.addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "An underlying IO error occurred during socket write operation.",
                    )))
            }

            // Write operation timed out
            Err(e) => {
                let report = Report::from(e)
                    .change_context(
                        NetworkError::TimeoutElapsedError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: self.addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "The write operation had elapsed the timeout.",
                    ));

                // Needs to be chained as attaching moves the report
                let report = if self.write_timeout < Timeout::DEFAULT_DURATION {
                    report.attach_printable(Recommendation::new(
                        "Possibly increase the write timeout duration as the current duration set \
                         is less than the default.",
                    ))
                } else {
                    report
                };

                Err(report)
            }
        }
    }
}
