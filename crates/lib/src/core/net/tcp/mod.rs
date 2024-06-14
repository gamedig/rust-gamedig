#[cfg(feature = "async-std-client")]
mod async_std;
#[cfg(feature = "sync-std-client")]
mod sync_std;
#[cfg(feature = "async-tokio-client")]
mod tokio;

use std::{
    fmt::{self, Display, Formatter},
    net::SocketAddr,
};

use error_stack::{Context, Report, Result, ResultExt};

use crate::settings::timeout::Timeout;

#[derive(Debug)]
pub(crate) struct TcpClient {
    #[cfg(feature = "async-tokio-client")]
    inner: tokio::AsyncTokioTcpClient,

    #[cfg(feature = "sync-std-client")]
    inner: sync_std::SyncStdTcpClient,

    #[cfg(feature = "async-std-client")]
    inner: async_std::AsyncStdTcpClient,
}

#[maybe_async::maybe_async]
impl TcpClient {
    pub(crate) async fn new(
        addr: &SocketAddr,
        timeout: Option<&Timeout>,
    ) -> Result<Self, TCPClientError> {
        let timeout = timeout.unwrap_or(&Timeout::DEFAULT);

        
        Ok(Self {
            #[cfg(feature = "async-tokio-client")]
            inner: tokio::AsyncTokioTcpClient::new(addr, timeout)
                .await
                .map_err(Report::from)
                .attach_printable("Unable to create a tokio TCP client")
                .change_context(TCPClientError)?,

            #[cfg(feature = "sync-std-client")]
            inner: sync_std::SyncStdTcpClient::new(addr, timeout)
                .map_err(Report::from)
                .attach_printable("Unable to create a sync std TCP client")
                .change_context(TCPClientError)?,

            #[cfg(feature = "async-std-client")]
            inner: async_std::AsyncStdTcpClient::new(addr, timeout)
                .await
                .map_err(Report::from)
                .attach_printable("Unable to create an async std TCP client")
                .change_context(TCPClientError)?,
        })
    }

    pub(crate) async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, TCPClientError> {
        Ok(self
            .inner
            .read(size)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to read data from the TCP Client")
            .change_context(TCPClientError)?)
    }

    pub(crate) async fn write(&mut self, data: &[u8]) -> Result<(), TCPClientError> {
        Ok(self
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

#[maybe_async::maybe_async]
pub(super) trait Tcp {
    type Error: Context;

    const DEFAULT_PACKET_SIZE: u16 = 1024;

    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self, Self::Error>
    where Self: Sized;

    async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>, Self::Error>;
    async fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::TcpClient;
    use std::net::SocketAddr;

    #[cfg(feature = "async-std-client")]
    use async_std::{
        io::{ReadExt, WriteExt},
        net::TcpListener,
    };

    #[cfg(feature = "sync-std-client")]
    use std::{
        io::{Read, Write},
        net::TcpListener,
    };

    #[cfg(feature = "async-tokio-client")]
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    const MOCK_DATA: &[u8; 7] = b"Gamedig";

    #[maybe_async::maybe_async]
    async fn mock_tcp_server() -> (SocketAddr, TcpListener) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

        (listener.local_addr().unwrap(), listener)
    }

    #[maybe_async::test(
        feature = "sync-std-client",
        async(feature = "async-std-client", async_std::test),
        async(feature = "async-tokio-client", tokio::test)
    )]
    async fn test_client_new() {
        let (addr, _listener) = mock_tcp_server().await;
        let client = TcpClient::new(&addr, None).await;

        assert!(
            client.is_ok(),
            "Expected Ok(TcpClient), Received Err: {:?}",
            client.err()
        );
    }

    #[maybe_async::test(
        feature = "sync-std-client",
        async(feature = "async-std-client", async_std::test),
        async(feature = "async-tokio-client", tokio::test)
    )]
    async fn test_client_new_connection_failure() {
        let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        let client = TcpClient::new(&addr, None).await;

        assert!(
            client.is_err(),
            "Expected Err, Received Ok: {:?}",
            client.ok()
        );
    }

    #[maybe_async::test(
        feature = "sync-std-client",
        async(feature = "async-std-client", async_std::test),
        async(feature = "async-tokio-client", tokio::test)
    )]
    async fn test_client_read() {
        let (addr, listener) = mock_tcp_server().await;
        let mut client = TcpClient::new(&addr, None).await.unwrap();

        #[cfg(feature = "sync-std-client")]
        let handle = std::thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket.write_all(MOCK_DATA).unwrap();
        });

        #[cfg(feature = "async-std-client")]
        let handle = async_std::task::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            socket.write_all(MOCK_DATA).await.unwrap();
        });

        #[cfg(feature = "async-tokio-client")]
        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            socket.write_all(MOCK_DATA).await.unwrap();
        });

        let result = client.read(Some(7)).await;

        #[cfg(feature = "sync-std-client")]
        {
            handle.join().unwrap();
        }

        #[cfg(not(feature = "sync-std-client"))]
        {
            handle.await.unwrap();
        }

        assert!(
            result.is_ok(),
            "Expected Ok with data, Received Err: {:?}",
            result.err()
        );

        assert_eq!(result.unwrap(), MOCK_DATA);
    }

    #[maybe_async::test(
        feature = "sync-std-client",
        async(feature = "async-std-client", async_std::test),
        async(feature = "async-tokio-client", tokio::test)
    )]
    async fn test_client_write() {
        let (addr, listener) = mock_tcp_server().await;
        let mut client = TcpClient::new(&addr, None).await.unwrap();

        #[cfg(feature = "sync-std-client")]
        let handle = std::thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();

            let mut buf = vec![0; 7];
            socket.read_exact(&mut buf).unwrap();

            assert_eq!(buf, MOCK_DATA);
        });

        #[cfg(feature = "async-std-client")]
        let handle = async_std::task::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();

            let mut buf = vec![0; 7];
            socket.read_exact(&mut buf).await.unwrap();

            assert_eq!(buf, MOCK_DATA);
        });

        #[cfg(feature = "async-tokio-client")]
        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();

            let mut buf = vec![0; 7];
            socket.read_exact(&mut buf).await.unwrap();

            assert_eq!(buf, MOCK_DATA);
        });

        let result = client.write(MOCK_DATA).await;

        #[cfg(feature = "sync-std-client")]
        {
            handle.join().unwrap();
        }

        #[cfg(not(feature = "sync-std-client"))]
        {
            handle.await.unwrap();
        }

        assert!(
            result.is_ok(),
            "Expected Ok, Received Err: {:?}",
            result.err()
        );
    }
}
