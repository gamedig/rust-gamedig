use std::collections::HashMap;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub team: u8,
    pub ping: u16,
    pub face: String,
    pub skin: String,
    pub mesh: String,
    pub frags: u32,
    pub deaths: Option<u32>,
    pub health: Option<u32>,
    pub secret: bool
}

/// A query response.
#[derive(Debug)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub admin_contact: String,
    pub admin_name: String,
    pub has_password: bool,
    pub game_type: String,
    pub game_version: String,
    pub balance_teams: bool,
    pub players_maximum: usize,
    pub players_online: usize,
    pub players_minimum: u8,
    pub max_teams: u8,
    pub map_title: String,
    pub players: Vec<Player>,
    pub tournament: bool,
    pub unused_entries: HashMap<String, String>
}
