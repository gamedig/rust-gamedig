use std::collections::HashMap;
use crate::GDResult;
use crate::protocols::valve;
use crate::protocols::valve::{Server, ServerPlayer, get_optional_extracted_data, SteamApp};

#[derive(Debug)]
pub struct TheShipPlayer {
    pub name: String,
    pub score: u32,
    pub duration: f32,
    pub deaths: u32,
    pub money: u32
}

impl TheShipPlayer {
    pub fn new_from_valve_player(player: &ServerPlayer) -> Self {
        Self {
            name: player.name.clone(),
            score: player.score,
            duration: player.duration,
            deaths: player.deaths.unwrap(),
            money: player.money.unwrap()
        }
    }
}

#[derive(Debug)]
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
    pub duration: u8
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
            players_details: response.players.unwrap().iter().map(TheShipPlayer::new_from_valve_player).collect(),
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
            duration: the_unwrapped_ship.duration
        }
    }
}

pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    let valve_response = valve::query(address, match port {
        None => 27015,
        Some(port) => port
    }, SteamApp::TS.as_engine(), None, None)?;

    Ok(Response::new_from_valve_response(valve_response))
}
