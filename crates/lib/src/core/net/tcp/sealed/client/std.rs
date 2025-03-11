use {
    crate::error::{
        NetworkError,
        Report,
        Result,
        diagnostic::{FailureReason, Recommendation},
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
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Std>::New: Creating new TCP client for {addr}"
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
            "TCP::<Std>::New: Attempting to connect to {addr:?} with a timeout of {valid_timeout:?}",
        );

        match TcpStream::connect_timeout(addr, valid_timeout) {
            Ok(stream) => {
                #[cfg(feature = "_DEV_LOG")]
                log::debug!(
                    target: crate::log::EventTarget::GAMEDIG_DEV,
                    "TCP::<Std>::New: Successfully connected to {addr:?}",
                );

                Ok(Self {
                    peer_addr: *addr,
                    stream,
                    read_timeout_set: false,
                    write_timeout_set: false,
                })
            }
            Err(e) => {
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
        }
    }

    fn read_exact(&mut self, buf: &mut [u8], timeout: Option<&Duration>) -> Result<()> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Std>::Read: Reading data from {}", &self.peer_addr);

        // Set the read timeout if not already set
        if !self.read_timeout_set {
            // Validate the timeout duration
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
                "TCP::<Std>::Read: Setting read timeout for {} to {valid_timeout:?}",
                &self.peer_addr


            );

            match self.stream.set_read_timeout(Some(valid_timeout)) {
                Ok(_) => {
                    #[cfg(feature = "_DEV_LOG")]
                    log::debug!(
                        target: crate::log::EventTarget::GAMEDIG_DEV,
                        "TCP::<Std>::Read: Successfully set read timeout for {}",
                        &self.peer_addr
                    );

                    self.read_timeout_set = true;
                }

                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(
                            NetworkError::TcpSetTimeoutError {
                                peer_addr: self.peer_addr,
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

        match self.stream.read_exact(buf) {
            // Data read successfully
            Ok(_) => Ok(()),

            // Error during the read operation
            Err(e) => {
                Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpReadError {
                            peer_addr: self.peer_addr,
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

    fn read_to_end(&mut self, buf: &mut Vec<u8>, timeout: Option<&Duration>) -> Result<()> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Std>::Read: Reading data from {}", &self.peer_addr);

        // Set the read timeout if not already set
        if !self.read_timeout_set {
            // Validate the timeout duration
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
                "TCP::<Std>::Read: Setting read timeout for {} to {valid_timeout:?}",
                &self.peer_addr


            );

            match self.stream.set_read_timeout(Some(valid_timeout)) {
                Ok(_) => {
                    #[cfg(feature = "_DEV_LOG")]
                    log::debug!(
                        target: crate::log::EventTarget::GAMEDIG_DEV,
                        "TCP::<Std>::Read: Successfully set read timeout for {}",
                        &self.peer_addr
                    );

                    self.read_timeout_set = true;
                }

                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(
                            NetworkError::TcpSetTimeoutError {
                                peer_addr: self.peer_addr,
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

        match self.stream.read_to_end(buf) {
            // Data read successfully
            Ok(_) => Ok(()),

            // Error during the read operation
            Err(e) => {
                Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpReadError {
                            peer_addr: self.peer_addr,
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
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "TCP::<Std>::Write: Writing data to {}", &self.peer_addr);

        if !self.write_timeout_set {
            // Validate the timeout duration
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
                "TCP::<Std>::Write: Setting write timeout for {} to {valid_timeout:?}",
                &self.peer_addr
            );

            match self.stream.set_write_timeout(Some(valid_timeout)) {
                Ok(_) => {
                    self.write_timeout_set = true;
                }
                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(
                            NetworkError::TcpSetTimeoutError {
                                peer_addr: self.peer_addr,
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
                        NetworkError::TcpWriteError {
                            peer_addr: self.peer_addr,
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
