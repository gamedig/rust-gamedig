//! Game Server Query Library.
//!
//! # Usage example:
//!
//! ```
//! use gamedig::games::tf2;
//!
//! let response = tf2::query("127.0.0.1", None); // None is the default port (which is 27015), could also be Some(27015)
//! match response { // Result type, must check what it is...
//!     Err(error) => println!("Couldn't query, error: {}", error),
//!     Ok(r) => println!("{:#?}", r)
//! }
//! ```
//!
//! # Crate features:
//! Enabled by default: None
//!
//! `no_games` - disables the included games support.
//! `serde` - enables json serialization/deserialization for all types

pub mod errors;
#[cfg(not(feature = "no_games"))]
pub mod games;
pub mod protocols;
pub mod services;

mod bufferer;
mod socket;
mod utils;

pub use errors::*;
#[cfg(not(feature = "no_games"))]
pub use games::*;
pub use services::*;
