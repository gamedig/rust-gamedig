use std::{
    fmt::{self, Display, Formatter},
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

use error_stack::{Context, Report, Result, ResultExt};

use crate::settings::Timeout;

#[derive(Debug)]
pub(super) struct SyncStdTcpClient {
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
        let mut buf = Vec::with_capacity(size.unwrap_or(Self::DEFAULT_PACKET_SIZE as usize));

        self.stream
            .read_to_end(&mut buf)
            .map_err(Report::from)
            .attach_printable("Failed to read data from the TCP stream")
            .change_context(SyncStdTcpClientError)?;

        Ok(buf)
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

#[cfg(test)]
mod tests {
    use crate::core::net::tcp::Tcp;

    use super::*;

    use std::{
        io::{Read, Write},
        net::{SocketAddr, TcpListener},
        thread,
        time::Duration,
    };

    fn create_mock_server() -> (SocketAddr, TcpListener) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        (addr, listener)
    }

    #[test]
    fn test_new_success() {
        let (addr, _listener) = create_mock_server();
        let timeout = Timeout::new(Some(Duration::from_secs(1)), None, None, None);

        let client = SyncStdTcpClient::new(&addr, &timeout);
        assert!(
            client.is_ok(),
            "Expected Ok(SyncStdTcpClient), got Err: {:?}",
            client.err()
        );
    }

    #[test]
    fn test_new_connection_failure() {
        let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        let timeout = Timeout::new(Some(Duration::from_secs(1)), None, None, None);

        let client = SyncStdTcpClient::new(&addr, &timeout);
        assert!(client.is_err(), "Expected Err, got Ok: {:?}", client.ok());
    }

    #[test]
    fn test_read_success() {
        let (addr, listener) = create_mock_server();
        let timeout = Timeout::new(
            Some(Duration::from_secs(1)),
            Some(Duration::from_secs(1)),
            None,
            None,
        );

        let mut client = SyncStdTcpClient::new(&addr, &timeout).unwrap();

        let handle = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket.write_all(b"hello").unwrap();
        });

        let result = client.read(Some(5));
        handle.join().unwrap();
        assert!(
            result.is_ok(),
            "Expected Ok with data, got Err: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), b"hello");
    }

    #[test]
    fn test_read_timeout() {
        let (addr, listener) = create_mock_server();
        let timeout = Timeout::new(
            Some(Duration::from_secs(1)),
            Some(Duration::from_millis(100)),
            None,
            None,
        );

        let mut client = SyncStdTcpClient::new(&addr, &timeout).unwrap();

        let handle = thread::spawn(move || {
            let (socket, _) = listener.accept().unwrap();
            // Simulate a delay in the server to cause a read timeout
            thread::sleep(Duration::from_secs(1));
            drop(socket);
        });

        let result = client.read(Some(5));
        handle.join().unwrap();
        assert!(
            result.is_err(),
            "Expected Err due to timeout, got Ok: {:?}",
            result.ok()
        );
    }

    #[test]
    fn test_write_success() {
        let (addr, listener) = create_mock_server();
        let timeout = Timeout::new(
            Some(Duration::from_secs(1)),
            None,
            Some(Duration::from_secs(1)),
            None,
        );

        let mut client = SyncStdTcpClient::new(&addr, &timeout).unwrap();

        let handle = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let mut buf = vec![0; 5];
            socket.read_exact(&mut buf).unwrap();
            assert_eq!(buf, b"hello");
        });

        let result = client.write(b"hello");
        handle.join().unwrap();
        assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result.err());
    }
}
