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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum VersionedResponse {
    One(one::Response),
    Two(two::Response),
    Three(three::Response),
}
