//! Every GameDig errors.

/// The Error with backtrace.
pub mod error;
/// All defined Error kinds.
pub mod kind;
/// `GDResult`, a shorthand of `Result<T, GDError>`.
pub mod result;

pub use error::*;
pub use kind::*;
pub use result::*;
