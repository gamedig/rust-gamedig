use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
    time::timeout as tokio_timeout,
};

use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};

use error_stack::{Context, Report, Result};

use crate::settings::Timeout;

#[derive(Debug)]
pub(super) struct AsyncTokioTcpClient {
    read_stream: Arc<Mutex<OwnedReadHalf>>,
    read_timeout: Duration,
    write_stream: Arc<Mutex<OwnedWriteHalf>>,
    write_timeout: Duration,
}

#[maybe_async::async_impl]
impl super::Tcp for AsyncTokioTcpClient {
    type Error = AsyncTokioTcpClientError;

    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self, AsyncTokioTcpClientError> {
        let (orh, owh) = match tokio_timeout(timeout.connect, TcpStream::connect(addr)).await {
            Ok(Ok(stream)) => stream.into_split(),
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .attach_printable("Failed to establish a TCP connection")
                    .attach_printable(format!("Attempted to connect to address: {addr:?}"))
                    .change_context(AsyncTokioTcpClientError));
            }
            Err(e) => {
                return Err(Report::from(e)
                    .attach_printable("Connection operation timed out")
                    .attach_printable(format!("Attempted to connect to address: {addr:?}"))
                    .change_context(AsyncTokioTcpClientError));
            }
        };

        Ok(AsyncTokioTcpClient {
            read_stream: Arc::new(Mutex::new(orh)),
            read_timeout: timeout.read,
            write_stream: Arc::new(Mutex::new(owh)),
            write_timeout: timeout.write,
        })
    }

    async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, AsyncTokioTcpClientError> {
        let read_half = Arc::clone(&self.read_stream);
        let mut orh = read_half.lock().await;

        let mut buf = Vec::with_capacity(size.unwrap_or(Self::DEFAULT_PACKET_SIZE as usize));

        match tokio_timeout(self.read_timeout, orh.read_to_end(&mut buf)).await {
            Ok(Ok(_)) => Ok(buf),
            Ok(Err(e)) => {
                Err(Report::from(e)
                    .attach_printable("Failed to read data from its half of the TCP split stream")
                    .change_context(AsyncTokioTcpClientError))
            }
            Err(e) => {
                Err(Report::from(e)
                    .attach_printable("Read operation timed out")
                    .change_context(AsyncTokioTcpClientError))
            }
        }
    }

    async fn write(&mut self, data: &[u8]) -> Result<(), AsyncTokioTcpClientError> {
        let write_half = Arc::clone(&self.write_stream);
        let mut owh = write_half.lock().await;

        match tokio_timeout(self.write_timeout, owh.write_all(data)).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => {
                Err(Report::from(e)
                    .attach_printable("Failed to write data to its half of the TCP split stream")
                    .change_context(AsyncTokioTcpClientError))
            }
            Err(e) => {
                Err(Report::from(e)
                    .attach_printable("Write operation timed out")
                    .change_context(AsyncTokioTcpClientError))
            }
        }
    }
}

#[derive(Debug)]
pub struct AsyncTokioTcpClientError;

impl Context for AsyncTokioTcpClientError {}

impl Display for AsyncTokioTcpClientError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "GameDig Core Net Runtime Error (async_tokio_tcp_client)"
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::core::net::tcp::Tcp;

    use super::*;

    use std::net::SocketAddr;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
        time::Duration,
    };

    // Helper function to create a mock TcpListener and return its address
    async fn create_mock_server() -> (SocketAddr, TcpListener) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (addr, listener)
    }

    #[tokio::test]
    async fn test_new_success() {
        let (addr, _listener) = create_mock_server().await;
        let timeout = Timeout::new(Some(Duration::from_secs(1)), None, None, None);

        let client = AsyncTokioTcpClient::new(&addr, &timeout).await;
        assert!(
            client.is_ok(),
            "Expected Ok(AsyncTokioTcpClient), got Err: {:?}",
            client.err()
        );
    }

    #[tokio::test]
    async fn test_new_connection_failure() {
        let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        let timeout = Timeout::new(Some(Duration::from_secs(1)), None, None, None);

        let client = AsyncTokioTcpClient::new(&addr, &timeout).await;
        assert!(client.is_err(), "Expected Err, got Ok: {:?}", client.ok());
    }

    #[tokio::test]
    async fn test_read_success() {
        let (addr, listener) = create_mock_server().await;
        let timeout = Timeout::new(
            Some(Duration::from_secs(1)),
            Some(Duration::from_secs(1)),
            None,
            None,
        );

        let mut client = AsyncTokioTcpClient::new(&addr, &timeout).await.unwrap();

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let _ = socket.write_all(b"hello").await;
        });

        let result = client.read(Some(5)).await;
        assert!(
            result.is_ok(),
            "Expected Ok with data, got Err: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), b"hello");
    }

    #[tokio::test]
    async fn test_read_timeout() {
        let (addr, listener) = create_mock_server().await;
        let timeout = Timeout::new(
            Some(Duration::from_secs(1)),
            Some(Duration::from_millis(100)),
            None,
            None,
        );

        let mut client = AsyncTokioTcpClient::new(&addr, &timeout).await.unwrap();

        tokio::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            // Simulate a delay in the server to cause a read timeout
            tokio::time::sleep(Duration::from_secs(1)).await;
            drop(socket);
        });

        let result = client.read(Some(5)).await;
        assert!(
            result.is_err(),
            "Expected Err due to timeout, got Ok: {:?}",
            result.ok()
        );
    }

    #[tokio::test]
    async fn test_write_success() {
        let (addr, listener) = create_mock_server().await;
        let timeout = Timeout::new(
            Some(Duration::from_secs(1)),
            None,
            Some(Duration::from_secs(1)),
            None,
        );

        let mut client = AsyncTokioTcpClient::new(&addr, &timeout).await.unwrap();

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = vec![0; 5];
            let _ = socket.read_exact(&mut buf).await;
            assert_eq!(buf, b"hello");
        });

        let result = client.write(b"hello").await;
        assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result.err());
    }

    // Not implemented due to the lack of a way to simulate a write timeout
    // Write was successful in less than 1 nanosecond with [0; 65536]
    //
    // #[tokio::test]
    // async fn test_write_timeout() {
    //    unimplemented!();
    // }
}
