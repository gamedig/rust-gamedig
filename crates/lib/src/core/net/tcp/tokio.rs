use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
};

use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
    sync::Arc,
};

use error_stack::{Context, Report, Result, ResultExt};

pub(super) struct AsyncTokioTcpClient {
    read_stream: Arc<Mutex<OwnedReadHalf>>,
    write_stream: Arc<Mutex<OwnedWriteHalf>>,
}

#[maybe_async::async_impl]
impl super::Tcp for AsyncTokioTcpClient {
    type Error = AsyncTokioTcpClientError;

    async fn new(addr: &SocketAddr) -> Result<Self, AsyncTokioTcpClientError> {
        let (orh, owh) = TcpStream::connect(addr)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to establish a TCP connection")
            .attach_printable(format!("Attempted to connect to address: {addr:?}"))
            .change_context(AsyncTokioTcpClientError)?
            .into_split();

        Ok(AsyncTokioTcpClient {
            read_stream: Arc::new(Mutex::new(orh)),
            write_stream: Arc::new(Mutex::new(owh)),
        })
    }

    async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, AsyncTokioTcpClientError> {
        let read_half = Arc::clone(&self.read_stream);
        let mut orh = read_half.lock().await;

        let mut buf = Vec::with_capacity(size.unwrap_or(Self::DEFAULT_PACKET_SIZE as usize));

        orh.read_to_end(&mut buf)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to read data from its half of the TCP split stream")
            .change_context(AsyncTokioTcpClientError)?;

        Ok(buf)
    }

    async fn write(&mut self, data: &[u8]) -> Result<(), AsyncTokioTcpClientError> {
        let write_half = Arc::clone(&self.write_stream);
        let mut owh = write_half.lock().await;

        owh.write(data)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to write data to its half of the TCP split stream")
            .change_context(AsyncTokioTcpClientError)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct AsyncTokioTcpClientError;

impl Context for AsyncTokioTcpClientError {}

impl Display for AsyncTokioTcpClientError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "GameDig Core Net Runtime Error (async_tokio_tcp_client)"
        )
    }
}
