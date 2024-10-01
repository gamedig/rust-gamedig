use {
    crate::error::{
        diagnostic::{metadata::NetworkProtocol, FailureReason, Recommendation},
        NetworkError,
        Report,
        Result,
    },

    std::{
        io::{Read, Write},
        net::{SocketAddr, TcpStream},
        time::Duration,
    },
};

#[derive(Debug)]
pub(crate) struct StdTcpClient {
    peer_addr: SocketAddr,
    stream: TcpStream,

    read_timeout_set: bool,
    write_timeout_set: bool,
}

#[maybe_async::sync_impl]
impl super::AbstractTcp for StdTcpClient {
    fn new(addr: &SocketAddr, timeout: Option<&Duration>) -> Result<Self> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Std>::New: Creating new TCP client for {addr} with timeout: {timeout:?}"
        );

        let timeout = match timeout {
            Some(timeout) => {
                match timeout.is_zero() {
                    true => Duration::from_secs(5),
                    false => *timeout,
                }
            }

            None => Duration::from_secs(5),
        };

        match TcpStream::connect_timeout(addr, timeout) {
            Ok(stream) => {
                Ok(Self {
                    peer_addr: *addr,
                    stream,
                    read_timeout_set: false,
                    write_timeout_set: false,
                })
            }
            Err(e) => {
                Err(Report::from(e)
                    .change_context(
                        NetworkError::ConnectionError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: *addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "Failed to establish a TCP connection due to an underlying I/O error.",
                    ))
                    .attach_printable(Recommendation::new(format!(
                        "Verify the server address ({addr:?}) is reachable, ensure the server is \
                         running, and that no firewall or network restrictions are blocking the \
                         connection."
                    ))))
            }
        }
    }

    fn read(
        &mut self,
        size: Option<usize>,
        timeout: Option<&Duration>,
    ) -> Result<(Vec<u8>, usize)> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Std>::Read: Reading data from {} with size: {:?}",
            &self.peer_addr,
            size
        );

        // Set the read timeout if not already set
        if !self.read_timeout_set {
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

            match self.stream.set_read_timeout(Some(timeout)) {
                Ok(_) => {
                    self.read_timeout_set = true;
                }
                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(
                            NetworkError::SetTimeoutError {
                                _protocol: NetworkProtocol::Tcp,
                                addr: &self.peer_addr,
                            }
                            .into(),
                        )
                        .attach_printable(FailureReason::new(
                            "Failed to set the read timeout for the TCP stream.",
                        ))
                        .attach_printable(Recommendation::new(
                            "Ensure the timeout value is valid and that the stream is not in a \
                             disconnected or invalid state.",
                        )));
                }
            }
        }

        // Validate size and set vector capacity
        let valid_size = size.unwrap_or(Self::DEFAULT_BUF_CAPACITY as usize);
        let mut vec = Vec::with_capacity(valid_size);

        match self.stream.read_to_end(&mut vec) {
            // Data read successfully
            Ok(len) => {
                #[cfg(feature = "attribute_log")]
                if valid_size < len {
                    log::debug!(
                        "TCP::<Std>::Read: More data than expected. Realloc was required. \
                         Expected: {valid_size} bytes, Read: {len} bytes",
                    );
                }

                // Shrink the vector to fit the data if there's excess capacity
                if vec.capacity() > (len + Self::BUF_SHRINK_MARGIN as usize) {
                    vec.shrink_to_fit();
                }

                Ok((vec, len))
            }

            // Error during the read operation
            Err(e) => {
                Err(Report::from(e)
                    .change_context(
                        NetworkError::ReadError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: &self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "An underlying I/O error occurred during the socket read operation.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Ensure the socket connection is stable and there are no issues with the \
                         network or server.",
                    )))
            }
        }
    }

    fn write(&mut self, data: &[u8], timeout: Option<&Duration>) -> Result<()> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Std>::Write: Writing data to {} with size: {}",
            &self.peer_addr,
            data.len()
        );

        if !self.write_timeout_set {
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

            match self.stream.set_write_timeout(Some(*timeout)) {
                Ok(_) => {
                    self.write_timeout_set = true;
                }
                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(
                            NetworkError::SetTimeoutError {
                                _protocol: NetworkProtocol::Tcp,
                                addr: &self.peer_addr,
                            }
                            .into(),
                        )
                        .attach_printable(FailureReason::new(
                            "Failed to set the write timeout for the TCP stream.",
                        ))
                        .attach_printable(Recommendation::new(
                            "Ensure the timeout value is valid and that the stream is not in a \
                             disconnected or invalid state.",
                        )));
                }
            }
        }

        match self.stream.write_all(data) {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(Report::from(e)
                    .change_context(
                        NetworkError::WriteError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: &self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "An underlying I/O error occurred during the socket write operation.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Check if the server is accepting data correctly and ensure network \
                         stability.",
                    )))
            }
        }
    }
}
