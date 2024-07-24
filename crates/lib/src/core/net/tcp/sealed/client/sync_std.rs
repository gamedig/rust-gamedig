use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use crate::{
    error::{NetworkError, Report, Result, _metadata::NetworkProtocol},
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
        match TcpStream::connect_timeout(addr, timeout.connect) {
            Ok(stream) => {
                match stream.set_read_timeout(Some(timeout.read)) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(Report::from(e).change_context(
                            NetworkError::SetTimeoutError {
                                _protocol: NetworkProtocol::Tcp,
                                addr: *addr,
                            }
                            .into(),
                        ));
                    }
                }

                match stream.set_write_timeout(Some(timeout.write)) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(Report::from(e).change_context(
                            NetworkError::SetTimeoutError {
                                _protocol: NetworkProtocol::Tcp,
                                addr: *addr,
                            }
                            .into(),
                        ));
                    }
                }

                Ok(Self {
                    addr: *addr,
                    stream,
                })
            }
            Err(e) => {
                Err(Report::from(e).change_context(
                    NetworkError::ConnectionError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: *addr,
                    }
                    .into(),
                ))
            }
        }
    }

    fn read(&mut self, size: Option<u16>) -> Result<Vec<u8>> {
        let mut vec = Vec::with_capacity(match size {
            Some(size) => size.min(Self::MAX_TCP_PACKET_SIZE) as usize,
            None => Self::MAX_TCP_PACKET_SIZE as usize,
        });

        match self.stream.read_to_end(&mut vec) {
            Ok(len) => {
                if vec.capacity() * (Self::VEC_CAPACITY_SHRINK_MARGIN as usize) > (len << 7) {
                    vec.shrink_to_fit();
                }

                Ok(vec)
            }
            Err(e) => {
                Err(Report::from(e).change_context(
                    NetworkError::ReadError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: self.addr,
                    }
                    .into(),
                ))
            }
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        match self.stream.write_all(data) {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(Report::from(e).change_context(
                    NetworkError::WriteError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: self.addr,
                    }
                    .into(),
                ))
            }
        }
    }
}
