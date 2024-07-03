mod sealed;

use sealed::client::Tcp;

use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
};

use error_stack::{Context, Report, Result, ResultExt};

use crate::{core::io::Buffer, settings::Timeout};

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct TcpClient {
    client: sealed::client::Inner,
}

#[maybe_async::maybe_async]
impl TcpClient {
    #[allow(dead_code)]
    pub(crate) async fn new(
        addr: &SocketAddr,
        timeout: Option<&Timeout>,
    ) -> Result<Self, TCPClientError> {
        Ok(Self {
            client: sealed::client::Inner::new(addr, timeout)
                .await
                .map_err(Report::from)
                .attach_printable("Unable to create a TCP client")
                .change_context(TCPClientError)?,
        })
    }

    #[allow(dead_code)]
    pub(crate) async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, TCPClientError> {
        self.client
            .inner
            .read(size)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to read data from the TCP Client")
            .change_context(TCPClientError)
    }

    #[allow(dead_code)]
    pub(crate) async fn read_into_buf<B: byteorder::ByteOrder>(
        &mut self,
        size: Option<usize>,
    ) -> Result<Buffer<B>, TCPClientError> {
        Ok(Buffer::<B>::new(
            self.read(size)
                .await
                .map_err(Report::from)
                .attach_printable("Failed to read data from the TCP Client")
                .change_context(TCPClientError)?,
        ))
    }

    #[allow(dead_code)]
    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<(), TCPClientError> {
        Ok(self
            .client
            .inner
            .write(data)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to write data to the TCP Client")
            .change_context(TCPClientError)?)
    }
}

#[derive(Debug)]
pub struct TCPClientError;

impl Context for TCPClientError {}

impl Display for TCPClientError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "GameDig Core Net Runtime Error (tcp_client)")
    }
}
