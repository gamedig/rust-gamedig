use std::net::SocketAddr;

use crate::{
    buffer::{self, Buffer},
    socket::{Socket, UdpSocket},
    utils,
    GDResult,
    TimeoutSettings,
};

use super::types::ServerData;

/// Mindustry max datagram packet size.
pub const MAX_BUFFER_SIZE: usize = 500;

/// Send a ping packet.
///
/// [Reference](https://github.com/Anuken/Mindustry/blob/a2e5fbdedb2fc1c8d3c157bf344d10ad6d321442/core/src/mindustry/net/ArcNetProvider.java#L248)
pub(crate) fn send_ping(socket: &mut UdpSocket) -> GDResult<()> { socket.send(&[-2i8 as u8, 1i8 as u8]) }

/// Parse server data.
///
/// [Reference](https://github.com/Anuken/Mindustry/blob/a2e5fbdedb2fc1c8d3c157bf344d10ad6d321442/core/src/mindustry/net/NetworkIO.java#L122-L135)
pub fn parse_server_data<B: byteorder::ByteOrder, D: buffer::StringDecoder>(
    buffer: &mut Buffer<B>,
) -> GDResult<ServerData> {
    Ok(ServerData {
        host: buffer.read_string::<D>(None)?,
        map: buffer.read_string::<D>(None)?,
        players: buffer.read()?,
        wave: buffer.read()?,
        version: buffer.read()?,
        version_type: buffer.read_string::<D>(None)?,
        gamemode: buffer.read::<u8>()?.try_into()?,
        player_limit: buffer.read()?,
        description: buffer.read_string::<D>(None)?,
        mode_name: buffer.read_string::<D>(None).ok(),
    })
}

/// Query a Mindustry server (without retries).
pub fn query(address: &SocketAddr, timeout_settings: &Option<TimeoutSettings>) -> GDResult<ServerData> {
    let mut socket = UdpSocket::new(address, timeout_settings)?;

    send_ping(&mut socket)?;

    let socket_data = socket.receive(Some(MAX_BUFFER_SIZE))?;
    let mut buffer = Buffer::new(&socket_data);

    parse_server_data::<byteorder::BigEndian, buffer::Utf8LengthPrefixedDecoder>(&mut buffer)
}

/// Query a Mindustry server.
pub fn query_with_retries(address: &SocketAddr, timeout_settings: &Option<TimeoutSettings>) -> GDResult<ServerData> {
    let retries = TimeoutSettings::get_retries_or_default(timeout_settings);

    utils::retry_on_timeout(retries, || query(address, timeout_settings))
}
