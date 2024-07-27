use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use crate::{
    error::{NetworkError, Report, Result, _metadata::NetworkProtocol},
    settings::Timeout,
};

use log::{debug, trace};

#[derive(Debug)]
pub(crate) struct StdTcpClient {
    addr: SocketAddr,
    stream: TcpStream,
}

#[maybe_async::sync_impl]
impl super::Tcp for StdTcpClient {
    fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self> {
        trace!("TCP::<Std>::New: Creating new TCP client for {addr} with timeout: {timeout:?}");

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
        trace!(
            "TCP::<Std>::Read: Reading data from {} with size: {:?}",
            self.addr,
            size
        );

        let validated_size = match size {
            Some(size) => size.min(Self::MAX_TCP_PACKET_SIZE) as usize,
            None => Self::MAX_TCP_PACKET_SIZE as usize,
        };

        let mut vec = Vec::with_capacity(validated_size);

        match self.stream.read_to_end(&mut vec) {
            Ok(len) => {
                if validated_size < len {
                    debug!(
                        "TCP::<Std>::Read: More data than expected. Realloc was required. \
                         Expected: {validated_size} bytes, Read: {len} bytes",
                    );
                }

                let capacity = vec.capacity();
                if capacity * (Self::VEC_CAPACITY_SHRINK_MARGIN as usize)
                    > (len * Self::VEC_CAPACITY_BASE_UNIT as usize)
                {
                    debug!(
                        "TCP::<Std>::Read: Shrink threshold exceeded. Shrinking vec to fit. \
                         Capacity: {capacity}, Read: {len} bytes",
                    );

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
        trace!(
            "TCP::<Std>::Write: Writing data to {} with size: {}",
            self.addr,
            data.len()
        );

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
