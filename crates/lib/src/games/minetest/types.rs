use crate::minetest_master_server::Server;
use crate::protocols::types::{CommonPlayer, CommonResponse, GenericPlayer};
use crate::protocols::GenericResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
}

impl CommonPlayer for Player {
    fn as_original(&self) -> GenericPlayer { GenericPlayer::Minetest(self) }

    fn name(&self) -> &str { &self.name }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    pub name: String,
    pub description: String,
    pub game_version: String,
    pub players_maximum: u32,
    pub players_online: u32,
    pub has_password: Option<bool>,
    pub players: Vec<Player>,
    pub address: String,
    pub creative: Option<bool>,
    pub damage: bool,
    pub game_time: u32,
    pub gameid: String,
    pub lag: Option<f32>,
    pub port: u16,
    pub proto_max: u16,
    pub proto_min: u16,
    pub pvp: bool,
    pub uptime: u32,
    pub url: Option<String>,
    pub ip: String,
    pub update_time: u32,
    pub start: u32,
    pub clients_top: u32,
    pub updates: u32,
    pub pop_v: f32,
    pub geo_continent: Option<String>,
    pub ping: f32,
}

impl From<Server> for Response {
    fn from(server: Server) -> Self {
        Self {
            name: server.name,
            description: server.description,
            game_version: server.version,
            players_maximum: server.clients_max,
            players_online: server.total_clients,
            has_password: server.password,
            players: server
                .clients_list
                .unwrap_or_default()
                .into_iter()
                .map(|name| Player { name })
                .collect(),
            address: server.address,
            creative: server.creative,
            damage: server.damage,
            game_time: server.game_time,
            gameid: server.gameid,
            lag: server.lag,
            port: server.port,
            proto_max: server.proto_max,
            proto_min: server.proto_min,
            pvp: server.pvp,
            uptime: server.uptime,
            url: server.url,
            ip: server.ip,
            update_time: server.update_time,
            start: server.start,
            clients_top: server.clients_top,
            updates: server.updates,
            pop_v: server.pop_v,
            geo_continent: server.geo_continent,
            ping: server.ping,
        }
    }
}

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse { GenericResponse::Minetest(self) }

    fn name(&self) -> Option<&str> { Some(&self.name) }

    fn description(&self) -> Option<&str> { Some(&self.description) }

    fn game_version(&self) -> Option<&str> { Some(&self.game_version) }

    fn players_maximum(&self) -> u32 { self.players_maximum }

    fn players_online(&self) -> u32 { self.players_online }

    fn has_password(&self) -> Option<bool> { self.has_password }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        Some(
            self.players
                .iter()
                .map(|p| p as &dyn CommonPlayer)
                .collect(),
        )
    }
}
