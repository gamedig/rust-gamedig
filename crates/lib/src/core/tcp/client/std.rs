use {
    crate::core::error::{
        Report,
        diagnostic::{CRATE_INFO, FailureReason, SYSTEM_INFO},
    },
    std::{
        io::{Read, Write},
        net::{SocketAddr, TcpStream},
        time::Duration,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum StdTcpClientError {
    #[error("[GameDig]::[TCP::STD::CONNECTION]: Failed to establish connection")]
    Connection,

    #[error("[GameDig]::[TCP::STD::READ_TIMEOUT]: Failed to set read timeout on stream")]
    SetReadTimeout,

    #[error("[GameDig]::[TCP::STD::WRITE_TIMEOUT]: Failed to set write timeout on stream")]
    SetWriteTimeout,

    #[error("[GameDig]::[TCP::STD::READ_EXACT]: Failed to read exact bytes from stream")]
    ReadExact,

    #[error("[GameDig]::[TCP::STD::READ_TO_END]: Failed to read all bytes from stream")]
    ReadToEnd,
    
    #[error("[GameDig]::[TCP::STD::WRITE]: Failed to write to stream")]
    Write,
}

#[derive(Debug)]
pub(crate) struct StdTcpClient {
    peer_addr: SocketAddr,
    stream: TcpStream,

    read_timeout_set: bool,
    write_timeout_set: bool,
}

#[maybe_async::sync_impl]
impl super::AbstractTcp for StdTcpClient {
    type Error = Report<StdTcpClientError>;

    fn new(addr: SocketAddr, timeout: Duration) -> Result<Self, Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::TCP::CLIENT::STD::<NEW>: {:?}", |f| {
            f.debug_struct("Args")
                .field("addr", &addr)
                .field("timeout", &timeout)
                .finish()
        });

        match TcpStream::connect_timeout(&addr, timeout) {
            Ok(stream) => {
                Ok(Self {
                    peer_addr: addr,
                    stream,
                    read_timeout_set: false,
                    write_timeout_set: false,
                })
            }

            Err(e) => {
                Err(Report::from(e)
                    .change_context(StdTcpClientError::Connection)
                    .attach(FailureReason::new(
                        "Failed to establish a TCP connection due to an underlying OS I/O error.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO))
            }
        }
    }

    fn read_exact(&mut self, buf: &mut [u8], timeout: Duration) -> Result<(), Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::TCP::CLIENT::STD::<READ_EXACT>: {:?}", |f| {
            f.debug_struct("Args")
                .field("buf", format_args!("len({})", buf.len()))
                .field("timeout", &timeout)
                .finish()
        });

        if !self.read_timeout_set {
            match self.stream.set_read_timeout(Some(timeout)) {
                Ok(_) => {
                    self.read_timeout_set = true;
                }

                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(StdTcpClientError::SetReadTimeout)
                        .attach(FailureReason::new(format!(
                            "An underlying OS I/O error occurred while setting the read timeout \
                             to {timeout:?} for the TCP stream."
                        )))
                        .attach(SYSTEM_INFO)
                        .attach(CRATE_INFO));
                }
            }
        }

        match self.stream.read_exact(buf) {
            // Data read successfully
            Ok(_) => Ok(()),

            // Error during the read operation
            Err(e) => {
                Err(Report::from(e)
                    .change_context(StdTcpClientError::ReadExact)
                    .attach(FailureReason::new(
                        "An underlying I/O error occurred during the socket read operation.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO))
            }
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>, timeout: Duration) -> Result<(), Self::Error> {
        dev_trace_fmt!(
            "GAMEDIG::CORE::TCP::CLIENT::STD::<READ_TO_END>: {:?}",
            |f| {
                f.debug_struct("Args")
                    .field("buf", format_args!("cap({})", buf.capacity()))
                    .field("timeout", &timeout)
                    .finish()
            }
        );

        if !self.read_timeout_set {
            match self.stream.set_read_timeout(Some(timeout)) {
                Ok(_) => {
                    self.read_timeout_set = true;
                }

                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(StdTcpClientError::SetReadTimeout)
                        .attach(FailureReason::new(format!(
                            "An underlying OS I/O error occurred while setting the read timeout \
                             to {timeout:?} for the TCP stream."
                        )))
                        .attach(SYSTEM_INFO)
                        .attach(CRATE_INFO));
                }
            }
        }

        match self.stream.read_to_end(buf) {
            // Data read successfully
            Ok(_) => Ok(()),

            // Error during the read operation
            Err(e) => {
                Err(Report::from(e)
                    .change_context(StdTcpClientError::ReadToEnd)
                    .attach(FailureReason::new(
                        "An underlying I/O error occurred during the socket read operation.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO))
            }
        }
    }

    fn write(&mut self, data: &[u8], timeout: Duration) -> Result<(), Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::TCP::CLIENT::STD::<WRITE>: {:?}", |f| {
            f.debug_struct("Args")
                .field("data", format_args!("len({})", data.len()))
                .field("timeout", &timeout)
                .finish()
        });

        if !self.write_timeout_set {
            match self.stream.set_write_timeout(Some(timeout)) {
                Ok(_) => {
                    self.write_timeout_set = true;
                }

                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(StdTcpClientError::SetWriteTimeout)
                        .attach(FailureReason::new(format!(
                            "An underlying OS I/O error occurred while setting the write timeout \
                             to {timeout:?} for the TCP stream."
                        )))
                        .attach(SYSTEM_INFO)
                        .attach(CRATE_INFO));
                }
            }
        }

        match self.stream.write_all(data) {
            Ok(_) => Ok(()),

            Err(e) => {
                Err(Report::from(e)
                    .change_context(StdTcpClientError::Write)
                    .attach(FailureReason::new(
                        "An underlying RT or OS I/O error occurred during TCP write operation.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO))
            }
        }
    }
}
