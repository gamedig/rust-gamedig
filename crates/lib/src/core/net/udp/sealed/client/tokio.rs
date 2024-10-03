use {
    crate::error::{
        diagnostic::{FailureReason, Recommendation},
        NetworkError,
        Report,
        Result,
    },

    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
        time::Duration,
    },

    tokio::{net::UdpSocket, time::timeout as timer},
};

pub(crate) struct TokioUdpClient {
    peer_addr: SocketAddr,
    socket: UdpSocket,
}

#[maybe_async::async_impl]
impl super::AbstractUdp for TokioUdpClient {
    async fn new(addr: &SocketAddr) -> Result<Self> {
        match UdpSocket::bind(match addr {
            SocketAddr::V4(_) => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
            SocketAddr::V6(_) => SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)),
        })
        .await
        {
            Ok(socket) => {
                match socket.connect(addr).await {
                    Ok(_) => {
                        Ok(Self {
                            peer_addr: *addr,
                            socket,
                        })
                    }

                    // Connection error
                    Err(e) => {
                        return Err(Report::from(e)
                            .change_context(
                                NetworkError::UdpConnectionError { peer_addr: *addr }.into(),
                            )
                            .attach_printable(FailureReason::new(
                                "Failed to establish a UDP connection due to an underlying I/O \
                                 error.",
                            ))
                            .attach_printable(Recommendation::new(
                                "Ensure the server is running and that no firewall or network \
                                 restrictions are blocking the connection.",
                            )));
                    }
                }
            }

            // Bind error
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(NetworkError::UdpBindError {}.into())
                    .attach_printable(FailureReason::new("Failed to bind to the UDP socket.")));
            }
        }
    }

    async fn send(&mut self, data: &[u8], timeout: Option<&Duration>) -> Result<()> {
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

        match timer(timeout, self.socket.send(data)).await {
            Ok(Ok(_)) => Ok(()),

            // Error during the send operation
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::UdpSendError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "Failed to send data over the UDP socket.",
                    )));
            }

            // Send operation timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::UdpTimeoutElapsedError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "The send operation exceeded the specified timeout duration.",
                    ))
                    .attach_printable(Recommendation::new(
                        "Check the server's status for high traffic or downtime, and consider \
                         increasing the timeout duration for distant or busy servers.",
                    )));
            }
        }
    }

    async fn recv(
        &mut self,
        size: Option<usize>,
        timeout: Option<&Duration>,
    ) -> Result<(Vec<u8>, usize)> {
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

        let valid_size = size.unwrap_or(Self::DEFAULT_BUF_CAPACITY as usize);
        let mut vec = Vec::with_capacity(valid_size);

        match timer(timeout, self.socket.recv(&mut vec)).await {
            Ok(Ok(len)) => {
                #[cfg(feature = "attribute_log")]
                if valid_size < len {
                    log::debug!(
                        "UDP::<Std>::Read: More data than expected. Realloc was required. \
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
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::UdpRecvError {
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
                    )));
            }

            // Read operation timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::UdpTimeoutElapsedError {
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
                    )));
            }
        }
    }
}
