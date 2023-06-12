use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Team {
    pub name: String,
    pub score: u16,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
    pub score: u16,
    pub ping: u16,
    pub team_index: u16,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub has_password: bool,
    pub max_players: u8,
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
    pub unused_entries: HashMap<String, String>,
}
