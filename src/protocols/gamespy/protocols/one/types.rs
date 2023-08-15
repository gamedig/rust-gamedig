use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::protocols::gamespy::{VersionedPlayer, VersionedResponse};
use crate::protocols::types::{CommonPlayer, CommonResponse, GenericPlayer};
use crate::protocols::GenericResponse;

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
    pub score: u32,
    pub deaths: Option<u32>,
    pub health: Option<u32>,
    pub secret: bool,
}

impl CommonPlayer for Player {
    fn as_original(&self) -> GenericPlayer { GenericPlayer::Gamespy(VersionedPlayer::One(self)) }

    fn name(&self) -> &str { &self.name }
    fn score(&self) -> Option<u32> { Some(self.score) }
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
    pub game_mode: String,
    pub game_version: String,
    pub players_maximum: u32,
    pub players_online: u32,
    pub players_minimum: Option<u8>,
    pub players: Vec<Player>,
    pub tournament: bool,
    pub unused_entries: HashMap<String, String>,
}

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse { GenericResponse::GameSpy(VersionedResponse::One(self)) }

    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn map(&self) -> Option<&str> { Some(&self.map) }
    fn has_password(&self) -> Option<bool> { Some(self.has_password) }
    fn game_mode(&self) -> Option<&str> { Some(&self.game_mode) }
    fn game_version(&self) -> Option<&str> { Some(&self.game_version) }
    fn players_maximum(&self) -> u32 { self.players_maximum.into() }
    fn players_online(&self) -> u32 { self.players_online.into() }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        Some(
            self.players
                .iter()
                .map(|p| p as &dyn CommonPlayer)
                .collect(),
        )
    }
}
