use crate::buffer::{Buffer, Utf8Decoder};
use crate::jc2m::{Player, Response};
use crate::protocols::gamespy::common::has_password;
use crate::protocols::gamespy::three::{data_to_map, GameSpy3};
use crate::protocols::types::TimeoutSettings;
use crate::GDErrorKind::{PacketBad, TypeParse};
use crate::GDResult;
use byteorder::BigEndian;
use std::net::{IpAddr, SocketAddr};

fn parse_players_and_teams(packet: &[u8]) -> GDResult<Vec<Player>> {
    let mut buf = Buffer::<BigEndian>::new(packet);

    let count = buf.read::<u16>()?;
    let mut players = Vec::with_capacity(count as usize);

    while buf.remaining_length() != 0 {
        players.push(Player {
            name: buf.read_string::<Utf8Decoder>(None)?,
            steam_id: buf.read_string::<Utf8Decoder>(None)?,
            ping: buf.read::<u16>()?,
        });
    }

    Ok(players)
}

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, None) }

pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response> {
    let mut client = GameSpy3::new_custom(
        &SocketAddr::new(*address, port.unwrap_or(7777)),
        timeout_settings,
        [0xFF, 0xFF, 0xFF, 0x02],
        true,
    )?;

    let packets = client.get_server_packets()?;
    let data = packets
        .first()
        .ok_or_else(|| PacketBad.context("First packet missing"))?;

    let (mut server_vars, remaining_data) = data_to_map(data)?;
    let players = parse_players_and_teams(&remaining_data)?;

    let players_maximum = server_vars
        .remove("maxplayers")
        .ok_or_else(|| PacketBad.context("Server variables missing maxplayers"))?
        .parse()
        .map_err(|e| TypeParse.context(e))?;
    let players_online = match server_vars.remove("numplayers") {
        None => players.len(),
        Some(v) => {
            let reported_players = v.parse().map_err(|e| TypeParse.context(e))?;
            match reported_players < players.len() {
                true => players.len(),
                false => reported_players,
            }
        }
    } as u32;

    Ok(Response {
        game_version: server_vars.remove("version").ok_or(PacketBad)?,
        description: server_vars.remove("description").ok_or(PacketBad)?,
        name: server_vars.remove("hostname").ok_or(PacketBad)?,
        has_password: has_password(&mut server_vars)?,
        players,
        players_maximum,
        players_online,
    })
}
