
//! Game Server Query Library.
//!
//! # An usage example:
//!
//! ```no_run
//!use gamedig::games::tf2;
//!
//!fn main() {
//!    let response = tf2::query("127.0.0.1", None); // None is the default port (which is 27015), could also be Some(27015)
//!    match response { // Result type, must check what it is...
//!        Err(error) => println!("Couldn't query, error: {}", error),
//!        Ok(r) => println!("{:#?}", r)
//!    }
//!}
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
