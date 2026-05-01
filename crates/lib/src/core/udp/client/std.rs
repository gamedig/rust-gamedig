use {
    crate::core::error::{
        Report,
        ResultExt,
        diagnostic::{CRATE_INFO, FailureReason, wrap},
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

    #[error("[GameDig]::[UDP::STD::SET_RECV_TIMEOUT]: Failed to set receive timeout for UDP socket")]
    SetRecvTimeout,
}

pub(crate) struct StdUdpClient {
    socket: UdpSocket,
}

#[maybe_async::sync_impl]
impl super::AbstractUdp for StdUdpClient {
    type Error = Report<StdUdpClientError>;

    #[cfg_attr(
        feature = "ext_tracing",
        tracing::instrument(
            level = "trace",
            fields(
                addr = ?addr,
            )
        )
    )]
    fn new(addr: SocketAddr) -> Result<Self, Self::Error> {
        let socket = UdpSocket::bind(match addr {
            SocketAddr::V4(_) => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
            SocketAddr::V6(_) => SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)),
        })
        .change_context(StdUdpClientError::Bind)
        .attach(FailureReason::new(wrap!(
            "The UDP client attempted to bind a local socket to an unspecified local address so the operating system could \
             choose an appropriate local interface and assign an ephemeral port for communication. This may fail if the process \
             lacks sufficient permissions, no local ports are available, or the network stack cannot allocate a new socket."
        )))
        .attach(CRATE_INFO)?;

        socket
            .connect(addr)
            .change_context(StdUdpClientError::Connect)
            .attach(FailureReason::new(wrap!(
                "The UDP client attempted to set a default peer address for the UDP socket so outgoing datagrams could be sent \
                 without specifying a destination and incoming datagrams would be accepted only from that peer. This may fail \
                 if the target address is invalid or if the operating system cannot determine a valid route to the destination."
            )))
            .attach(CRATE_INFO)?;

        Ok(Self { socket })
    }

    #[cfg_attr(
        feature = "ext_tracing",
        tracing::instrument(
            level = "trace",
            skip(self),
            fields(
                data = ?data,
                timeout = ?timeout,
            )
        )
    )]
    fn send(&mut self, data: &[u8], timeout: Duration) -> Result<(), Self::Error> {
        self.socket
            .set_write_timeout(Some(timeout))
            .change_context(StdUdpClientError::SetSendTimeout)
            .attach(FailureReason::new(wrap!(
                "The UDP client attempted to configure a write timeout on the socket before sending a request to prevent the \
                 operation from blocking indefinitely. This may fail if the timeout value is invalid, cannot be represented by \
                 the operating system or if the socket is in an invalid state."
            )))
            .attach(CRATE_INFO)?;

        self.socket
            .send(data)
            .change_context(StdUdpClientError::Send)
            .attach(FailureReason::new(wrap!(
                "The UDP client attempted to send a datagram containing the request payload to the configured remote peer. This \
                 may fail if the network is unavailable, the operating system rejects the datagram because of resource \
                 limitations or socket state."
            )))
            .attach(CRATE_INFO)?;

        Ok(())
    }

    #[cfg_attr(
        feature = "ext_tracing",
        tracing::instrument(
            level = "trace",
            skip(self, buf),
            fields(
                timeout = ?timeout,
            )
        )
    )]
    fn recv(&mut self, buf: &mut [u8], timeout: Duration) -> Result<usize, Self::Error> {
        self.socket
            .set_read_timeout(Some(timeout))
            .change_context(StdUdpClientError::SetRecvTimeout)
            .attach(FailureReason::new(wrap!(
                "The UDP client attempted to configure a read timeout on the socket before receiving a response to prevent the \
                 operation from blocking indefinitely. This may fail if the timeout value is invalid, cannot be represented by \
                 the operating system or if the socket is in an invalid state."
            )))
            .attach(CRATE_INFO)?;

        let size = self
            .socket
            .recv(buf)
            .change_context(StdUdpClientError::Recv)
            .attach(FailureReason::new(wrap!(
                "The UDP client attempted to receive a datagram from the configured remote peer into the provided buffer. This \
                 may fail if the remote endpoint reports an error, if the socket is in an invalid state, or if the network \
                 stack encounters an error while waiting for data."
            )))
            .attach(CRATE_INFO)?;

        Ok(size)
    }
}
