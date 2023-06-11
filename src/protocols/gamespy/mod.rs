#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

mod common;
/// The implementations.
pub mod protocols;

pub use protocols::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum GameSpyVersion {
    One,
    Three,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseVersion {
    One(protocols::one::Response),
    Three(protocols::three::Response),
}
