use crate::{
    protocols::types::TimeoutSettings,
    GDErrorKind::{PacketReceive, PacketSend, SocketBind, SocketConnect},
    GDResult,
};

use std::{
    io::{Read, Write},
    net::{self, SocketAddr},
};

const DEFAULT_PACKET_SIZE: usize = 1024;

/// A trait defining the basic functionalities of a network socket.
pub trait Socket {
    /// Create a new socket and connect to the remote address.
    ///
    /// # Arguments
    /// * `address` - The address to connect the socket to.
    /// * `timeout_settings` - Optional timeout settings for the socket.
    ///
    /// # Returns
    /// A result containing the socket instance or an error.
    fn new(address: &SocketAddr, timeout_settings: &Option<TimeoutSettings>) -> GDResult<Self>
    where
        Self: Sized;

    /// Apply read and write timeouts to the socket.
    ///
    /// # Arguments
    /// * `timeout_settings` - Optional timeout settings to apply.
    ///
    /// # Returns
    /// A result indicating success or error in applying timeouts.
    fn apply_timeout(&self, timeout_settings: &Option<TimeoutSettings>) -> GDResult<()>;

    /// Send data over the socket.
    ///
    /// # Arguments
    /// * `data` - Data to be sent.
    ///
    /// # Returns
    /// A result indicating success or error in sending data.
    fn send(&mut self, data: &[u8]) -> GDResult<()>;

    /// Receive data from the socket.
    ///
    /// # Arguments
    /// * `size` - Optional size of data to receive.
    ///
    /// # Returns
    /// A result containing received data or an error.
    fn receive(&mut self, size: Option<usize>) -> GDResult<Vec<u8>>;

    /// Get the remote port of the socket.
    ///
    /// # Returns
    /// The port number.
    fn port(&self) -> u16;

    /// Get the local SocketAddr.
    ///
    /// # Returns
    /// The local SocketAddr.
    fn local_addr(&self) -> std::io::Result<SocketAddr>;
}

/// Implementation of a TCP socket.
pub struct TcpSocketImpl {
    /// The underlying TCP socket stream.
    socket: net::TcpStream,
    /// The address of the remote host.
    address: SocketAddr,
}

impl Socket for TcpSocketImpl {
    fn new(address: &SocketAddr, timeout_settings: &Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = if let Some(timeout) = TimeoutSettings::get_connect_or_default(timeout_settings) {
            net::TcpStream::connect_timeout(address, timeout)
        } else {
            net::TcpStream::connect(address)
        };

        let socket = Self {
            socket: socket.map_err(|e| SocketConnect.context(e))?,
            address: *address,
        };

        socket.apply_timeout(timeout_settings)?;

        Ok(socket)
    }

    fn apply_timeout(&self, timeout_settings: &Option<TimeoutSettings>) -> GDResult<()> {
        let (read, write) = TimeoutSettings::get_read_and_write_or_defaults(timeout_settings);
        self.socket.set_read_timeout(read).unwrap(); // unwrapping because TimeoutSettings::new
        self.socket.set_write_timeout(write).unwrap(); // checks if these are 0 and throws an error

        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> GDResult<()> {
        self.socket.write(data).map_err(|e| PacketSend.context(e))?;
        Ok(())
    }

    fn receive(&mut self, size: Option<usize>) -> GDResult<Vec<u8>> {
        let mut buf = Vec::with_capacity(size.unwrap_or(DEFAULT_PACKET_SIZE));
        self.socket
            .read_to_end(&mut buf)
            .map_err(|e| PacketReceive.context(e))?;

        Ok(buf)
    }

    fn port(&self) -> u16 {
        self.address.port()
    }
    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.socket.local_addr()
    }
}

/// Implementation of a UDP socket.
pub struct UdpSocketImpl {
    /// The underlying UDP socket.
    socket: net::UdpSocket,
    /// The address of the remote host.
    address: SocketAddr,
}

impl Socket for UdpSocketImpl {
    fn new(address: &SocketAddr, timeout_settings: &Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = net::UdpSocket::bind("0.0.0.0:0").map_err(|e| SocketBind.context(e))?;

        let socket = Self {
            socket,
            address: *address,
        };

        socket.apply_timeout(timeout_settings)?;

        Ok(socket)
    }

    fn apply_timeout(&self, timeout_settings: &Option<TimeoutSettings>) -> GDResult<()> {
        let (read, write) = TimeoutSettings::get_read_and_write_or_defaults(timeout_settings);
        self.socket.set_read_timeout(read).unwrap(); // unwrapping because TimeoutSettings::new
        self.socket.set_write_timeout(write).unwrap(); // checks if these are 0 and throws an error

        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> GDResult<()> {
        self.socket
            .send_to(data, self.address)
            .map_err(|e| PacketSend.context(e))?;

        Ok(())
    }

    fn receive(&mut self, size: Option<usize>) -> GDResult<Vec<u8>> {
        let mut buf: Vec<u8> = vec![0; size.unwrap_or(DEFAULT_PACKET_SIZE)];
        let (number_of_bytes_received, _) = self
            .socket
            .recv_from(&mut buf)
            .map_err(|e| PacketReceive.context(e))?;

        Ok(buf[..number_of_bytes_received].to_vec())
    }

    fn port(&self) -> u16 {
        self.address.port()
    }
    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.socket.local_addr()
    }
}

/// Things used for capturing packets.
#[cfg(feature = "packet_capture")]
pub mod capture {
    use std::{marker::PhantomData, net::SocketAddr};

    use super::{Socket, TcpSocketImpl, UdpSocketImpl};

    use crate::{
        capture::{
            packet::CapturePacket,
            packet::{Direction, Protocol},
            writer::{Writer, CAPTURE_WRITER},
        },
        protocols::types::TimeoutSettings,
        GDResult,
    };

    /// Sets a global capture writer for handling all packet data.
    ///
    /// # Panics
    /// Panics if a capture writer is already set.
    ///
    /// # Arguments
    /// * `writer` - A boxed writer that implements the `Writer` trait.
    pub(crate) fn set_writer(writer: Box<dyn Writer + Send + Sync>) {
        let mut lock = CAPTURE_WRITER.lock().unwrap();

        if lock.is_some() {
            panic!("Capture writer already set");
        }

        *lock = Some(writer);
    }

    /// A trait representing a provider of a network protocol.
    pub(crate) trait ProtocolProvider {
        /// Returns the protocol used by the provider.
        fn protocol() -> Protocol;
    }

    /// Represents the TCP protocol provider.
    pub(crate) struct ProtocolTCP;
    impl ProtocolProvider for ProtocolTCP {
        fn protocol() -> Protocol {
            Protocol::TCP
        }
    }

    /// Represents the UDP protocol provider.
    pub(crate) struct ProtocolUDP;
    impl ProtocolProvider for ProtocolUDP {
        fn protocol() -> Protocol {
            Protocol::UDP
        }
    }

    /// A socket wrapper that allows capturing packets.
    ///
    /// # Type parameters
    /// * `I` - The inner socket type.
    /// * `P` - The protocol provider.
    #[derive(Clone, Debug)]
    pub(crate) struct WrappedCaptureSocket<I: Socket, P: ProtocolProvider> {
        inner: I,
        remote_address: SocketAddr,
        _protocol: PhantomData<P>,
    }

    impl<I: Socket, P: ProtocolProvider> Socket for WrappedCaptureSocket<I, P> {
        /// Creates a new wrapped socket for capturing packets.
        ///
        /// Initializes a new socket of type `I`, wrapping it to enable packet capturing.
        /// Capturing is protocol-specific, as indicated by the `ProtocolProvider`.
        ///
        /// # Arguments
        /// * `address` - The address to connect the socket to.
        /// * `timeout_settings` - Optional timeout settings for the socket.
        ///
        /// # Returns
        /// A `GDResult` containing either the wrapped socket or an error.
        fn new(address: &SocketAddr, timeout_settings: &Option<TimeoutSettings>) -> GDResult<Self>
        where
            Self: Sized,
        {
            let v = Self {
                inner: I::new(address, timeout_settings)?,
                remote_address: *address,
                _protocol: PhantomData,
            };

            let info = CapturePacket {
                direction: Direction::Send,
                protocol: P::protocol(),
                remote_address: address,
                local_address: &v.local_addr().unwrap(),
            };

            if let Some(writer) = CAPTURE_WRITER.lock().unwrap().as_mut() {
                writer.new_connect(&info)?;
            }

            Ok(v)
        }

        /// Sends data over the socket and captures the packet.
        ///
        /// The method sends data using the inner socket and captures the sent packet
        /// if a capture writer is set.
        ///
        /// # Arguments
        /// * `data` - Data to be sent.
        ///
        /// # Returns
        /// A result indicating success or error in sending data.
        fn send(&mut self, data: &[u8]) -> GDResult<()> {
            let info = CapturePacket {
                direction: Direction::Send,
                protocol: P::protocol(),
                remote_address: &self.remote_address,
                local_address: &self.local_addr().unwrap(),
            };

            if let Some(writer) = CAPTURE_WRITER.lock().unwrap().as_mut() {
                writer.write(&info, data)?;
            }

            self.inner.send(data)
        }

        /// Receives data from the socket and captures the packet.
        ///
        /// The method receives data using the inner socket and captures the incoming packet
        /// if a capture writer is set.
        ///
        /// # Arguments
        /// * `size` - Optional size of data to receive.
        ///
        /// # Returns
        /// A result containing received data or an error.
        fn receive(&mut self, size: Option<usize>) -> crate::GDResult<Vec<u8>> {
            let data = self.inner.receive(size)?;
            let info = CapturePacket {
                direction: Direction::Receive,
                protocol: P::protocol(),
                remote_address: &self.remote_address,
                local_address: &self.local_addr().unwrap(),
            };

            if let Some(writer) = CAPTURE_WRITER.lock().unwrap().as_mut() {
                writer.write(&info, &data)?;
            }

            Ok(data)
        }

        /// Applies timeout settings to the wrapped socket.
        ///
        /// Delegates the operation to the inner socket implementation.
        ///
        /// # Arguments
        /// * `timeout_settings` - Optional timeout settings to apply.
        ///
        /// # Returns
        /// A result indicating success or error in applying timeouts.
        fn apply_timeout(
            &self,
            timeout_settings: &Option<crate::protocols::types::TimeoutSettings>,
        ) -> crate::GDResult<()> {
            self.inner.apply_timeout(timeout_settings)
        }

        /// Returns the remote port of the wrapped socket.
        ///
        /// Delegates the operation to the inner socket implementation.
        ///
        /// # Returns
        /// The remote port number.
        fn port(&self) -> u16 {
            self.inner.port()
        }

        /// Returns the local SocketAddr of the wrapped socket.
        ///
        /// Delegates the operation to the inner socket implementation.
        ///
        /// # Returns
        /// The local SocketAddr.
        fn local_addr(&self) -> std::io::Result<SocketAddr> {
            self.inner.local_addr()
        }
    }

    // this seems a bad way to do this, but its safe
    impl<I: Socket, P: ProtocolProvider> Drop for WrappedCaptureSocket<I, P> {
        fn drop(&mut self) {
            // Construct the CapturePacket info
            let info = CapturePacket {
                direction: Direction::Send,
                protocol: P::protocol(),
                remote_address: &self.remote_address,
                local_address: &self
                    .local_addr()
                    .unwrap_or_else(|_| SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), 0)),
            };

            // If a capture writer is set, close the connection and capture the packet.
            if let Some(writer) = CAPTURE_WRITER.lock().unwrap().as_mut() {
                let _ = writer.close_connection(&info);
            }
        }
    }

    /// A specialized `WrappedCaptureSocket` for UDP, using `UdpSocketImpl` as the inner socket
    /// and `ProtocolUDP` as the protocol provider.
    ///
    /// This type captures and processes UDP packets, wrapping around standard UDP socket
    /// functionalities with additional packet capture capabilities.
    pub(crate) type CapturedUdpSocket = WrappedCaptureSocket<UdpSocketImpl, ProtocolUDP>;

    /// A specialized `WrappedCaptureSocket` for TCP, using `TcpSocketImpl` as the inner socket
    /// and `ProtocolTCP` as the protocol provider.
    ///
    /// This type captures and processes TCP packets, wrapping around standard TCP socket
    /// functionalities with additional packet capture capabilities.
    pub(crate) type CapturedTcpSocket = WrappedCaptureSocket<TcpSocketImpl, ProtocolTCP>;
}

#[cfg(not(feature = "packet_capture"))]
pub type UdpSocket = UdpSocketImpl;
#[cfg(not(feature = "packet_capture"))]
pub type TcpSocket = TcpSocketImpl;

#[cfg(feature = "packet_capture")]
pub(crate) type UdpSocket = capture::CapturedUdpSocket;
#[cfg(feature = "packet_capture")]
pub(crate) type TcpSocket = capture::CapturedTcpSocket;

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_tcp_socket_send_and_receive() {
        // Spawn a thread to run the server
        let listener = net::TcpListener::bind("127.0.0.1:0").unwrap();
        let bound_address = listener.local_addr().unwrap();
        let server_thread = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0; 1024];
            let _ = stream.read(&mut buf).unwrap();
            let _ = stream.write(&buf).unwrap();
        });

        // Create a TCP socket and send a message to the server
        let mut socket = TcpSocket::new(&bound_address, &None).unwrap();
        let message = b"hello, world!";
        socket.send(message).unwrap();

        // Receive the response from the server
        let received_message: Vec<u8> = socket
            .receive(None)
            .unwrap()
            // Iterate over the buffer and remove 0s that are alone in the buffer
            // just added to pass default size
            .into_iter()
            .filter(|&x| x != 0)
            .collect();

        server_thread.join().expect("server thread panicked");

        assert_eq!(message, &received_message[..]);
    }

    #[test]
    fn test_udp_socket_send_and_receive() {
        // Spawn a thread to run the server
        let socket = net::UdpSocket::bind("127.0.0.1:0").unwrap();
        let bound_address = socket.local_addr().unwrap();
        let server_thread = thread::spawn(move || {
            let mut buf = [0; 1024];
            let (_, src_addr) = socket.recv_from(&mut buf).unwrap();
            socket.send_to(&buf, src_addr).unwrap();
        });

        // Create a UDP socket and send a message to the server
        let mut socket = UdpSocket::new(&bound_address, &None).unwrap();
        let message = b"hello, world!";
        socket.send(message).unwrap();

        // Receive the response from the server
        let received_message: Vec<u8> = socket
            .receive(None)
            .unwrap()
            // Iterate over the buffer and remove 0s that are alone in the buffer
            // just added to pass default size
            .into_iter()
            .filter(|&x| x != 0)
            .collect();

        server_thread.join().expect("server thread panicked");

        assert_eq!(message, &received_message[..]);
    }
}
