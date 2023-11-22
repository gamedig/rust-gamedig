//! Game related types

use crate::protocols::types::Protocol;

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

/// Generic extra request settings
///
/// Fields of this struct may not be used depending on which protocol
/// is selected, the individual fields link to the specific places
/// they will be used with additional documentation.
///
/// ## Examples
/// Create minecraft settings with builder:
/// ```
/// use gamedig::games::minecraft;
/// use gamedig::protocols::ExtraRequestSettings;
/// let mc_settings: minecraft::RequestSettings = ExtraRequestSettings::default().set_hostname("mc.hypixel.net".to_string()).into();
/// ```
///
/// Create valve settings with builder:
/// ```
/// use gamedig::protocols::{valve, ExtraRequestSettings};
/// let valve_settings: valve::GatheringSettings = ExtraRequestSettings::default().set_check_app_id(false).into();
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ExtraRequestSettings {
    /// The server's hostname.
    ///
    /// Used by:
    /// - [crate::games::minecraft::RequestSettings#structfield.hostname]
    pub hostname: Option<String>,
    /// The protocol version to use.
    ///
    /// Used by:
    /// - [crate::games::minecraft::RequestSettings#structfield.
    ///   protocol_version]
    pub protocol_version: Option<i32>,
    /// Whether to gather player information
    ///
    /// Used by:
    /// - [crate::protocols::valve::GatheringSettings#structfield.players]
    pub gather_players: Option<bool>,
    /// Whether to gather rule information.
    ///
    /// Used by:
    /// - [crate::protocols::valve::GatheringSettings#structfield.rules]
    pub gather_rules: Option<bool>,
    /// Whether to check if the App ID is valid.
    ///
    /// Used by:
    /// - [crate::protocols::valve::GatheringSettings#structfield.check_app_id]
    pub check_app_id: Option<bool>,
}

impl ExtraRequestSettings {
    /// [Sets hostname](ExtraRequestSettings#structfield.hostname)
    pub fn set_hostname(mut self, hostname: String) -> Self {
        self.hostname = Some(hostname);
        self
    }
    /// [Sets protocol
    /// version](ExtraRequestSettings#structfield.protocol_version)
    pub fn set_protocol_version(mut self, protocol_version: i32) -> Self {
        self.protocol_version = Some(protocol_version);
        self
    }
    /// [Sets gather players](ExtraRequestSettings#structfield.gather_players)
    pub fn set_gather_players(mut self, gather_players: bool) -> Self {
        self.gather_players = Some(gather_players);
        self
    }
    /// [Sets gather rules](ExtraRequestSettings#structfield.gather_rules)
    pub fn set_gather_rules(mut self, gather_rules: bool) -> Self {
        self.gather_rules = Some(gather_rules);
        self
    }
    /// [Sets check app ID](ExtraRequestSettings#structfield.check_app_id)
    pub fn set_check_app_id(mut self, check_app_id: bool) -> Self {
        self.check_app_id = Some(check_app_id);
        self
    }
}
