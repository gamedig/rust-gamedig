use crate::core::error::Report;

/// High level classification of errors exposed by the public API.
///
/// This enum provides a coarse grained view of errors suitable for matching
/// and handling without needing to use specific client error types.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Errors caused by client side logic, invariants, or sanity checks.
    Client,

    /// Errors related to parsing or decoding data.
    Parse,

    /// Errors originating from network operations or connectivity.
    Networking,
}

/// Extension trait for retrieving an [`ErrorCategory`] from an error value.
///
/// This trait is intended for classification only.
pub trait ErrorCategoryExt: Send + Sync + 'static {
    /// Returns the [`ErrorCategory`] associated with this error.
    #[must_use]
    fn category(&self) -> ErrorCategory;
}

impl<T: ErrorCategoryExt> ErrorCategoryExt for Report<T> {
    fn category(&self) -> ErrorCategory { self.current_context().category() }
}
