use crate::{
    protocols::{
        types::{CommonPlayer, CommonResponse, GenericPlayer, TimeoutSettings},
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

impl CommonPlayer for TheShipPlayer {
    fn as_original(&self) -> GenericPlayer { GenericPlayer::TheShip(self) }

    fn name(&self) -> &str { &self.name }
    fn score(&self) -> Option<u32> { Some(self.score) }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub protocol_version: u8,
    pub name: String,
    pub map: String,
    pub game: String,
    pub game_version: String,
    pub players: Vec<TheShipPlayer>,
    pub players_online: u8,
    pub players_maximum: u8,
    pub players_bots: u8,
    pub server_type: Server,
    pub has_password: bool,
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

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse { GenericResponse::TheShip(self) }

    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn map(&self) -> Option<&str> { Some(&self.map) }
    fn game(&self) -> Option<&str> { Some(&self.game) }
    fn players_maximum(&self) -> u64 { self.players_maximum.into() }
    fn players_online(&self) -> u64 { self.players_online.into() }
    fn players_bots(&self) -> Option<u64> { Some(self.players_bots.into()) }
    fn has_password(&self) -> Option<bool> { Some(self.has_password) }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        Some(
            self.players
                .iter()
                .map(|p| p as &dyn CommonPlayer)
                .collect(),
        )
    }
}

impl Response {
    pub fn new_from_valve_response(response: valve::Response) -> Self {
        let (port, steam_id, tv_port, tv_name, keywords) = get_optional_extracted_data(response.info.extra_data);

        let the_unwrapped_ship = response.info.the_ship.unwrap();

        Self {
            protocol_version: response.info.protocol_version,
            name: response.info.name,
            map: response.info.map,
            game: response.info.game,
            game_version: response.info.game_version,
            players_online: response.info.players_online,
            players: response
                .players
                .unwrap()
                .iter()
                .map(TheShipPlayer::new_from_valve_player)
                .collect(),
            players_maximum: response.info.players_maximum,
            players_bots: response.info.players_bots,
            server_type: response.info.server_type,
            has_password: response.info.has_password,
            vac_secured: response.info.vac_secured,
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

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, None) }

pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response> {
    let valve_response = valve::query(
        &SocketAddr::new(*address, port.unwrap_or(27015)),
        SteamApp::TS.as_engine(),
        None,
        timeout_settings,
    )?;

    Ok(Response::new_from_valve_response(valve_response))
}
