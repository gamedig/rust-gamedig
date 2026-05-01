use {
    crate::core::error::{
        Report,
        diagnostic::{CRATE_INFO, FailureReason},
    },
    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
        time::Duration,
    },
    tokio::{net::UdpSocket, time::timeout as timer},
};

#[derive(Debug, thiserror::Error)]
pub enum TokioUdpClientError {
    #[error("[GameDig]::[UDP::TOKIO::BIND]: Failed to bind the UDP socket")]
    Bind,
    #[error("[GameDig]::[UDP::TOKIO::CONNECT]: Failed to connect the UDP socket")]
    Connect,
    #[error("[GameDig]::[UDP::TOKIO::SEND]: Failed to send data over UDP socket")]
    Send,
    #[error("[GameDig]::[UDP::TOKIO::SEND_TIMEOUT]: Sending data over UDP socket timed out")]
    SendTimeout,
    #[error("[GameDig]::[UDP::TOKIO::RECV]: Failed to receive data from UDP socket")]
    Recv,
    #[error("[GameDig]::[UDP::TOKIO::RECV_TIMEOUT]: Receiving data from UDP socket timed out")]
    RecvTimeout,
}

pub(crate) struct TokioUdpClient {
    socket: UdpSocket,
}

#[maybe_async::async_impl]
impl super::AbstractUdp for TokioUdpClient {
    type Error = Report<TokioUdpClientError>;

    #[cfg_attr(
        feature = "ext_tracing",
        tracing::instrument(
            level = "trace",
            fields(
                addr = %addr,
            )
        )
    )]
    async fn new(addr: SocketAddr) -> Result<Self, Self::Error> {
        match UdpSocket::bind(match addr {
            SocketAddr::V4(_) => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
            SocketAddr::V6(_) => SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)),
        })
        .await
        {
            Ok(socket) => {
                match socket.connect(addr).await {
                    Ok(_) => Ok(Self { socket }),

                    Err(e) => {
                        return Err(Report::from(e)
                            .change_context(TokioUdpClientError::Connect)
                            .attach(FailureReason::new(
                                "Failed to associate the UDP socket with the target address due to an I/O error.",
                            ))
                            .attach(CRATE_INFO));
                    }
                }
            }

            Err(e) => {
                return Err(Report::from(e)
                    .change_context(TokioUdpClientError::Bind)
                    .attach(FailureReason::new(
                        "Failed to bind a local UDP socket due to an I/O error.",
                    ))
                    .attach(CRATE_INFO));
            }
        }
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
    async fn send(&mut self, data: &[u8], timeout: Duration) -> Result<(), Self::Error> {
        match timer(timeout, self.socket.send(data)).await {
            Ok(Ok(_)) => Ok(()),

            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(TokioUdpClientError::Send)
                    .attach(FailureReason::new(
                        "Failed to send data over the UDP socket.",
                    ))
                    .attach(CRATE_INFO));
            }

            Err(e) => {
                return Err(Report::from(e)
                    .change_context(TokioUdpClientError::SendTimeout)
                    .attach(FailureReason::new(
                        "The send operation exceeded the specified timeout duration.",
                    ))
                    .attach(CRATE_INFO));
            }
        }
    }

    #[cfg_attr(
        feature = "ext_tracing",
        tracing::instrument(
            level = "trace",
            skip(self, buf),
            fields(
                buf_len = buf.len(),
                timeout = ?timeout,
            )
        )
    )]
    async fn recv(&mut self, buf: &mut [u8], timeout: Duration) -> Result<usize, Self::Error> {
        match timer(timeout, self.socket.recv(buf)).await {
            Ok(Ok(size)) => Ok(size),

            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(TokioUdpClientError::Recv)
                    .attach(FailureReason::new(
                        "An underlying I/O error occurred during the socket read operation.",
                    ))
                    .attach(CRATE_INFO));
            }

            Err(e) => {
                return Err(Report::from(e)
                    .change_context(TokioUdpClientError::RecvTimeout)
                    .attach(FailureReason::new(
                        "The read operation exceeded the specified timeout duration.",
                    ))
                    .attach(CRATE_INFO));
            }
        }
    }
}
