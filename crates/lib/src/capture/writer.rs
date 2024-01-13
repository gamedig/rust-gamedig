use std::io::Write;

use crate::{
    capture::packet::{CapturePacket, Protocol},
    GDResult,
};

use super::pcap::Pcap;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub(crate) static ref CAPTURE_WRITER: Mutex<Option<Box<dyn Writer + Send + Sync>>> = Mutex::new(None);
}

pub(crate) trait Writer {
    fn write(&mut self, packet: &CapturePacket, data: &[u8]) -> crate::GDResult<()>;
    fn new_connect(&mut self, packet: &CapturePacket) -> crate::GDResult<()>;
}

impl<W: Write> Writer for Pcap<W> {
    fn write(&mut self, info: &CapturePacket, data: &[u8]) -> GDResult<()> {
        self.write_transport_packet(info, data);

        Ok(())
    }

    fn new_connect(&mut self, packet: &CapturePacket) -> GDResult<()> {
        match packet.protocol {
            Protocol::TCP => {
                self.write_tcp_handshake(packet);
            }
            Protocol::UDP => {}
        }

        self.state.stream_count = self.state.stream_count.wrapping_add(1);

        Ok(())
    }
}
