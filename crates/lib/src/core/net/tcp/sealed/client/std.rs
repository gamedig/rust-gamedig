use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use crate::{
    error::{
        diagnostic::{metadata::NetworkProtocol, FailureReason, Recommendation},
        NetworkError,
        Report,
        Result,
    },
    settings::Timeout,
};

#[derive(Debug)]
pub(crate) struct StdTcpClient {
    addr: SocketAddr,
    stream: TcpStream,
}

#[maybe_async::sync_impl]
impl super::Tcp for StdTcpClient {
    fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Std>::New: Creating new TCP client for {addr} with timeout: {timeout:?}"
        );

        match TcpStream::connect_timeout(addr, timeout.connect) {
            Ok(stream) => {
                match stream.set_read_timeout(Some(timeout.read)) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(Report::from(e)
                            .change_context(
                                NetworkError::SetTimeoutError {
                                    _protocol: NetworkProtocol::Tcp,
                                    addr: *addr,
                                }
                                .into(),
                            )
                            .attach_printable(FailureReason::new(
                                "Failed to set the read timeout for the TCP stream",
                            )));
                    }
                }

                match stream.set_write_timeout(Some(timeout.write)) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(Report::from(e)
                            .change_context(
                                NetworkError::SetTimeoutError {
                                    _protocol: NetworkProtocol::Tcp,
                                    addr: *addr,
                                }
                                .into(),
                            )
                            .attach_printable(FailureReason::new(
                                "Failed to set the write timeout for the TCP stream",
                            )));
                    }
                }

                Ok(Self {
                    addr: *addr,
                    stream,
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
                        "Failed to establish a TCP connection due to an underlying I/O error",
                    ))
                    .attach_printable(Recommendation::new(format!(
                        "Verify the server address ({addr:?}) is reachable.",
                    ))))
            }
        }
    }

    fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Std>::Read: Reading data from {} with size: {:?}",
            self.addr,
            size
        );

        let valid_size = match size {
            Some(size) => size,
            None => Self::DEFAULT_BUF_CAPACITY as usize,
        };

        let mut vec = Vec::with_capacity(valid_size);

        match self.stream.read_to_end(&mut vec) {
            Ok(len) => {
                #[cfg(feature = "attribute_log")]
                if valid_size < len {
                    log::debug!(
                        "TCP::<Std>::Read: More data than expected. Realloc was required. \
                         Expected: {valid_size} bytes, Read: {len} bytes",
                    );
                }

                if vec.capacity() > (len + Self::BUF_SHRINK_MARGIN as usize) {
                    vec.shrink_to_fit();
                }

                Ok(vec)
            }
            Err(e) => {
                Err(Report::from(e)
                    .change_context(
                        NetworkError::ReadError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: self.addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "An underlying IO error occurred during socket read operation.",
                    )))
            }
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            "TCP::<Std>::Write: Writing data to {} with size: {}",
            self.addr,
            data.len()
        );

        match self.stream.write_all(data) {
            Ok(_) => Ok(()),
            Err(e) => {
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
        }
    }
}
