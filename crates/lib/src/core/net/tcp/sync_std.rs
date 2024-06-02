use std::{
    fmt::{self, Display, Formatter},
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use error_stack::{Context, Report, Result, ResultExt};

pub(super) struct SyncStdTcpClient {
    stream: TcpStream,
}

#[maybe_async::sync_impl]
impl super::Tcp for SyncStdTcpClient {
    type Error = SyncStdTcpClientError;

    fn new(addr: &SocketAddr) -> Result<Self, SyncStdTcpClientError> {
        Ok(Self {
            stream: TcpStream::connect(addr)
                .map_err(Report::from)
                .attach_printable("Failed to establish a TCP connection")
                .change_context(SyncStdTcpClientError)?,
        })
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, SyncStdTcpClientError> {
        Ok(self
            .stream
            .read(buf)
            .map_err(Report::from)
            .attach_printable("Failed to read data from the TCP stream")
            .change_context(SyncStdTcpClientError)?)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, SyncStdTcpClientError> {
        Ok(self
            .stream
            .write(buf)
            .map_err(Report::from)
            .attach_printable("Failed to write data to the TCP stream")
            .change_context(SyncStdTcpClientError)?)
    }
}

#[derive(Debug)]
pub struct SyncStdTcpClientError;

impl Context for SyncStdTcpClientError {}

impl Display for SyncStdTcpClientError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "GameDig Core Net Sync Std Runtime Error: SyncStdTcpClient"
        )
    }
}
