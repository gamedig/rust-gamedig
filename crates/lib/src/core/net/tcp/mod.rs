mod sealed;

use sealed::client::Tcp;
use std::net::SocketAddr;

use crate::{error::Result, settings::Timeout};

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
            client: sealed::client::Inner::new(addr, timeout).await?,
        })
    }

    #[allow(dead_code)]
    pub(crate) async fn read(&mut self, size: Option<u16>) -> Result<Vec<u8>> {
        self.client.inner.read(size).await
    }

    #[allow(dead_code)]
    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<()> {
        self.client.inner.write(data).await
    }
}
