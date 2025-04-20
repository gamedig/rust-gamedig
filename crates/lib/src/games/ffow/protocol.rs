use crate::buffer::{Buffer, Utf8Decoder};
use crate::games::ffow::types::Response;
use crate::protocols::types::TimeoutSettings;
use crate::protocols::valve::{Engine, Environment, Server, ValveProtocol};
use crate::GDResult;
use byteorder::LittleEndian;
use std::net::{IpAddr, SocketAddr};

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, None) }

pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response> {
    let mut client = ValveProtocol::new(
        &SocketAddr::new(*address, port.unwrap_or(5478)),
        timeout_settings,
    )?;
    let data = client.get_request_data(
        &Engine::GoldSrc(true),
        0,
        0x46,
        String::from("LSQ").into_bytes(),
    )?;

    let mut buffer = Buffer::<LittleEndian>::new(&data);

    let protocol_version = buffer.read::<u8>()?;
    let name = buffer.read_string::<Utf8Decoder>(None)?;
    let map = buffer.read_string::<Utf8Decoder>(None)?;
    let active_mod = buffer.read_string::<Utf8Decoder>(None)?;
    let game_mode = buffer.read_string::<Utf8Decoder>(None)?;
    let description = buffer.read_string::<Utf8Decoder>(None)?;
    let game_version = buffer.read_string::<Utf8Decoder>(None)?;
    buffer.move_cursor(2)?;
    let players_online = buffer.read::<u8>()?;
    let players_maximum = buffer.read::<u8>()?;
    let server_type = Server::from_gldsrc(buffer.read::<u8>()?)?;
    let environment_type = Environment::from_gldsrc(buffer.read::<u8>()?)?;
    let has_password = buffer.read::<u8>()? == 1;
    let vac_secured = buffer.read::<u8>()? == 1;
    buffer.move_cursor(1)?; // average fps
    let round = buffer.read::<u8>()?;
    let rounds_maximum = buffer.read::<u8>()?;
    let time_left = buffer.read::<u16>()?;

    Ok(Response {
        protocol_version,
        name,
        active_mod,
        game_mode,
        game_version,
        description,
        map,
        players_online,
        players_maximum,
        server_type,
        environment_type,
        has_password,
        vac_secured,
        round,
        rounds_maximum,
        time_left,
    })
}
