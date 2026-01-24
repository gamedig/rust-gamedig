use {
    crate::core::error::{
        Report,
        diagnostic::{CRATE_INFO, FailureReason, SYSTEM_INFO},
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
    #[error("[GameDig]::[TCP::TOKIO::CONNECTION]: Failed to establish connection")]
    Connection,
    #[error("[GameDig]::[TCP::TOKIO::CONNECTION_TIMEOUT]: Connection attempt timed out")]
    ConnectionTimeout,

    #[error("[GameDig]::[TCP::TOKIO::READ_EXACT]: Failed to read exact bytes from stream")]
    ReadExact,
    #[error("[GameDig]::[TCP::TOKIO::READ_EXACT_TIMEOUT]: Operation timed out")]
    ReadExactTimeout,

    #[error("[GameDig]::[TCP::TOKIO::READ_TO_END]: Failed to read all bytes from stream")]
    ReadToEnd,
    #[error("[GameDig]::[TCP::TOKIO::READ_TO_END_TIMEOUT]: Operation timed out")]
    ReadToEndTimeout,

    #[error("[GameDig]::[TCP::TOKIO::WRITE]: Failed to write bytes to stream")]
    Write,
    #[error("[GameDig]::[TCP::TOKIO::WRITE_TIMEOUT]: Operation timed out")]
    WriteTimeout,
}

#[derive(Debug)]
pub(crate) struct TokioTcpClient {
    peer_addr: SocketAddr,
    read_stream: Mutex<OwnedReadHalf>,
    write_stream: Mutex<OwnedWriteHalf>,
}

#[maybe_async::async_impl]
impl super::AbstractTcp for TokioTcpClient {
    type Error = Report<TokioTcpClientError>;

    async fn new(addr: SocketAddr, timeout: Duration) -> Result<Self, Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::TCP::CLIENT::TOKIO::<NEW>: {:?}", |f| {
            f.debug_struct("Args")
                .field("addr", &addr)
                .field("timeout", &timeout)
                .finish()
        });

        let (orh, owh) = match timer(timeout, TcpStream::connect(addr)).await {
            // Connection established successfully
            Ok(Ok(stream)) => stream.into_split(),

            // Error during the connection attempt
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(TokioTcpClientError::Connection)
                    .attach(FailureReason::new(
                        "Failed to establish a TCP connection due to an underlying RT or OS I/O \
                         error.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }

            // Connection attempt timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(TokioTcpClientError::ConnectionTimeout)
                    .attach(FailureReason::new(
                        "The connection attempt exceeded the specified timeout duration.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }
        };

        Ok(TokioTcpClient {
            peer_addr: addr,
            read_stream: Mutex::new(orh),
            write_stream: Mutex::new(owh),
        })
    }

    async fn read_exact(&mut self, buf: &mut [u8], timeout: Duration) -> Result<(), Self::Error> {
        dev_trace_fmt!(
            "GAMEDIG::CORE::TCP::CLIENT::TOKIO::<READ_EXACT>: {:?}",
            |f| {
                f.debug_struct("Args")
                    .field("buf", format_args!("len({})", buf.len()))
                    .field("timeout", &timeout)
                    .finish()
            }
        );

        // Await the read stream lock
        let mut orh_mg = self.read_stream.lock().await;
        let orh = &mut *orh_mg;

        match timer(timeout, orh.read_exact(buf)).await {
            // Data read successfully
            Ok(Ok(_)) => Ok(()),

            // Error during the read operation
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(TokioTcpClientError::ReadExact)
                    .attach(FailureReason::new(
                        "An underlying RT or OS I/O error occurred during TCP read operation.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }

            // Read operation timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(TokioTcpClientError::ReadExactTimeout)
                    .attach(FailureReason::new(
                        "The operation exceeded the specified timeout duration.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }
        }
    }

    async fn read_to_end(
        &mut self,
        buf: &mut Vec<u8>,
        timeout: Duration,
    ) -> Result<(), Self::Error> {
        dev_trace_fmt!(
            "GAMEDIG::CORE::TCP::CLIENT::TOKIO::<READ_TO_END>: {:?}",
            |f| {
                f.debug_struct("Args")
                    .field("buf", format_args!("cap({})", buf.capacity()))
                    .field("timeout", &timeout)
                    .finish()
            }
        );

        // Await the read stream lock
        let mut orh_mg = self.read_stream.lock().await;
        let orh = &mut *orh_mg;

        match timer(timeout, orh.read_to_end(buf)).await {
            // Data read successfully
            Ok(Ok(_)) => Ok(()),

            // Error during the read operation
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(TokioTcpClientError::ReadToEnd)
                    .attach(FailureReason::new(
                        "An underlying RT or OS I/O error occurred during TCP read operation.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }

            // Read operation timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(TokioTcpClientError::ReadToEndTimeout)
                    .attach(FailureReason::new(
                        "The operation exceeded the specified timeout duration.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }
        }
    }

    async fn write(&mut self, data: &[u8], timeout: Duration) -> Result<(), Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::TCP::CLIENT::TOKIO::<WRITE>: {:?}", |f| {
            f.debug_struct("Args")
                .field("data", format_args!("len({})", data.len()))
                .field("timeout", &timeout)
                .finish()
        });

        // Await the write stream lock
        let mut owh_mg = self.write_stream.lock().await;
        let owh = &mut *owh_mg;

        match timer(timeout, owh.write_all(data)).await {
            // Data written successfully
            Ok(Ok(_)) => Ok(()),

            // Error during the write operation
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .change_context(TokioTcpClientError::Write)
                    .attach(FailureReason::new(
                        "An underlying RT or OS I/O error occurred during TCP write operation.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }

            // Write operation timed out
            Err(e) => {
                return Err(Report::from(e)
                    .change_context(TokioTcpClientError::WriteTimeout)
                    .attach(FailureReason::new(
                        "The operation exceeded the specified timeout duration.",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO));
            }
        }
    }
}
