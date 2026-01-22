use {
    crate::core::error::{
        Report,
        diagnostic::{CRATE_INFO, FailureReason, SYSTEM_INFO},
    },
    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket},
        time::Duration,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum StdUdpClientError {
    #[error("[GameDig]::[UDP::STD::BIND]: Failed to bind the UDP socket")]
    Bind,
    #[error("[GameDig]::[UDP::STD::CONNECT]: Failed to connect the UDP socket")]
    Connect,
    #[error("[GameDig]::[UDP::STD::SEND]: Failed to send data over UDP socket")]
    Send,
    #[error("[GameDig]::[UDP::STD::SET_SEND_TIMEOUT]: Failed to set send timeout for UDP socket")]
    SetSendTimeout,
    #[error("[GameDig]::[UDP::STD::RECV]: Failed to receive data from UDP socket")]
    Recv,
    #[error(
        "[GameDig]::[UDP::STD::SET_RECV_TIMEOUT]: Failed to set receive timeout for UDP socket"
    )]
    SetRecvTimeout,
}

pub(crate) struct StdUdpClient {
    socket: UdpSocket,

    send_timeout_set: bool,
    recv_timeout_set: bool,
}

#[maybe_async::sync_impl]
impl super::AbstractUdp for StdUdpClient {
    type Error = Report<StdUdpClientError>;

    fn new(addr: SocketAddr) -> Result<Self, Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::UDP::CLIENT::STD::<NEW>: {:?}", |f| {
            f.debug_struct("Args").field("addr", &addr).finish()
        });

        match UdpSocket::bind(match addr {
            SocketAddr::V4(_) => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
            SocketAddr::V6(_) => SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)),
        }) {
            Ok(socket) => {
                match socket.connect(addr) {
                    Ok(_) => {
                        Ok(Self {
                            socket,
                            send_timeout_set: false,
                            recv_timeout_set: false,
                        })
                    }

                    // Connection error
                    Err(e) => {
                        return Err(Report::from(e)
                            .change_context(StdUdpClientError::Connect)
                            .attach(FailureReason::new(
                                "Failed to establish a UDP connection due to an underlying I/O \
                                 error.",
                            ))
                            .attach(CRATE_INFO)
                            .attach(SYSTEM_INFO));
                    }
                }
            }

            // Bind error
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(StdUdpClientError::Bind)
                    .attach(FailureReason::new("Failed to bind to the UDP socket."))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }
        }
    }

    fn send(&mut self, data: &[u8], timeout: Duration) -> Result<(), Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::UDP::CLIENT::STD::<SEND>: {:?}", |f| {
            f.debug_struct("Args")
                .field("data", format_args!("len({})", data.len()))
                .field("timeout", &timeout)
                .finish()
        });

        if !self.send_timeout_set {
            match self.socket.set_write_timeout(Some(timeout)) {
                Ok(_) => {
                    self.send_timeout_set = true;
                }

                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(StdUdpClientError::SetSendTimeout)
                        .attach(FailureReason::new(
                            "Failed to set the send timeout for the UDP socket.",
                        ))
                        .attach(SYSTEM_INFO)
                        .attach(CRATE_INFO));
                }
            }
        }

        match self.socket.send(data) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(StdUdpClientError::Send)
                    .attach(FailureReason::new(
                        "Failed to write data to the UDP socket.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }
        }
    }

    fn recv(&mut self, buf: &mut [u8], timeout: Duration) -> Result<(), Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::UDP::CLIENT::STD::<RECV>: {:?}", |f| {
            f.debug_struct("Args")
                .field("buf", format_args!("len({})", buf.len()))
                .field("timeout", &timeout)
                .finish()
        });

        if !self.recv_timeout_set {
            match self.socket.set_read_timeout(Some(timeout)) {
                Ok(_) => {
                    self.recv_timeout_set = true;
                }

                Err(e) => {
                    return Err(Report::from(e)
                        .change_context(StdUdpClientError::SetRecvTimeout)
                        .attach(FailureReason::new(
                            "Failed to set the recv timeout for the UDP socket.",
                        ))
                        .attach(SYSTEM_INFO)
                        .attach(CRATE_INFO));
                }
            }
        }

        match self.socket.recv(buf) {
            Ok(_) => Ok(()),

            Err(e) => {
                return Err(Report::from(e)
                    .change_context(StdUdpClientError::Recv)
                    .attach(FailureReason::new(
                        "Failed to read data from the UDP socket.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }
        }
    }
}
