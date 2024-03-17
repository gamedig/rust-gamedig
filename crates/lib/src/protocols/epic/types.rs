use serde_json::Value;

#[derive(Debug)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub has_password: bool,
    pub players_online: u32,
    pub players_maxmimum: u32,
    pub players: Vec<Player>,
    pub raw: Value,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
}
