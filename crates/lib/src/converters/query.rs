use std::net::SocketAddr;

/// Extension trait for types that can perform a generic address based query.
#[maybe_async::maybe_async]
pub trait GenericQueryExt {
    /// Successful response type returned by the query.
    type Response: super::GenericServerExt;

    /// Error type returned by the query.
    type Error: super::ErrorCategoryExt;

    /// Performs a one-off query against the given socket address.
    #[must_use]
    async fn query_addr(addr: &SocketAddr) -> Result<Self::Response, Self::Error>;
}
