use crate::protocols::types::{CommonPlayer, CommonResponse, GenericPlayer};
use crate::protocols::GenericResponse;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
    pub steam_id: String,
    pub ping: u16,
}

impl CommonPlayer for Player {
    fn as_original(&self) -> GenericPlayer<'_> { GenericPlayer::JCMP2(self) }

    fn name(&self) -> &str { &self.name }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Response {
    pub game_version: String,
    pub description: String,
    pub name: String,
    pub has_password: bool,
    pub players: Vec<Player>,
    pub players_maximum: u32,
    pub players_online: u32,
}

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse<'_> { GenericResponse::JC2M(self) }

    fn game_version(&self) -> Option<&str> { Some(&self.game_version) }
    fn description(&self) -> Option<&str> { Some(&self.description) }
    fn name(&self) -> Option<&str> { Some(&self.name) }
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
