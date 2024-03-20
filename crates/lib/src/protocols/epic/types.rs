use crate::protocols::types::{CommonPlayer, CommonResponse, GenericPlayer};
use crate::protocols::GenericResponse;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub has_password: bool,
    pub players_online: u32,
    pub players_maxmimum: u32,
    pub players: Vec<Player>,
    pub game_version: Option<String>,
    pub raw: Value,
}

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse { GenericResponse::Epic(self) }
    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn map(&self) -> Option<&str> { Some(&self.map) }
    fn players_maximum(&self) -> u32 { self.players_maxmimum }

    fn players_online(&self) -> u32 { self.players_online }

    fn has_password(&self) -> Option<bool> { Some(self.has_password) }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        Some(
            self.players
                .iter()
                .map(|p| p as &dyn CommonPlayer)
                .collect(),
        )
    }

    fn game_version(&self) -> Option<&str> { self.game_version.as_deref() }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
}

impl CommonPlayer for Player {
    fn as_original(&self) -> GenericPlayer { GenericPlayer::Epic(self) }

    fn name(&self) -> &str { &self.name }
}
