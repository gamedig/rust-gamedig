#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod one;
pub mod three;
pub mod two;

/// All types used by the implementation.
pub mod types;
pub use types::*;

mod client;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuakeVersion {
    One,
    Two,
    Three,
}
