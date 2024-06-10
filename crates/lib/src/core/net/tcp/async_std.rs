use async_std::{
    future::timeout as async_timeout,
    io::{ReadExt, WriteExt},
    net::TcpStream,
};

use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
    time::Duration,
};

use error_stack::{Context, Report, Result};

use crate::settings::Timeout;

pub(super) struct AsyncStdTcpClient {
    stream: TcpStream,
    read_timeout: Duration,
    write_timeout: Duration,
}

#[maybe_async::async_impl]
impl super::Tcp for AsyncStdTcpClient {
    type Error = AsyncStdTcpClientError;

    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self, AsyncStdTcpClientError> {
        Ok(Self {
            stream: match async_timeout(timeout.connect, TcpStream::connect(addr)).await {
                Ok(Ok(stream)) => stream,
                Ok(Err(e)) => {
                    return Err(Report::from(e)
                        .attach_printable("Failed to establish a TCP connection")
                        .attach_printable(format!("Attempted to connect to address: {addr:?}"))
                        .change_context(AsyncStdTcpClientError));
                }
                Err(e) => {
                    return Err(Report::from(e)
                        .attach_printable("Connection operation timed out")
                        .attach_printable(format!("Attempted to connect to address: {addr:?}"))
                        .change_context(AsyncStdTcpClientError));
                }
            },
            read_timeout: timeout.read,
            write_timeout: timeout.write,
        })
    }

    async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, AsyncStdTcpClientError> {
        let mut buf = Vec::with_capacity(size.unwrap_or(Self::DEFAULT_PACKET_SIZE as usize));

        match async_timeout(self.read_timeout, self.stream.read_to_end(&mut buf)).await {
            Ok(Ok(_)) => Ok(buf),
            Ok(Err(e)) => {
                Err(Report::from(e)
                    .attach_printable("Failed to read data from the TCP stream")
                    .change_context(AsyncStdTcpClientError))
            }
            Err(e) => {
                Err(Report::from(e)
                    .attach_printable("Read operation timed out")
                    .change_context(AsyncStdTcpClientError))
            }
        }
    }

    async fn write(&mut self, data: &[u8]) -> Result<(), AsyncStdTcpClientError> {
        match async_timeout(self.write_timeout, self.stream.write_all(data)).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => {
                Err(Report::from(e)
                    .attach_printable("Failed to write data to the TCP stream")
                    .change_context(AsyncStdTcpClientError))
            }
            Err(e) => {
                Err(Report::from(e)
                    .attach_printable("Write operation timed out")
                    .change_context(AsyncStdTcpClientError))
            }
        }
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
