mod sealed;

use sealed::client::Tcp;
use std::net::SocketAddr;

use crate::{
    error::{
        NetworkError,
        Report,
        Result,
        ResultExt,
        _metadata::{NetworkInterface, NetworkProtocol},
    },
    settings::Timeout,
};

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct TcpClient {
    client: sealed::client::Inner,
}

#[maybe_async::maybe_async]
impl TcpClient {
    #[allow(dead_code)]
    pub(crate) async fn new(addr: &SocketAddr, timeout: Option<&Timeout>) -> Result<Self> {
        Ok(Self {
            client: sealed::client::Inner::new(addr, timeout)
                .await
                .map_err(Report::from)
                .attach_printable("Unable to initialize a TCP Client")
                .attach_printable(format!("Address: {:#?}", addr))
                .attach_printable(format!("Timeout: {:#?}", timeout))
                .change_context(
                    NetworkError::ConnectionError {
                        _protocol: NetworkProtocol::Tcp,
                        _interface: NetworkInterface::Client,
                    }
                    .into(),
                )?,
        })
    }

    #[allow(dead_code)]
    pub(crate) async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>> {
        self.client
            .inner
            .read(size)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to read data from the TCP Client")
            .attach_printable(format!("Size requested: {:#?}", size))
            .change_context(
                NetworkError::ReadError {
                    _protocol: NetworkProtocol::Tcp,
                    _interface: NetworkInterface::Client,
                }
                .into(),
            )
    }

    #[allow(dead_code)]
    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<()> {
        Ok(self
            .client
            .inner
            .write(data)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to write data to the TCP Client")
            .change_context(
                NetworkError::WriteError {
                    _protocol: NetworkProtocol::Tcp,
                    _interface: NetworkInterface::Client,
                }
                .into(),
            )?)
    }
}
