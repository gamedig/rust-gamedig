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
    pub players_maximum: usize,
    pub players_online: usize,
    pub players: Vec<Player>,
    pub unused_entries: HashMap<String, String>
}
