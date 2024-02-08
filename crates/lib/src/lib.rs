//! Game Server Query Library.
//!
//! # Usage example:
//!
//! ## For a specific game
//! ```
//! use gamedig::games::teamfortress2;
//!
//! let response = teamfortress2::query(&"127.0.0.1".parse().unwrap(), None); // None is the default port (which is 27015), could also be Some(27015)
//! match response { // Result type, must check what it is...
//!     Err(error) => println!("Couldn't query, error: {}", error),
//!     Ok(r) => println!("{:#?}", r)
//! }
//! ```
//!
//! ## Using a game definition
//! ```
//! use gamedig::{GAMES, query};
//!
//! let game = GAMES.get("teamfortress2").unwrap(); // Get a game definition, the full list can be found in src/games/mod.rs
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
//! `serde` - enables serde serialization/deserialization for many gamedig types
//! using serde derive. <br>
//! `games` - include games support. <br>
//! `services` - include services support. <br>
//! `game_defs` - include game definitions for programmatic access (enabled by
//! default). <br>
//! `clap` - enable clap derivations for gamedig settings types. <br>
//! `tls` - enable TLS support for the HTTP client.

pub mod errors;
#[cfg(feature = "games")]
pub mod games;
pub mod protocols;
#[cfg(feature = "services")]
pub mod services;

mod buffer;
mod http;
mod socket;
mod utils;

pub use errors::*;
#[cfg(feature = "games")]
pub use games::*;
#[cfg(feature = "games")]
pub use query::*;
#[cfg(feature = "services")]
pub use services::*;

// Re-export types needed to call games::query::query in the root
pub use protocols::types::{ExtraRequestSettings, TimeoutSettings};
