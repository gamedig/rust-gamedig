use {
    crate::error::{
        NetworkError,
        Report,
        Result,
        diagnostic::{CrateInfo, FailureReason, Recommendation, SystemInfo},
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
    fn new(addr: SocketAddr, timeout: Duration) -> Result<Self> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<NEW>: [addr: {addr:?}, timeout: \
             {timeout:?}]",
        );

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<NEW>: Attempting to connect to \
             {addr:?} with a timeout of {timeout:?}",
        );

        match TcpStream::connect_timeout(&addr, timeout) {
            Ok(stream) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<NEW>: Successfully connected \
                     to {addr:?}",
                );

                Ok(Self {
                    peer_addr: addr,
                    stream,
                    read_timeout_set: false,
                    write_timeout_set: false,
                })
            }

            Err(e) => {
                Err(Report::from(e)
                    .change_context(NetworkError::TcpConnectionError { peer_addr: addr }.into())
                    .attach(FailureReason::new(
                        "Failed to establish a TCP connection due to an underlying OS I/O error.",
                    ))
                    .attach(Recommendation::new(format!(
                        "Verify the server address ({addr}) is reachable, ensure the server is \
                         running, and that no firewall or network restrictions are blocking the \
                         connection. Also consider increasing the timeout duration ({timeout:?}) \
                         for distant or busy servers."
                    )))
                    .attach(SystemInfo::new())
                    .attach(CrateInfo::new()))
            }
        }
    }

    fn read_exact(&mut self, buf: &mut [u8], timeout: Duration) -> Result<()> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_EXACT>: [buf: len({:?}), \
             timeout: {timeout:?}]",
            buf.len()
        );

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_EXACT>: OS level socket read \
             timeout status for {} : {}",
            &self.peer_addr,
            if self.read_timeout_set {
                "already set"
            } else {
                "not set"
            }
        );

        // Set the read timeout if not already set
        if !self.read_timeout_set {
            dev_debug!(
                "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_EXACT>: Attempting to set \
                 read timeout for {} to {timeout:?}",
                &self.peer_addr
            );

            match self.stream.set_read_timeout(Some(timeout)) {
                Ok(_) => {
                    dev_debug!(
                        "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_EXACT>: Successfully \
                         set read timeout for {} to {timeout:?}",
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
                        .attach(FailureReason::new(format!(
                            "An underlying OS I/O error occurred while setting the read timeout \
                             to {timeout:?} for the TCP stream."
                        )))
                        .attach(SystemInfo::new())
                        .attach(CrateInfo::new()));
                }
            }
        }

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_EXACT>: Attempting to read from \
             {} with a timeout of {timeout:?}",
            &self.peer_addr,
        );

        match self.stream.read_exact(buf) {
            // Data read successfully
            Ok(_) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_EXACT>: Successfully \
                     read data from {:?}",
                    &self.peer_addr,
                );

                Ok(())
            }

            // Error during the read operation
            Err(e) => {
                Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpReadError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach(FailureReason::new(
                        "An underlying I/O error occurred during the socket read operation.",
                    ))
                    .attach(Recommendation::new(
                        "Ensure the socket connection is stable and there are no issues with the \
                         network or server.",
                    ))
                    .attach(SystemInfo::new())
                    .attach(CrateInfo::new()))
            }
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>, timeout: Duration) -> Result<()> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_TO_END>: [buf: cap({:?}), \
             timeout: {timeout:?}]",
            buf.capacity()
        );

        // Set the read timeout if not already set
        if !self.read_timeout_set {
            dev_debug!(
                "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_TO_END>: Attempting to set \
                 read timeout for {} to {timeout:?}",
                &self.peer_addr
            );

            match self.stream.set_read_timeout(Some(timeout)) {
                Ok(_) => {
                    dev_debug!(
                        "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_EXACT>: Successfully \
                         set read timeout for {} to {timeout:?}",
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
                        .attach(FailureReason::new(format!(
                            "An underlying OS I/O error occurred while setting the read timeout \
                             to {timeout:?} for the TCP stream."
                        )))
                        .attach(SystemInfo::new())
                        .attach(CrateInfo::new()));
                }
            }
        }

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_TO_END>: Attempting to read from \
             {} with a timeout of {timeout:?}",
            &self.peer_addr,
        );

        match self.stream.read_to_end(buf) {
            // Data read successfully
            Ok(_num_bytes) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<READ_TO_END>: Successfully \
                     read {} bytes from {}",
                    _num_bytes,
                    &self.peer_addr,
                );

                Ok(())
            }

            // Error during the read operation
            Err(e) => {
                Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpReadError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach(FailureReason::new(
                        "An underlying I/O error occurred during the socket read operation.",
                    ))
                    .attach(Recommendation::new(
                        "Ensure the socket connection is stable and there are no issues with the \
                         network or server.",
                    ))
                    .attach(SystemInfo::new())
                    .attach(CrateInfo::new()))
            }
        }
    }

    fn write(&mut self, data: &[u8], timeout: Duration) -> Result<()> {
        dev_trace!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<WRITE>: [data: len({:?}), timeout: \
             {timeout:?}]",
            data.len()
        );

        if !self.write_timeout_set {
            dev_debug!(
                "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<WRITE>: Attempting to set write \
                 timeout for {} to {timeout:?}",
                &self.peer_addr
            );

            match self.stream.set_write_timeout(Some(timeout)) {
                Ok(_) => {
                    dev_debug!(
                        "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<WRITE>: Successfully set \
                         write timeout for {} to {timeout:?}",
                        &self.peer_addr
                    );

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
                        .attach(FailureReason::new(format!(
                            "An underlying OS I/O error occurred while setting the write timeout \
                             to {timeout:?} for the TCP stream."
                        )))
                        .attach(SystemInfo::new())
                        .attach(CrateInfo::new()));
                }
            }
        }

        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<WRITE>: Attempting to write to {} \
             with a timeout of {timeout:?}",
            &self.peer_addr,
        );

        match self.stream.write_all(data) {
            Ok(_) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<WRITE>: Successfully wrote \
                     data to {}",
                    &self.peer_addr,
                );

                Ok(())
            }

            Err(e) => {
                Err(Report::from(e)
                    .change_context(
                        NetworkError::TcpWriteError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach(FailureReason::new(
                        "An underlying RT or OS I/O error occurred during TCP write operation.",
                    ))
                    .attach(Recommendation::new(
                        "Check if the server is accepting data correctly and ensure network \
                         stability.",
                    ))
                    .attach(SystemInfo::new())
                    .attach(CrateInfo::new()))
            }
        }
    }
}

impl Drop for StdTcpClient {
    fn drop(&mut self) {
        dev_debug!(
            "GAMEDIG::CORE::NET::TCP::SEALED::CLIENT::STD::<DROP>: Dropping connection to {:?}",
            &self.peer_addr,
        );
    }
}
