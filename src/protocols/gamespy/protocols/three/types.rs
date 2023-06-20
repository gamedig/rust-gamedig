use crate::protocols::gamespy::{VersionedPlayer, VersionedResponse};
use crate::protocols::types::{CommonPlayer, CommonResponse, GenericPlayer};
use crate::protocols::GenericResponse;
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

impl CommonPlayer for Player {
    fn as_original(&self) -> crate::protocols::types::GenericPlayer {
        GenericPlayer::Gamespy(VersionedPlayer::Three(self))
    }

    fn name(&self) -> &str { &self.name }
    fn score(&self) -> Option<u32> { Some(self.score.try_into().unwrap_or(0)) }
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

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse { GenericResponse::GameSpy(VersionedResponse::Three(self)) }

    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn map(&self) -> Option<&str> { Some(&self.map) }
    fn has_password(&self) -> Option<bool> { Some(self.has_password) }
    fn game(&self) -> Option<&str> { Some(&self.game_type) }
    fn game_version(&self) -> Option<&str> { Some(&self.game_version) }
    fn players_maximum(&self) -> u64 { self.players_maximum.try_into().unwrap_or(0) }
    fn players_online(&self) -> u64 { self.players_online.try_into().unwrap_or(0) }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        Some(
            self.players
                .iter()
                .map(|p| p as &dyn CommonPlayer)
                .collect(),
        )
    }
}
