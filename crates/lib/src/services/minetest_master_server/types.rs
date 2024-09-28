use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Server {
    pub address: String,
    pub clients: u32,
    pub clients_list: Vec<String>,
    pub clients_max: u32,
    pub creative: bool,
    pub damage: bool,
    pub description: String,
    pub game_time: u32,
    pub gameid: String,
    pub lag: Option<f32>,
    pub name: String,
    pub password: bool,
    pub port: u16,
    pub proto_max: u16,
    pub proto_min: u16,
    pub pvp: bool,
    pub uptime: u32,
    pub url: Option<String>,
    pub version: String,
    pub ip: String,
    pub update_time: u32,
    pub start: u32,
    pub clients_top: u32,
    pub updates: u32,
    pub total_clients: u32,
    pub pop_v: f32,
    pub geo_continent: Option<String>,
    pub ping: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServersClients {
    pub servers: u32,
    pub clients: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub total: ServersClients,
    pub total_max: ServersClients,
    pub list: Vec<Server>,
}
