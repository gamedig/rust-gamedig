use crate::buffer::{Buffer, Utf8Decoder};
use crate::games::savage2::types::Response;
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::GDResult;
use byteorder::LittleEndian;
use std::net::{IpAddr, SocketAddr};

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, None) }

pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response> {
    let addr = &SocketAddr::new(*address, port.unwrap_or(11235));
    let mut socket = UdpSocket::new(addr, &timeout_settings)?;
    socket.send(&[0x01])?;
    let data = socket.receive(None)?;
    let mut buffer = Buffer::<LittleEndian>::new(&data);

    buffer.move_cursor(12)?;

    Ok(Response {
        name: buffer.read_string::<Utf8Decoder>(None)?,
        players_online: buffer.read::<u8>()?,
        players_maximum: buffer.read::<u8>()?,
        time: buffer.read_string::<Utf8Decoder>(None)?,
        map: buffer.read_string::<Utf8Decoder>(None)?,
        next_map: buffer.read_string::<Utf8Decoder>(None)?,
        location: buffer.read_string::<Utf8Decoder>(None)?,
        players_minimum: buffer.read::<u8>()?,
        game_mode: buffer.read_string::<Utf8Decoder>(None)?,
        protocol_version: buffer.read_string::<Utf8Decoder>(None)?,
        level_minimum: buffer.read::<u8>()?,
    })
}
