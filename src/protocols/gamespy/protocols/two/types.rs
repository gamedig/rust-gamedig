use std::collections::HashMap;

use crate::protocols::gamespy::VersionedExtraResponse;
use crate::protocols::types::SpecificResponse;
use crate::protocols::GenericResponse;
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
    pub teams: Vec<Team>,
    pub players_maximum: usize,
    pub players_online: usize,
    pub players_minimum: Option<u8>,
    pub players: Vec<Player>,
    pub unused_entries: HashMap<String, String>,
}

/// Non-generic query response
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraResponse {
    pub teams: Vec<Team>,
    pub players_minimum: Option<u8>,
    pub unused_entries: HashMap<String, String>,
    pub players: Vec<Player>,
}

impl From<Response> for GenericResponse {
    fn from(r: Response) -> Self {
        Self {
            name: Some(r.name),
            description: None,
            game: None,
            game_version: None,
            map: Some(r.map),
            players_maximum: r.players_maximum.try_into().unwrap(), // FIXME: usize to u64 may fail
            players_online: r.players_online.try_into().unwrap(),
            players_bots: None,
            has_password: Some(r.has_password),
            inner: SpecificResponse::Gamespy(VersionedExtraResponse::Two(ExtraResponse {
                teams: r.teams,
                players_minimum: r.players_minimum,
                unused_entries: r.unused_entries,
                players: r.players,
            })),
        }
    }
}
