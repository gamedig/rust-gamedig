//! Game Server Query Library.
//!
//! # Usage example:
//!
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
//! # Crate features:
//! Enabled by default: None
//!
//! `serde` - enables json serialization/deserialization for all response types. <br>
//! `no_games` - disables the included games support. <br>
//! `no_services` - disables the included services support.

pub mod errors;
#[cfg(not(feature = "no_games"))]
pub mod games;
pub mod protocols;
#[cfg(not(feature = "no_services"))]
pub mod services;

mod bufferer;
mod socket;
mod types;
mod utils;

pub use errors::*;
#[cfg(not(feature = "no_games"))]
pub use games::*;
#[cfg(not(feature = "no_services"))]
pub use services::*;
pub use types::*;
