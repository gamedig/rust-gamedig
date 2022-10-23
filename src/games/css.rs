use crate::{GDResult, valve};
use crate::valve::{ValveProtocol, App, Server, ServerRule, ServerPlayer};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub duration: f32
}

impl Player {
    fn from_valve_response(player: &ServerPlayer) -> Self {
        Self {
            name: player.name.clone(),
            score: player.score,
            duration: player.duration
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
    pub players_details: Vec<Player>,
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
    pub rules: Vec<ServerRule>
}

impl Response {
    pub fn new_from_valve_response(response: valve::Response) -> Self {
        let (port, steam_id, tv_port, tv_name, keywords) = match response.info.extra_data {
            None => (None, None, None, None, None),
            Some(ed) => (ed.port, ed.steam_id, ed.tv_port, ed.tv_name, ed.keywords)
        };

        Self {
            protocol: response.info.protocol,
            name: response.info.name,
            map: response.info.map,
            game: response.info.game,
            players: response.info.players,
            players_details: response.players.unwrap().iter().map(|p| Player::from_valve_response(p)).collect(),
            max_players: response.info.max_players,
            bots: response.info.bots,
            server_type: response.info.server_type,
            has_password: response.info.has_password,
            vac_secured: response.info.vac_secured,
            version: response.info.version,
            port,
            steam_id,
            tv_port,
            tv_name,
            keywords,
            rules: response.rules.unwrap()
        }
    }
}

pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    let valve_response = ValveProtocol::query(address, match port {
        None => 27015,
        Some(port) => port
    }, Some(App::CSS), None)?;

    Ok(Response::new_from_valve_response(valve_response))
}
