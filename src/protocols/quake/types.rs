use std::collections::HashMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// General server information's.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response<P> {
    /// Name of the server.
    pub name: String,
    /// Map name.
    pub map: String,
    /// Current online players.
    pub players: Vec<P>,
    /// Number of players on the server.
    pub players_online: u8,
    /// Maximum number of players the server reports it can hold.
    pub players_maximum: u8,
    /// Indicates whether the server requires a password.
    pub has_password: bool,
    /// Maximum server frags.
    pub frag_limit: u8,
    /// Maximum server time.
    pub time_limit: u8,
    /// The server version.
    pub version: String,
    /// Other server entries that weren't used.
    pub unused_entries: HashMap<String, String>,
}
