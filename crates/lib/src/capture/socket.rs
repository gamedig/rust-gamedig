use std::{marker::PhantomData, net::SocketAddr};

use crate::{
    capture::{
        packet::CapturePacket,
        packet::{Direction, Protocol},
        writer::{Writer, CAPTURE_WRITER},
    },
    protocols::types::TimeoutSettings,
    socket::{Socket, TcpSocketImpl, UdpSocketImpl},
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
    fn protocol() -> Protocol { Protocol::Tcp }
}

/// Represents the UDP protocol provider.
pub(crate) struct ProtocolUDP;
impl ProtocolProvider for ProtocolUDP {
    fn protocol() -> Protocol { Protocol::Udp }
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
    /// Initializes a new socket of type `I`, wrapping it to enable packet
    /// capturing. Capturing is protocol-specific, as indicated by
    /// the `ProtocolProvider`.
    ///
    /// # Arguments
    /// * `address` - The address to connect the socket to.
    /// * `timeout_settings` - Optional timeout settings for the socket.
    ///
    /// # Returns
    /// A `GDResult` containing either the wrapped socket or an error.
    fn new(address: &SocketAddr, timeout_settings: &Option<TimeoutSettings>) -> GDResult<Self>
    where Self: Sized {
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
    /// The method sends data using the inner socket and captures the sent
    /// packet if a capture writer is set.
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
    /// The method receives data using the inner socket and captures the
    /// incoming packet if a capture writer is set.
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
    fn port(&self) -> u16 { self.inner.port() }

    /// Returns the local SocketAddr of the wrapped socket.
    ///
    /// Delegates the operation to the inner socket implementation.
    ///
    /// # Returns
    /// The local SocketAddr.
    fn local_addr(&self) -> std::io::Result<SocketAddr> { self.inner.local_addr() }
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

/// A specialized `WrappedCaptureSocket` for UDP, using `UdpSocketImpl` as
/// the inner socket and `ProtocolUDP` as the protocol provider.
///
/// This type captures and processes UDP packets, wrapping around standard
/// UDP socket functionalities with additional packet capture
/// capabilities.
pub(crate) type CapturedUdpSocket = WrappedCaptureSocket<UdpSocketImpl, ProtocolUDP>;

/// A specialized `WrappedCaptureSocket` for TCP, using `TcpSocketImpl` as
/// the inner socket and `ProtocolTCP` as the protocol provider.
///
/// This type captures and processes TCP packets, wrapping around standard
/// TCP socket functionalities with additional packet capture
/// capabilities.
pub(crate) type CapturedTcpSocket = WrappedCaptureSocket<TcpSocketImpl, ProtocolTCP>;
