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

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::{
        io::{ReadExt, WriteExt},
        net::TcpListener,
        task,
    };

    use std::{net::SocketAddr, time::Duration};

    use crate::core::net::tcp::Tcp;

    async fn create_mock_server() -> (SocketAddr, TcpListener) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (addr, listener)
    }

    #[async_std::test]
    async fn test_new_success() {
        let (addr, _listener) = create_mock_server().await;
        let client = AsyncStdTcpClient::new(&addr, &Timeout::DEFAULT).await;

        assert!(
            client.is_ok(),
            "Expected OK, Received Err: {:?}",
            client.err()
        );
    }

    #[async_std::test]
    async fn test_new_connection_failure() {
        let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        let client = AsyncStdTcpClient::new(&addr, &Timeout::DEFAULT).await;

        assert!(
            client.is_err(),
            "Expected Err, Received Ok: {:?}",
            client.ok()
        );
    }

    #[async_std::test]
    async fn test_read_success() {
        let (addr, listener) = create_mock_server().await;
        let mut client = AsyncStdTcpClient::new(&addr, &Timeout::DEFAULT)
            .await
            .unwrap();

        task::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            socket.write_all(b"hello").await.unwrap();
        });

        let result = client.read(Some(5)).await;

        assert!(
            result.is_ok(),
            "Expected Ok with data, Received Err: {:?}",
            result.err()
        );

        assert_eq!(result.unwrap(), b"hello");
    }

    #[async_std::test]
    async fn test_read_timeout() {
        let (addr, listener) = create_mock_server().await;
        let mut client = AsyncStdTcpClient::new(
            &addr,
            &Timeout::new(None, Some(Duration::from_millis(100)), None, None),
        )
        .await
        .unwrap();

        task::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();

            task::sleep(Duration::from_secs(1)).await;
            drop(socket);
        });

        let result = client.read(Some(5)).await;

        assert!(
            result.is_err(),
            "Expected Err due to timeout, Received Ok: {:?}",
            result.ok()
        );
    }

    #[async_std::test]
    async fn test_write_success() {
        let (addr, listener) = create_mock_server().await;
        let mut client = AsyncStdTcpClient::new(&addr, &Timeout::DEFAULT)
            .await
            .unwrap();

        task::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();

            let mut buf = vec![0; 5];
            socket.read_exact(&mut buf).await.unwrap();

            assert_eq!(buf, b"hello");
        });

        let result = client.write(b"hello").await;

        assert!(
            result.is_ok(),
            "Expected Ok, Received Err: {:?}",
            result.err()
        );
    }
}
