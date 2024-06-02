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
                .change_context(AsyncStdTcpClientError)?,
        })
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, AsyncStdTcpClientError> {
        Ok(self
            .stream
            .read(buf)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to read data from the TCP stream")
            .change_context(AsyncStdTcpClientError)?)
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize, AsyncStdTcpClientError> {
        Ok(self
            .stream
            .write(buf)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to write data to the TCP stream")
            .change_context(AsyncStdTcpClientError)?)
    }
}

#[derive(Debug)]
pub struct AsyncStdTcpClientError;

impl Context for AsyncStdTcpClientError {}

impl Display for AsyncStdTcpClientError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "GameDig Core Net Async Std Runtime Error: AsyncStdTcpClient"
        )
    }
}
