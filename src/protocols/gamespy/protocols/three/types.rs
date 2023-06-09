use crate::protocols::{gamespy::ResponseVersion, GenericResponse};
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A player’s details.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
    pub score: i32,
    pub ping: u16,
    pub team: u8,
    pub deaths: u32,
    pub skill: u32,
}

/// A team's details
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Team {
    pub name: String,
    pub score: i32,
}

/// A query response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub has_password: bool,
    pub game_type: String,
    pub game_version: String,
    pub players_maximum: usize,
    pub players_online: usize,
    pub players_minimum: Option<u8>,
    pub players: Vec<Player>,
    pub teams: Vec<Team>,
    pub tournament: bool,
    pub unused_entries: HashMap<String, String>,
}

impl From<Response> for GenericResponse {
    fn from(r: Response) -> Self {
        let clone = r.clone();
        Self {
            server_name: Some(r.name),
            server_description: None,
            server_game: Some(r.game_type),
            server_game_version: Some(r.game_version),
            server_map: Some(r.map),
            players_maximum: Some(r.players_maximum.try_into().unwrap()), // FIXME: usize to u64 may fail
            players_online: Some(r.players_online.try_into().unwrap()),
            players_bots: None,
            has_password: Some(r.has_password),
            inner: crate::protocols::SpecificResponse::Gamespy(ResponseVersion::Three(clone)),
        }
    }
}
