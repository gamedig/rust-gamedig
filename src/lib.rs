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
//! Enabled by default: `protocols_all`, `services_all`
//!
//! `serde` - enables json serialization/deserialization for all response types. <br>
//! `protocol_all` - Enables all protocols. <br>
//! `service_all` - Enables all services. <br><br>
//! `protocol_valve` - Enables valve query protocol.<br>
//! `protocol_gamespy` - Enable gamespy protocol.<br>
//! `protocol_minecraft` - Enable minecraft protocol.<br><br>
//! `service_valve` - Enable valve master server protocol.

pub mod errors;
#[cfg(feature = "games")]
pub mod games;
pub mod protocols;
#[cfg(feature = "services")]
pub mod services;

mod bufferer;
mod socket;
mod utils;

pub use errors::*;
#[cfg(feature = "games")]
pub use games::*;
#[cfg(feature = "services")]
pub use services::*;
