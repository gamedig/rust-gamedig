use {
    crate::core::error::{
        Report,
        diagnostic::{CRATE_INFO, FailureReason},
    },
    std::{net::SocketAddr, time::Duration},
    tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::{
            TcpStream,
            tcp::{OwnedReadHalf, OwnedWriteHalf},
        },
        sync::Mutex,
        time::timeout as timer,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum TokioTcpClientError {
    #[error("[GameDig]::[TCP::TOKIO::CONNECTION]: Failed to establish TCP connection")]
    Connection,
    #[error("[GameDig]::[TCP::TOKIO::CONNECTION_TIMEOUT]: TCP connection attempt timed out")]
    ConnectionTimeout,

    #[error("[GameDig]::[TCP::TOKIO::READ_EXACT]: Failed to read required bytes from TCP stream")]
    ReadExact,
    #[error("[GameDig]::[TCP::TOKIO::READ_EXACT_TIMEOUT]: Timed out while reading required bytes from TCP stream")]
    ReadExactTimeout,

    #[error("[GameDig]::[TCP::TOKIO::READ_TO_END]: Failed to read remaining data from TCP stream")]
    ReadToEnd,
    #[error("[GameDig]::[TCP::TOKIO::READ_TO_END_TIMEOUT]: Timed out while reading remaining data from TCP stream")]
    ReadToEndTimeout,

    #[error("[GameDig]::[TCP::TOKIO::WRITE]: Failed to write data to TCP stream")]
    Write,
    #[error("[GameDig]::[TCP::TOKIO::WRITE_TIMEOUT]: Timed out while writing data to TCP stream")]
    WriteTimeout,
}

#[derive(Debug)]
pub(crate) struct TokioTcpClient {
    read_stream: Mutex<OwnedReadHalf>,
    write_stream: Mutex<OwnedWriteHalf>,
}

#[maybe_async::async_impl]
impl super::AbstractTcp for TokioTcpClient {
    type Error = Report<TokioTcpClientError>;

    #[cfg_attr(
        feature = "ext_tracing",
        tracing::instrument(
            level = "trace",
            fields(
                addr = %addr,
                timeout = ?timeout,
            )
        )
    )]
    async fn new(addr: SocketAddr, timeout: Duration) -> Result<Self, Self::Error> {
        let (orh, owh) = match timer(timeout, TcpStream::connect(addr)).await {
            Ok(Ok(stream)) => stream.into_split(),

            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(TokioTcpClientError::Connection)
                    .attach(FailureReason::new(
                        "Failed to establish a TCP connection due to an I/O error.",
                    ))
                    .attach(CRATE_INFO));
            }

            Err(e) => {
                return Err(Report::from(e)
                    .change_context(TokioTcpClientError::ConnectionTimeout)
                    .attach(FailureReason::new("TCP connection attempt timed out."))
                    .attach(CRATE_INFO));
            }
        };

        Ok(TokioTcpClient {
            read_stream: Mutex::new(orh),
            write_stream: Mutex::new(owh),
        })
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
    async fn read_exact(&mut self, buf: &mut [u8], timeout: Duration) -> Result<(), Self::Error> {
        match timer(timeout, async {
            let mut guard = self.read_stream.lock().await;

            guard.read_exact(buf).await
        })
        .await
        {
            Ok(Ok(_)) => Ok(()),

            Ok(Err(e)) => {
                Err(Report::from(e)
                    .change_context(TokioTcpClientError::ReadExact)
                    .attach(FailureReason::new(
                        "Failed to read the requested number of bytes from the TCP stream due to an I/O error.",
                    ))
                    .attach(CRATE_INFO))
            }

            Err(e) => {
                Err(Report::from(e)
                    .change_context(TokioTcpClientError::ReadExactTimeout)
                    .attach(FailureReason::new(
                        "Timed out before reading the requested number of bytes from the TCP stream.",
                    ))
                    .attach(CRATE_INFO))
            }
        }
    }

    #[cfg_attr(
        feature = "ext_tracing",
        tracing::instrument(
            level = "trace",
            skip(self, buf),
            fields(
                buf_cap = buf.capacity(),
                timeout = ?timeout,
            )
        )
    )]
    async fn read_to_end(&mut self, buf: &mut Vec<u8>, timeout: Duration) -> Result<usize, Self::Error> {
        match timer(timeout, async {
            let mut guard = self.read_stream.lock().await;

            guard.read_to_end(buf).await
        })
        .await
        {
            Ok(Ok(size)) => {
                #[cfg(feature = "ext_tracing")]
                tracing::debug!(bytes_read = size, "read_to_end completed");

                Ok(size)
            }

            Ok(Err(e)) => {
                Err(Report::from(e)
                    .change_context(TokioTcpClientError::ReadToEnd)
                    .attach(FailureReason::new(
                        "Failed to read all remaining data from the TCP stream due to an I/O error.",
                    ))
                    .attach(CRATE_INFO))
            }

            Err(e) => {
                Err(Report::from(e)
                    .change_context(TokioTcpClientError::ReadToEndTimeout)
                    .attach(FailureReason::new(
                        "Timed out before reading all remaining data from the TCP stream.",
                    ))
                    .attach(CRATE_INFO))
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
    async fn write(&mut self, data: &[u8], timeout: Duration) -> Result<(), Self::Error> {
        match timer(timeout, async {
            let mut guard = self.write_stream.lock().await;

            guard.write_all(data).await
        })
        .await
        {
            Ok(Ok(())) => Ok(()),

            Ok(Err(e)) => {
                Err(Report::from(e)
                    .change_context(TokioTcpClientError::Write)
                    .attach(FailureReason::new(
                        "Failed to write all provided data to the TCP stream due to an I/O error.",
                    ))
                    .attach(CRATE_INFO))
            }

            Err(e) => {
                Err(Report::from(e)
                    .change_context(TokioTcpClientError::WriteTimeout)
                    .attach(FailureReason::new(
                        "Timed out before writing all provided data to the TCP stream.",
                    ))
                    .attach(CRATE_INFO))
            }
        }
    }
}
