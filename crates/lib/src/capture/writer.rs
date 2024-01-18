use std::{io::Write, sync::Mutex};

use super::{
    packet::{CapturePacket, Protocol},
    pcap::Pcap,
};
use crate::GDResult;
use lazy_static::lazy_static;

lazy_static! {
    /// A globally accessible, lazily-initialized static writer instance.
    /// This writer is intended for capturing and recording network packets.
    /// The writer is wrapped in a Mutex to ensure thread-safe access and modification.
    pub(crate) static ref CAPTURE_WRITER: Mutex<Option<Box<dyn Writer + Send + Sync>>> = Mutex::new(None);
}

/// Trait defining the functionality for a writer that handles network packet
/// captures. This trait includes methods for writing packet data, handling new
/// connections, and closing connections.
pub(crate) trait Writer {
    /// Writes a given packet's data to an underlying storage or stream.
    ///
    /// # Arguments
    /// * `packet` - Reference to the packet being captured.
    /// * `data` - The raw byte data associated with the packet.
    ///
    /// # Returns
    /// A `GDResult` indicating the success or failure of the write operation.
    fn write(&mut self, packet: &CapturePacket, data: &[u8]) -> GDResult<()>;

    /// Handles the creation of a new connection, potentially logging or
    /// initializing resources.
    ///
    /// # Arguments
    /// * `packet` - Reference to the packet indicating a new connection.
    ///
    /// # Returns
    /// A `GDResult` indicating the success or failure of handling the new
    /// connection.
    fn new_connect(&mut self, packet: &CapturePacket) -> GDResult<()>;

    /// Closes a connection, handling any necessary cleanup or finalization.
    ///
    /// # Arguments
    /// * `packet` - Reference to the packet indicating the closure of a
    ///   connection.
    ///
    /// # Returns
    /// A `GDResult` indicating the success or failure of the connection closure
    /// operation.
    fn close_connection(&mut self, packet: &CapturePacket) -> GDResult<()>;
}

/// Implementation of the `Writer` trait for the `Pcap` struct.
/// This implementation enables writing, connection handling, and closure
/// specific to PCAP (Packet Capture) format.
impl<W: Write> Writer for Pcap<W> {
    fn write(&mut self, info: &CapturePacket, data: &[u8]) -> GDResult<()> {
        self.write_transport_packet(info, data);

        Ok(())
    }

    fn new_connect(&mut self, packet: &CapturePacket) -> GDResult<()> {
        match packet.protocol {
            Protocol::Tcp => {
                self.write_tcp_handshake(packet);
            }
            Protocol::Udp => {}
        }

        self.state.stream_count = self.state.stream_count.wrapping_add(1);

        Ok(())
    }

    fn close_connection(&mut self, packet: &CapturePacket) -> GDResult<()> {
        match packet.protocol {
            Protocol::Tcp => {
                self.send_tcp_fin(packet);
            }
            Protocol::Udp => {}
        }
        Ok(())
    }
}
