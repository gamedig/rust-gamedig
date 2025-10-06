use {
    crate::error::{
        NetworkError,
        Report,
        Result,
        diagnostic::{CrateInfo, FailureReason, Recommendation, SystemInfo},
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
    async fn new(addr: SocketAddr) -> Result<Self> {
        dev_trace!("GAMEDIG::CORE::NET::UDP::SEALED::CLIENT::TOKIO::<NEW>: [addr: {addr:?}]");

        match UdpSocket::bind(match addr {
            SocketAddr::V4(_) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::UDP::SEALED::CLIENT::TOKIO::<NEW>: Attempting to bind to \
                     IPV4 ephemeral port"
                );

                SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0))
            }

            SocketAddr::V6(_) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::UDP::SEALED::CLIENT::TOKIO::<NEW>: Attempting to bind to \
                     IPV6 ephemeral port"
                );

                SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0))
            }
        })
        .await
        {
            Ok(socket) => {
                dev_debug!(
                    "GAMEDIG::CORE::NET::UDP::SEALED::CLIENT::TOKIO::<NEW>: Successfully bound to \
                     the ephemeral port, Attempting to set the peer address"
                );

                match socket.connect(addr).await {
                    Ok(_) => {
                        dev_debug!(
                            "GAMEDIG::CORE::NET::UDP::SEALED::CLIENT::TOKIO::<NEW>: Successfully \
                             set peer address, socket is ready"
                        );

                        Ok(Self {
                            peer_addr: addr,
                            socket,
                        })
                    }

                    // Connection error
                    Err(e) => {
                        return Err(Report::from(e)
                            .change_context(
                                NetworkError::UdpConnectionError { peer_addr: addr }.into(),
                            )
                            .attach_printable(FailureReason::new(
                                "Failed to establish a UDP connection due to an underlying I/O \
                                 error.",
                            ))
                            .attach_printable(Recommendation::new(
                                "Ensure the server is running and that no firewall or network \
                                 restrictions are blocking the connection.",
                            ))
                            .attach_printable(SystemInfo::new())
                            .attach_printable(CrateInfo::new()));
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

    async fn send(&mut self, data: &[u8], timeout: Duration) -> Result<()> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "UDP::<Tokio>::Send: Sending data to {}",
            &self.peer_addr
        );

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

    async fn recv(&mut self, buf: &mut [u8], timeout: Duration) -> Result<()> {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            "UDP::<Tokio>::Recv: Receiving data from {}",
            &self.peer_addr
        );

        match timer(timeout, self.socket.recv(buf)).await {
            Ok(Ok(_)) => Ok(()),

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
