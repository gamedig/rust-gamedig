use crate::protocols::types::CommonResponse;
use crate::protocols::valve::{Environment, Server};
use crate::protocols::GenericResponse;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The query response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Response {
    /// Protocol used by the server.
    pub protocol_version: u8,
    /// Name of the server.
    pub name: String,
    /// Map name.
    pub active_mod: String,
    /// Running game mode.
    pub game_mode: String,
    /// The version that the server is running on.
    pub game_version: String,
    /// Description of the server.
    pub description: String,
    /// Current map.
    pub map: String,
    /// Number of players on the server.
    pub players_online: u8,
    /// Maximum number of players the server reports it can hold.
    pub players_maximum: u8,
    /// Dedicated, NonDedicated or SourceTV
    pub server_type: Server,
    /// The Operating System that the server is on.
    pub environment_type: Environment,
    /// Indicates whether the server requires a password.
    pub has_password: bool,
    /// Indicates whether the server uses VAC.
    pub vac_secured: bool,
    /// Current round index.
    pub round: u8,
    /// Maximum amount of rounds.
    pub rounds_maximum: u8,
    /// Time left for the current round in seconds.
    pub time_left: u16,
}

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse<'_> { GenericResponse::FFOW(self) }

    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn game_mode(&self) -> Option<&str> { Some(&self.game_mode) }
    fn description(&self) -> Option<&str> { Some(&self.description) }
    fn game_version(&self) -> Option<&str> { Some(&self.game_version) }
    fn map(&self) -> Option<&str> { Some(&self.map) }
    fn has_password(&self) -> Option<bool> { Some(self.has_password) }
    fn players_maximum(&self) -> u32 { self.players_maximum.into() }
    fn players_online(&self) -> u32 { self.players_online.into() }
}
