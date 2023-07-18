//! Game Server Query Library.
//!
//! # Usage example:
//!
//! ## For a specific game
//! ```
//! use gamedig::games::tf2;
//!
//! let response = tf2::query(&"127.0.0.1".parse().unwrap(), None); // None is the default port (which is 27015), could also be Some(27015)
//! match response { // Result type, must check what it is...
//!     Err(error) => println!("Couldn't query, error: {}", error),
//!     Ok(r) => println!("{:#?}", r)
//! }
//! ```
//!
//! ## Using a game definition
//! ```
//! use gamedig::games::{GAMES, query};
//!
//! let game = GAMES.get("tf2").unwrap(); // Get a game definition, the full list can be found in src/games/mod.rs
//! let response = query(game, &"127.0.0.1".parse().unwrap(), None); // None will use the default port
//! match response {
//!     Err(error) => println!("Couldn't query, error: {}", error),
//!     Ok(r) => println!("{:#?}", r.as_json()),
//! }
//! ```
//!
//! # Crate features:
//! Enabled by default: `games`, `game_defs`, `services`
//!
//! `serde` - enables json serialization/deserialization for all response types.
//! <br> `games` - include games support. <br>
//! `services` - include services support. <br>
//! `game_defs` - Include game definitions for programmatic access (enabled by
//! default).

pub mod errors;
#[cfg(feature = "games")]
pub mod games;
pub mod protocols;
#[cfg(feature = "services")]
pub mod services;

mod buffer;
mod socket;
mod utils;

pub use errors::*;
#[cfg(feature = "games")]
pub use games::*;
#[cfg(feature = "services")]
pub use services::*;
