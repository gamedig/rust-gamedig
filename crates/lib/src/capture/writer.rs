use std::{io::Write, sync::Mutex};

use super::{
    packet::{CapturePacket, Protocol},
    pcap::Pcap,
};
use crate::GDResult;
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref CAPTURE_WRITER: Mutex<Option<Box<dyn Writer + Send + Sync>>> = Mutex::new(None);
}

pub(crate) trait Writer {
    fn write(&mut self, packet: &CapturePacket, data: &[u8]) -> GDResult<()>;
    fn new_connect(&mut self, packet: &CapturePacket) -> GDResult<()>;
    //TODO: Close connection
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
