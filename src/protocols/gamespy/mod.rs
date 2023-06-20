#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod common;
/// The implementations.
pub mod protocols;

pub use protocols::*;

/// Versions of the gamespy protocol
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum GameSpyVersion {
    One,
    Two,
    Three,
}

/// Versioned response type
#[derive(Debug, Clone, PartialEq)]
pub enum VersionedResponse<'a> {
    One(&'a one::Response),
    Two(&'a two::Response),
    Three(&'a three::Response),
}

/// Versioned player type
#[derive(Debug, Clone, PartialEq)]
pub enum VersionedPlayer<'a> {
    One(&'a one::Player),
    Two(&'a two::Player),
    Three(&'a three::Player),
}
