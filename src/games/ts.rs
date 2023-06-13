use crate::{
    protocols::{
        types::SpecificResponse,
        valve::{self, get_optional_extracted_data, Server, ServerPlayer, SteamApp},
        GenericResponse,
    },
    GDResult,
};
use std::net::{IpAddr, SocketAddr};

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TheShipPlayer {
    pub name: String,
    pub score: u32,
    pub duration: f32,
    pub deaths: u32,
    pub money: u32,
}

impl TheShipPlayer {
    pub fn new_from_valve_player(player: &ServerPlayer) -> Self {
        Self {
            name: player.name.clone(),
            score: player.score,
            duration: player.duration,
            deaths: player.deaths.unwrap(),
            money: player.money.unwrap(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub protocol: u8,
    pub name: String,
    pub map: String,
    pub game: String,
    pub players: u8,
    pub players_details: Vec<TheShipPlayer>,
    pub max_players: u8,
    pub bots: u8,
    pub server_type: Server,
    pub has_password: bool,
    pub vac_secured: bool,
    pub version: String,
    pub port: Option<u16>,
    pub steam_id: Option<u64>,
    pub tv_port: Option<u16>,
    pub tv_name: Option<String>,
    pub keywords: Option<String>,
    pub rules: HashMap<String, String>,
    pub mode: u8,
    pub witnesses: u8,
    pub duration: u8,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ExtraResponse {
    pub protocol: u8,
    pub player_details: Vec<TheShipPlayer>,
    pub server_type: Server,
    pub vac_secured: bool,
    pub port: Option<u16>,
    pub steam_id: Option<u64>,
    pub tv_port: Option<u16>,
    pub tv_name: Option<String>,
    pub keywords: Option<String>,
    pub rules: HashMap<String, String>,
    pub mode: u8,
    pub witnesses: u8,
    pub duration: u8,
}

impl From<Response> for GenericResponse {
    fn from(r: Response) -> Self {
        Self {
            name: Some(r.name),
            description: None,
            game: Some(r.game),
            game_version: Some(r.version),
            map: Some(r.map),
            players_maximum: r.max_players.into(),
            players_online: r.players.into(),
            players_bots: Some(r.bots.into()),
            has_password: Some(r.has_password),
            inner: SpecificResponse::TheShip(ExtraResponse {
                protocol: r.protocol,
                player_details: r.players_details,
                server_type: r.server_type,
                vac_secured: r.vac_secured,
                steam_id: r.steam_id,
                port: r.port,
                tv_port: r.tv_port,
                tv_name: r.tv_name,
                keywords: r.keywords,
                rules: r.rules,
                mode: r.mode,
                witnesses: r.witnesses,
                duration: r.duration,
            }),
        }
    }
}

impl Response {
    pub fn new_from_valve_response(response: valve::Response) -> Self {
        let (port, steam_id, tv_port, tv_name, keywords) = get_optional_extracted_data(response.info.extra_data);

        let the_unwrapped_ship = response.info.the_ship.unwrap();

        Self {
            protocol: response.info.protocol,
            name: response.info.name,
            map: response.info.map,
            game: response.info.game,
            players: response.info.players_online,
            players_details: response
                .players
                .unwrap()
                .iter()
                .map(TheShipPlayer::new_from_valve_player)
                .collect(),
            max_players: response.info.players_maximum,
            bots: response.info.players_bots,
            server_type: response.info.server_type,
            has_password: response.info.has_password,
            vac_secured: response.info.vac_secured,
            version: response.info.version,
            port,
            steam_id,
            tv_port,
            tv_name,
            keywords,
            rules: response.rules.unwrap(),
            mode: the_unwrapped_ship.mode,
            witnesses: the_unwrapped_ship.witnesses,
            duration: the_unwrapped_ship.duration,
        }
    }
}

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> {
    let valve_response = valve::query(
        &SocketAddr::new(*address, port.unwrap_or(27015)),
        SteamApp::TS.as_engine(),
        None,
        None,
    )?;

    Ok(Response::new_from_valve_response(valve_response))
}
