#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod common;
/// The implementations.
pub mod protocols;

pub use protocols::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum GameSpyVersion {
    One,
    Two,
    Three,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum VersionedExtraResponse {
    One(one::ExtraResponse),
    Two(two::ExtraResponse),
    Three(three::ExtraResponse),
}
