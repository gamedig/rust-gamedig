use crate::bufferer::{Bufferer, Endianess};
use crate::protocols::gamespy::common::has_password;
use crate::protocols::gamespy::three::{data_to_map, GameSpy3};
use crate::protocols::types::SpecificResponse;
use crate::protocols::GenericResponse;
use crate::{GDError, GDResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    name: String,
    steam_id: String,
    ping: u16,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Response {
    version: String,
    description: String,
    name: String,
    has_password: bool,
    players: Vec<Player>,
}

impl From<Response> for GenericResponse {
    fn from(r: Response) -> Self {
        Self {
            name: Some(r.name),
            description: Some(r.description),
            game: None,
            game_version: Some(r.version),
            map: None,
            players_maximum: 0, // todo: fuck!
            players_online: r.players.len() as u64,
            players_bots: None,
            has_password: Some(r.has_password),
            inner: SpecificResponse::JC2MP,
        }
    }
}

fn parse_players_and_teams(packet: Vec<u8>) -> GDResult<Vec<Player>> {
    let mut buf = Bufferer::new_with_data(Endianess::Big, &packet);

    let count = buf.get_u16()?;
    let mut players = Vec::with_capacity(count as usize);

    while buf.remaining_length() > 0 {
        players.push(Player {
            name: buf.get_string_utf8()?,
            steam_id: buf.get_string_utf8()?,
            ping: buf.get_u16()?,
        })
    }

    Ok(players)
}

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> {
    let mut client = GameSpy3::new_custom(
        &SocketAddr::new(*address, port.unwrap_or(7777)),
        None,
        [0xFF, 0xFF, 0xFF, 0x02],
        true,
    )?;

    let packets = client.get_server_packets()?;
    let data = packets.get(0).ok_or(GDError::PacketBad)?;

    let (mut server_vars, remaining_data) = data_to_map(data)?;

    let players = parse_players_and_teams(remaining_data)?;

    Ok(Response {
        version: server_vars.remove("version").ok_or(GDError::PacketBad)?,
        description: server_vars
            .remove("description")
            .ok_or(GDError::PacketBad)?,
        name: server_vars.remove("hostname").ok_or(GDError::PacketBad)?,
        has_password: has_password(&mut server_vars)?,
        players,
    })
}
