use std::{
    fmt::{self, Display, Formatter},
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use error_stack::{Context, Report, Result, ResultExt};

use crate::settings::Timeout;

#[derive(Debug)]
pub(crate) struct SyncStdTcpClient {
    stream: TcpStream,
}

#[maybe_async::sync_impl]
impl super::Tcp for SyncStdTcpClient {
    type Error = SyncStdTcpClientError;

    fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self, SyncStdTcpClientError> {
        let stream = TcpStream::connect_timeout(addr, timeout.connect)
            .map_err(Report::from)
            .attach_printable("Failed to establish a TCP connection")
            .attach_printable(format!("Attempted to connect to address: {addr:?}"))
            .change_context(SyncStdTcpClientError)?;

        stream
            .set_read_timeout(Some(timeout.read))
            .map_err(Report::from)
            .attach_printable("Failed to set read timeout")
            .change_context(SyncStdTcpClientError)?;

        stream
            .set_write_timeout(Some(timeout.write))
            .map_err(Report::from)
            .attach_printable("Failed to set write timeout")
            .change_context(SyncStdTcpClientError)?;

        Ok(Self { stream })
    }

    fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, SyncStdTcpClientError> {
        let mut vec = Vec::with_capacity(size.unwrap_or(Self::DEFAULT_PACKET_SIZE as usize));

        self.stream
            .read_to_end(&mut vec)
            .map_err(Report::from)
            .attach_printable("Failed to read data from the TCP stream")
            .change_context(SyncStdTcpClientError)?;

        Ok(vec)
    }

    fn write(&mut self, data: &[u8]) -> Result<(), SyncStdTcpClientError> {
        self.stream
            .write_all(data)
            .map_err(Report::from)
            .attach_printable("Failed to write data to the TCP stream")
            .change_context(SyncStdTcpClientError)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct SyncStdTcpClientError;

impl Context for SyncStdTcpClientError {}

impl Display for SyncStdTcpClientError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "GameDig Core Net Runtime Error (sync_std_tcp_client)")
    }
}
