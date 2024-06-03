use async_std::{
    io::{ReadExt, WriteExt},
    net::TcpStream,
};

use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
};

use error_stack::{Context, Report, Result, ResultExt};

pub(super) struct AsyncStdTcpClient {
    stream: TcpStream,
}

#[maybe_async::async_impl]
impl super::Tcp for AsyncStdTcpClient {
    type Error = AsyncStdTcpClientError;

    async fn new(addr: &SocketAddr) -> Result<Self, AsyncStdTcpClientError> {
        Ok(Self {
            stream: TcpStream::connect(addr)
                .await
                .map_err(Report::from)
                .attach_printable("Failed to establish a TCP connection")
                .attach_printable(format!("Attempted to connect to address: {addr:?}"))
                .change_context(AsyncStdTcpClientError)?,
        })
    }

    async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, AsyncStdTcpClientError> {
        let mut buf = Vec::with_capacity(size.unwrap_or(Self::DEFAULT_PACKET_SIZE as usize));

        self.stream
            .read_to_end(&mut buf)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to read data from the TCP stream")
            .change_context(AsyncStdTcpClientError)?;

        Ok(buf)
    }

    async fn write(&mut self, data: &[u8]) -> Result<(), AsyncStdTcpClientError> {
        self.stream
            .write(data)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to write data to the TCP stream")
            .change_context(AsyncStdTcpClientError)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct AsyncStdTcpClientError;

impl Context for AsyncStdTcpClientError {}

impl Display for AsyncStdTcpClientError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "GameDig Core Net Runtime Error (async_std_tcp_client)")
    }
}
