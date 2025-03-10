use {
    crate::error::{
        NetworkError,
        Report,
        Result,
        diagnostic::{FailureReason, Recommendation},
    },
    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket},
        time::Duration,
    },
};

pub(crate) struct StdUdpClient {
    peer_addr: SocketAddr,
    socket: UdpSocket,

    send_timeout_set: bool,
    recv_timeout_set: bool,
}

#[maybe_async::sync_impl]
impl super::AbstractUdp for StdUdpClient {
    fn new(addr: &SocketAddr) -> Result<Self> {
        match UdpSocket::bind(match addr {
            SocketAddr::V4(_) => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
            SocketAddr::V6(_) => SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)),
        }) {
            Ok(socket) => {
                match socket.connect(addr) {
                    Ok(_) => {
                        Ok(Self {
                            peer_addr: *addr,
                            socket,
                            send_timeout_set: false,
                            recv_timeout_set: false,
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

    fn send(&mut self, data: &[u8], timeout: Option<&Duration>) -> Result<()> {
        if !self.send_timeout_set {
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

            match self.socket.set_write_timeout(Some(timeout)) {
                Ok(_) => {
                    self.send_timeout_set = true;
                }

                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(
                            NetworkError::UdpSetTimeoutError {
                                peer_addr: self.peer_addr,
                            }
                            .into(),
                        )
                        .attach_printable(FailureReason::new(
                            "Failed to set the read timeout for the TCP stream",
                        )));
                }
            }
        }

        match self.socket.send(data) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::UdpSendError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "Failed to write data to the UDP socket.",
                    )));
            }
        }
    }

    fn recv(&mut self, size: Option<usize>, timeout: Option<&Duration>) -> Result<Vec<u8>> {
        if !self.recv_timeout_set {
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

            match self.socket.set_read_timeout(Some(timeout)) {
                Ok(_) => {
                    self.recv_timeout_set = true;
                }

                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(
                            NetworkError::UdpSetTimeoutError {
                                peer_addr: self.peer_addr,
                            }
                            .into(),
                        )
                        .attach_printable(FailureReason::new(
                            "Failed to set the recv timeout for the UDP socket.",
                        )));
                }
            }
        }

        let valid_size = size.unwrap_or(Self::DEFAULT_BUF_CAPACITY as usize);
        let mut vec = Vec::with_capacity(valid_size);

        match self.socket.recv(&mut vec) {
            Ok(len) => {
                #[cfg(feature = "_DEV_LOG")]
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

                Ok(vec)
            }

            Err(e) => {
                return Err(Report::from(e)
                    .change_context(
                        NetworkError::UdpRecvError {
                            peer_addr: self.peer_addr,
                        }
                        .into(),
                    )
                    .attach_printable(FailureReason::new(
                        "Failed to read data from the UDP socket.",
                    )));
            }
        }
    }
}
