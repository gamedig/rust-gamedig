use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use crate::{
    error::{NetworkError, Report, Result, ResultExt, _metadata::NetworkProtocol},
    settings::Timeout,
};

#[derive(Debug)]
pub(crate) struct SyncStdTcpClient {
    addr: SocketAddr,
    stream: TcpStream,
}

#[maybe_async::sync_impl]
impl super::Tcp for SyncStdTcpClient {
    fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self> {
        let stream = TcpStream::connect_timeout(addr, timeout.connect)
            .map_err(Report::from)
            .change_context(
                NetworkError::ConnectionError {
                    _protocol: NetworkProtocol::Tcp,
                    addr: *addr,
                }
                .into(),
            )?;

        stream
            .set_read_timeout(Some(timeout.read))
            .map_err(Report::from)
            .attach_printable("Failed to set read timeout")
            .change_context(
                NetworkError::SetTimeoutError {
                    _protocol: NetworkProtocol::Tcp,
                    addr: *addr,
                }
                .into(),
            )?;

        stream
            .set_write_timeout(Some(timeout.write))
            .map_err(Report::from)
            .attach_printable("Failed to set write timeout")
            .change_context(
                NetworkError::SetTimeoutError {
                    _protocol: NetworkProtocol::Tcp,
                    addr: *addr,
                }
                .into(),
            )?;

        Ok(Self {
            addr: *addr,
            stream,
        })
    }

    fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>> {
        let mut vec = Vec::with_capacity(size.unwrap_or(Self::DEFAULT_PACKET_SIZE as usize));

        self.stream
            .read_to_end(&mut vec)
            .map_err(Report::from)
            .change_context(
                NetworkError::ReadError {
                    _protocol: NetworkProtocol::Tcp,
                    addr: self.addr,
                }
                .into(),
            )?;

        Ok(vec)
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        self.stream
            .write_all(data)
            .map_err(Report::from)
            .change_context(
                NetworkError::WriteError {
                    _protocol: NetworkProtocol::Tcp,
                    addr: self.addr,
                }
                .into(),
            )?;

        Ok(())
    }
}
