use serde::{Deserialize, Serialize};
use crate::protocols::GenericResponse;
use crate::protocols::types::CommonResponse;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub name: String,
    pub players_online: u8,
    pub players_maximum: u8,
    pub players_minimum: u8,
    pub time: String,
    pub map: String,
    pub next_map: String,
    pub location: String,
    pub game_mode: String,
    pub protocol_version: String,
    pub level_minimum: u8,
}

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse { GenericResponse::Savage2(self) }

    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn game_mode(&self) -> Option<&str> { Some(&self.game_mode) }
    fn map(&self) -> Option<&str> { Some(&self.map) }
    fn players_maximum(&self) -> u32 { self.players_maximum.into() }
    fn players_online(&self) -> u32 { self.players_online.into() }
}
