//! Currently supported games.

#[cfg(feature = "tls")]
pub mod epic;
pub mod gamespy;
pub mod quake;
pub mod unreal2;
pub mod valve;

#[cfg(all(feature = "tls", feature = "serde", feature = "services"))]
pub mod minetest;

#[cfg(feature = "tls")]
pub use epic::*;
pub use gamespy::*;
pub use quake::*;
pub use unreal2::*;
pub use valve::*;

#[cfg(all(feature = "tls", feature = "serde", feature = "services"))]
pub use minetest::*;

/// Battalion 1944
pub mod battalion1944;
/// Eco
pub mod eco;
/// Frontlines: Fuel of War
pub mod ffow;
/// Just Cause 2: Multiplayer
pub mod jc2m;
/// Mindustry
pub mod mindustry;
/// Minecraft
pub mod minecraft;
/// Savage 2
pub mod savage2;
/// The Ship
pub mod theship;

pub mod types;
pub use types::*;

pub mod query;
pub use query::*;

#[cfg(feature = "game_defs")]
mod definitions;

#[cfg(feature = "game_defs")]
pub use definitions::GAMES;
