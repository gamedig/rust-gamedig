use crate::protocols::{gamespy, minecraft, quake, valve};
use crate::{GDError::InvalidInput, GDResult};

use std::time::Duration;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enumeration of all valid protocol types
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Protocol {
    Gamespy(gamespy::GameSpyVersion),
    Minecraft(Option<minecraft::types::Server>),
    Quake(quake::QuakeVersion),
    Valve(valve::SteamApp),
    #[cfg(not(feature = "no_games"))]
    TheShip,
    #[cfg(not(feature = "no_games"))]
    FFOW,
    JC2MP,
}

/// All response types
#[derive(Debug, Clone, PartialEq)]
pub enum GenericResponse<'a> {
    GameSpy(gamespy::VersionedResponse<'a>),
    Minecraft(minecraft::VersionedResponse<'a>),
    Quake(quake::VersionedResponse<'a>),
    Valve(&'a valve::Response),
    #[cfg(not(feature = "no_games"))]
    TheShip(&'a crate::games::ts::Response),
    #[cfg(not(feature = "no_games"))]
    FFOW(&'a crate::games::ffow::Response),
    #[cfg(not(feature = "no_games"))]
    JC2MP(&'a crate::games::jc2mp::Response),
}

/// All player types
#[derive(Debug, Clone, PartialEq)]
pub enum GenericPlayer<'a> {
    Valve(&'a valve::ServerPlayer),
    QuakeOne(&'a quake::one::Player),
    QuakeTwo(&'a quake::two::Player),
    Minecraft(&'a minecraft::Player),
    Gamespy(gamespy::VersionedPlayer<'a>),
    #[cfg(not(feature = "no_games"))]
    TheShip(&'a crate::games::ts::TheShipPlayer),
    #[cfg(not(feature = "no_games"))]
    JCMP2(&'a crate::games::jc2mp::Player),
}

pub trait CommonResponse {
    /// Get the original response type
    fn as_original(&self) -> GenericResponse;
    /// Get a struct that can be stored as JSON (you don't need to override
    /// this)
    fn as_json(&self) -> CommonResponseJson {
        CommonResponseJson {
            name: self.name(),
            description: self.description(),
            game: self.game(),
            game_version: self.game_version(),
            has_password: self.has_password(),
            map: self.map(),
            players_maximum: self.players_maximum(),
            players_online: self.players_online(),
            players_bots: self.players_bots(),
            players: self
                .players()
                .map(|players| players.iter().map(|p| p.as_json()).collect()),
        }
    }

    /// The name of the server
    fn name(&self) -> Option<&str> { None }
    /// Description of the server
    fn description(&self) -> Option<&str> { None }
    /// Name of the current game or game mode
    fn game(&self) -> Option<&str> { None }
    /// Version of the game being run on the server
    fn game_version(&self) -> Option<&str> { None }
    /// The current map name
    fn map(&self) -> Option<&str> { None }
    /// Maximum number of players allowed to connect
    fn players_maximum(&self) -> u64;
    /// Number of players currently connected
    fn players_online(&self) -> u64;
    /// Number of bots currently connected
    fn players_bots(&self) -> Option<u64> { None }
    /// Whether the server requires a password to join
    fn has_password(&self) -> Option<bool> { None }
    /// Currently connected players
    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> { None }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CommonResponseJson<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub game: Option<&'a str>,
    pub game_version: Option<&'a str>,
    pub map: Option<&'a str>,
    pub players_maximum: u64,
    pub players_online: u64,
    pub players_bots: Option<u64>,
    pub has_password: Option<bool>,
    pub players: Option<Vec<CommonPlayerJson<'a>>>,
}

pub trait CommonPlayer {
    /// Get the original player type
    fn as_original(&self) -> GenericPlayer;
    /// Get a struct that can be stored as JSON (you don't need to override
    /// this)
    fn as_json(&self) -> CommonPlayerJson {
        CommonPlayerJson {
            name: self.name(),
            score: self.score(),
        }
    }

    /// Player name
    fn name(&self) -> &str;
    /// Player score
    fn score(&self) -> Option<u32> { None }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CommonPlayerJson<'a> {
    pub name: &'a str,
    pub score: Option<u32>,
}

/// Timeout settings for socket operations
#[derive(Clone, Debug)]
pub struct TimeoutSettings {
    read: Option<Duration>,
    write: Option<Duration>,
}

impl TimeoutSettings {
    /// Construct new settings, passing None will block indefinitely. Passing
    /// zero Duration throws GDError::[InvalidInput](InvalidInput).
    pub fn new(read: Option<Duration>, write: Option<Duration>) -> GDResult<Self> {
        if let Some(read_duration) = read {
            if read_duration == Duration::new(0, 0) {
                return Err(InvalidInput);
            }
        }

        if let Some(write_duration) = write {
            if write_duration == Duration::new(0, 0) {
                return Err(InvalidInput);
            }
        }

        Ok(Self { read, write })
    }

    /// Get the read timeout.
    pub fn get_read(&self) -> Option<Duration> { self.read }

    /// Get the write timeout.
    pub fn get_write(&self) -> Option<Duration> { self.write }
}

impl Default for TimeoutSettings {
    /// Default values are 4 seconds for both read and write.
    fn default() -> Self {
        Self {
            read: Some(Duration::from_secs(4)),
            write: Some(Duration::from_secs(4)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // Test creating new TimeoutSettings with valid durations
    #[test]
    fn test_new_with_valid_durations() -> GDResult<()> {
        // Define valid read and write durations
        let read_duration = Duration::from_secs(1);
        let write_duration = Duration::from_secs(2);

        // Create new TimeoutSettings with the valid durations
        let timeout_settings = TimeoutSettings::new(Some(read_duration), Some(write_duration))?;

        // Verify that the get_read and get_write methods return the expected values
        assert_eq!(timeout_settings.get_read(), Some(read_duration));
        assert_eq!(timeout_settings.get_write(), Some(write_duration));

        Ok(())
    }

    // Test creating new TimeoutSettings with a zero duration
    #[test]
    fn test_new_with_zero_duration() {
        // Define a zero read duration and a valid write duration
        let read_duration = Duration::new(0, 0);
        let write_duration = Duration::from_secs(2);

        // Try to create new TimeoutSettings with the zero read duration (this should
        // fail)
        let result = TimeoutSettings::new(Some(read_duration), Some(write_duration));

        // Verify that the function returned an error and that the error type is
        // InvalidInput
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), InvalidInput);
    }

    // Test that the default TimeoutSettings values are correct
    #[test]
    fn test_default_values() {
        // Get the default TimeoutSettings values
        let default_settings = TimeoutSettings::default();

        // Verify that the get_read and get_write methods return the expected default
        // values
        assert_eq!(default_settings.get_read(), Some(Duration::from_secs(4)));
        assert_eq!(default_settings.get_write(), Some(Duration::from_secs(4)));
    }
}
