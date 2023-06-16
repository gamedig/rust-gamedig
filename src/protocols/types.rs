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
}

// A generic version of a response
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[derive(Debug, Clone, PartialEq)]
// pub struct GenericResponse {
// The name of the server
// pub name: Option<String>,
// Description of the server
// pub description: Option<String>,
// Name of the current game or game mode
// pub game: Option<String>,
// Version of the game being run on the server
// pub game_version: Option<String>,
// The current map name
// pub map: Option<String>,
// Maximum number of players allowed to connect
// pub players_maximum: u64,
// Number of players currently connected
// pub players_online: u64,
// Number of bots currently connected
// pub players_bots: Option<u64>,
// Whether the server requires a password to join
// pub has_password: Option<bool>,
// Data specific to non-generic responses
// pub inner: SpecificResponse,
// }

/// All response types
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum GenericResponse {
    GameSpy(gamespy::VersionedResponse),
    Minecraft(minecraft::VersionedResponse),
    Quake(quake::VersionedResponse),
    Valve(valve::Response),
    #[cfg(not(feature = "no_games"))]
    TheShip(crate::games::ts::Response),
    #[cfg(not(feature = "no_games"))]
    FFOW(crate::games::ffow::Response),
}

impl GenericResponse {
    /// The name of the server
    pub fn name(&self) -> Option<&String> {
        todo!();
    }
    /// Description of the server
    pub fn description(&self) -> Option<&String> {
        match self {
            GenericResponse::Minecraft(minecraft::VersionedResponse::Java(r)) => Some(&r.description),
            GenericResponse::FFOW(r) => Some(&r.description),
            _ => None,
        }
    }
    /// Name of the current game or game mode
    pub fn game(&self) -> Option<String> {
        todo!();
    }
    /// Version of the game being run on the server
    pub fn game_version(&self) -> Option<String> {
        todo!();
    }
    /// The current map name
    pub fn map(&self) -> Option<String> {
        todo!();
    }
    /// Maximum number of players allowed to connect
    pub fn players_maximum(&self) -> u64 {
        match self {
            GenericResponse::FFOW(r) => r.players_maximum.into(),
            GenericResponse::Quake(r) => match r {
                quake::VersionedResponse::One(r) => r.players_maximum.into(),
                quake::VersionedResponse::TwoAndThree(r) => r.players_maximum.into(),
            },
            GenericResponse::GameSpy(r) => match r {
                gamespy::VersionedResponse::One(r) => r.players_maximum.try_into().unwrap(),
                gamespy::VersionedResponse::Two(r) => r.players_maximum.try_into().unwrap(),
                gamespy::VersionedResponse::Three(r) => r.players_maximum.try_into().unwrap(),
            },
            GenericResponse::TheShip(r) => r.max_players.into(),
            GenericResponse::Minecraft(r) => match r {
                minecraft::VersionedResponse::Java(r) => r.players_maximum.into(),
                minecraft::VersionedResponse::Bedrock(r) => r.players_maximum.into(),
            },
            GenericResponse::Valve(r) => r.info.players_maximum.into(),
        }
    }
    /// Number of players currently connected
    pub fn players_online(&self) -> u64 {
        todo!();
    }
    /// Number of bots currently connected
    pub fn players_bots(&self) -> Option<u64> {
        todo!();
    }
    /// Whether the server requires a password to join
    pub fn has_password(&self) -> Option<bool> {
        todo!();
    }
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
