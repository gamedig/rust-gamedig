#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::protocols::{types::SpecificResponse, GenericResponse};

/// General server information's.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response<P> {
    /// Name of the server.
    pub name: String,
    /// Map name.
    pub map: String,
    /// Current online players.
    pub players: Vec<P>,
    /// Number of players on the server.
    pub players_online: u8,
    /// Maximum number of players the server reports it can hold.
    pub players_maximum: u8,
    /// The server version.
    pub version: String,
    /// Other server entries that weren't used.
    pub unused_entries: HashMap<String, String>,
}

/// Non-generic quake response
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraResponse {
    /// Other server entries that weren't used.
    pub unused_entries: HashMap<String, String>,
}

impl<T> From<Response<T>> for GenericResponse {
    fn from(r: Response<T>) -> Self {
        Self {
            name: Some(r.name),
            description: None,
            game: None,
            game_version: Some(r.version),
            map: Some(r.map),
            players_maximum: r.players_maximum.into(),
            players_online: r.players_online.into(),
            players_bots: None,
            has_password: None,
            inner: SpecificResponse::Quake(ExtraResponse {
                // TODO: Players
                unused_entries: r.unused_entries,
            }),
        }
    }
}
