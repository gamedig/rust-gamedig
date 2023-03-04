
//! Game Server Query Library.
//!
//! # An usage example:
//!
//! ```no_run
//! use gamedig::games::tf2;
//!
//! fn main() {
//!     let response = tf2::query("91.216.250.10", None); //or Some(27015), None is the default protocol port
//!     match response {
//!         Err(error) => println!("Couldn't query, error: {}", error),
//!         Ok(r) => println!("{:?}", r)
//!     }
//! }
//! ```
//!
//! # Crate features:
//! Enabled by default: None
//!
//! `no_games` - disables the included games support.

pub mod errors;
pub mod protocols;
#[cfg(not(feature = "no_games"))]
pub mod games;

mod utils;
mod socket;
mod bufferer;

pub use errors::*;
#[cfg(not(feature = "no_games"))]
pub use games::*;
