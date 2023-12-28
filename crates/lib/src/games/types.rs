//! Game related types

use crate::protocols::types::{ExtraRequestSettings, Protocol};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Definition of a game
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    /// Full name of the game
    pub name: &'static str,
    /// Default port used by game
    pub default_port: u16,
    /// The protocol the game's query uses
    pub protocol: Protocol,
    /// Request settings.
    pub request_settings: ExtraRequestSettings,
}
