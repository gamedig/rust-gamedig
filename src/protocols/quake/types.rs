#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::protocols::GenericResponse;

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

/// Versioned response type
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionedResponse {
    One(Response<crate::protocols::quake::one::Player>),
    TwoAndThree(Response<crate::protocols::quake::two::Player>),
}

impl From<VersionedResponse> for GenericResponse {
    fn from(r: VersionedResponse) -> Self { GenericResponse::Quake(r) }
}
