use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


use crate::protocols::gamespy::VersionedExtraResponse;
use crate::protocols::{GenericResponse, types::SpecificResponse};

/// A player’s details.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
    pub team: u8,
    /// The ping from the server's perspective.
    pub ping: u16,
    pub face: String,
    pub skin: String,
    pub mesh: String,
    pub frags: u32,
    pub deaths: Option<u32>,
    pub health: Option<u32>,
    pub secret: bool,
}

/// A query response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub map_title: Option<String>,
    pub admin_contact: Option<String>,
    pub admin_name: Option<String>,
    pub has_password: bool,
    pub game_type: String,
    pub game_version: String,
    pub players_maximum: usize,
    pub players_online: usize,
    pub players_minimum: Option<u8>,
    pub players: Vec<Player>,
    pub tournament: bool,
    pub unused_entries: HashMap<String, String>,
}

/// Non-generic query response
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraResponse {
    pub map_title: Option<String>,
    pub admin_contact: Option<String>,
    pub admin_name: Option<String>,
    pub players_minimum: Option<u8>,
    pub tournament: bool,
    pub unused_entries: HashMap<String, String>,
}

impl From<Response> for GenericResponse {
    fn from(r: Response) -> Self {
        Self {
            name: Some(r.name),
            description: None,
            game: Some(r.game_type),
            game_version: Some(r.game_version),
            map: Some(r.map),
            players_maximum: r.players_maximum.try_into().unwrap(), // FIXME: usize to u64 may fail
            players_online: r.players_online.try_into().unwrap(),
            players_bots: None,
            has_password: Some(r.has_password),
            inner: SpecificResponse::Gamespy(VersionedExtraResponse::One(ExtraResponse{
                map_title: r.map_title,
                admin_contact: r.admin_contact,
                admin_name: r.admin_name,
                players_minimum: r.players_minimum,
                tournament: r.tournament,
                unused_entries: r.unused_entries,
            })),
        }
    }
}
