
//! Query many servers
//!
//! # Example
//!
//! ```no_run
//! use gamedig::games::tf2;
//!
//! fn main() {
//!     let response = tf2::query("91.216.250.10", None); //or Some(27015), None is the default protocol port
//!     match response {
//!         Err(error) => println!("Couldn't query, error: {error}"),
//!         Ok(r) => println!("{:?}", r)
//!     }
//! }
//! ```

pub mod errors;
pub mod protocols;
pub mod games;
mod utils;

pub use errors::*;
pub use protocols::*;
pub use games::*;
