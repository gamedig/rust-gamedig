#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::protocols::{
    types::{CommonPlayer, CommonResponse},
    GenericResponse,
};

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

pub trait QuakePlayerType: Sized + CommonPlayer {
    fn version(response: &Response<Self>) -> VersionedResponse;
}

impl<P: QuakePlayerType> CommonResponse for Response<P> {
    fn as_original(&self) -> GenericResponse { GenericResponse::Quake(P::version(self)) }

    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn map(&self) -> Option<&str> { Some(&self.map) }
    fn players_maximum(&self) -> u64 { self.players_maximum.into() }
    fn players_online(&self) -> u64 { self.players_online.into() }
    fn game_version(&self) -> Option<&str> { Some(&self.version) }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        Some(
            self.players
                .iter()
                .map(|p| p as &dyn CommonPlayer)
                .collect(),
        )
    }
}

/// Versioned response type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionedResponse<'a> {
    One(&'a Response<crate::protocols::quake::one::Player>),
    TwoAndThree(&'a Response<crate::protocols::quake::two::Player>),
}
