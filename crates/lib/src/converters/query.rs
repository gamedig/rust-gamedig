use {
    crate::core::error::Report,
    std::{error::Error, net::SocketAddr},
};

/// Extension trait for types that can perform a generic address based query.
#[maybe_async::maybe_async]
pub trait GenericQueryExt {
    /// Successful response type returned by the query.
    type Response: super::GenericServerExt;

    /// Error type returned by the query.
    type Error: super::ErrorCategoryExt + Error;

    /// Type of timeout configuration accepted by the query.
    type Timeout: super::TimeoutShape;

    /// Performs a one-off query against the given socket address.
    #[must_use]
    async fn query_addr(addr: &SocketAddr) -> Result<Self::Response, Report<Self::Error>>;

    /// Performs a one-off query against the given socket address with the provided timeout configuration.
    #[must_use]
    async fn query_addr_with_timeout(
        addr: &SocketAddr,
        timeout: impl super::GenericTimeoutExt<Self::Timeout>,
    ) -> Result<Self::Response, Report<Self::Error>>;
}
