use std::collections::HashMap;

use crate::protocols::gamespy::{VersionedPlayer, VersionedResponse};
use crate::protocols::types::{CommonPlayer, CommonResponse, GenericPlayer};
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

impl CommonPlayer for Player {
    fn as_original(&self) -> GenericPlayer { GenericPlayer::Gamespy(VersionedPlayer::Two(self)) }

    fn name(&self) -> &str { &self.name }
    fn score(&self) -> Option<i32> { Some(self.score.into()) }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub has_password: bool,
    pub teams: Vec<Team>,
    pub players_maximum: u32,
    pub players_online: u32,
    pub players_minimum: Option<u32>,
    pub players: Vec<Player>,
    pub unused_entries: HashMap<String, String>,
}

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse { GenericResponse::GameSpy(VersionedResponse::Two(self)) }

    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn map(&self) -> Option<&str> { Some(&self.map) }
    fn has_password(&self) -> Option<bool> { Some(self.has_password) }
    fn players_maximum(&self) -> u32 { self.players_maximum }
    fn players_online(&self) -> u32 { self.players_online }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        Some(
            self.players
                .iter()
                .map(|p| p as &dyn CommonPlayer)
                .collect(),
        )
    }
}
