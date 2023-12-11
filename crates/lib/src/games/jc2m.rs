use crate::buffer::{Buffer, Utf8Decoder};
use crate::protocols::gamespy::common::has_password;
use crate::protocols::gamespy::three::{data_to_map, GameSpy3};
use crate::protocols::types::{CommonPlayer, CommonResponse, GenericPlayer, TimeoutSettings};
use crate::protocols::GenericResponse;
use crate::GDErrorKind::{PacketBad, TypeParse};
use crate::{GDErrorKind, GDResult};
use byteorder::BigEndian;
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

impl CommonPlayer for Player {
    fn as_original(&self) -> GenericPlayer { GenericPlayer::JCMP2(self) }

    fn name(&self) -> &str { &self.name }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Response {
    game_version: String,
    description: String,
    name: String,
    has_password: bool,
    players: Vec<Player>,
    players_maximum: u32,
    players_online: u32,
}

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse { GenericResponse::JC2M(self) }

    fn game_version(&self) -> Option<&str> { Some(&self.game_version) }
    fn description(&self) -> Option<&str> { Some(&self.description) }
    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn has_password(&self) -> Option<bool> { Some(self.has_password) }
    fn players_maximum(&self) -> u32 { self.players_maximum }
    fn players_online(&self) -> u32 { self.players_online }

    fn players(&self) -> Option<Vec<&dyn crate::protocols::types::CommonPlayer>> {
        Some(
            self.players
                .iter()
                .map(|p| p as &dyn CommonPlayer)
                .collect(),
        )
    }
}

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
        .get(0)
        .ok_or_else(|| PacketBad.context("First packet missing"))?;

    let (mut server_vars, remaining_data) = data_to_map(data)?;
    let players = parse_players_and_teams(&remaining_data)?;

    let players_maximum = server_vars
        .remove("maxplayers")
        .ok_or(PacketBad.context("Server variables missing maxplayers"))?
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
        game_version: server_vars
            .remove("version")
            .ok_or(GDErrorKind::PacketBad)?,
        description: server_vars
            .remove("description")
            .ok_or(GDErrorKind::PacketBad)?,
        name: server_vars
            .remove("hostname")
            .ok_or(GDErrorKind::PacketBad)?,
        has_password: has_password(&mut server_vars)?,
        players,
        players_maximum,
        players_online,
    })
}
