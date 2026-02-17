use {
    super::{GenericServerExt, GenericTimeoutExt, TimeoutShape},
    crate::core::{ToSocketAddr, error::Report},
    std::error::Error,
};

/// Extension trait for types that can perform a generic address based query.
///
/// This trait abstracts over different protocol implementations that can
/// query a server via a [`SocketAddr`] and produce a response that can be
/// normalized into a [`GenericServerExt`] representation.
#[maybe_async::maybe_async]
pub trait GenericQueryExt {
    /// Successful response type returned by the query.
    ///
    /// The response must support conversion into generic server types
    /// via [`GenericServerExt`].
    type Response: GenericServerExt;

    /// Error type returned by the query.
    ///
    /// Errors are wrapped in [`Report`] to provide richer diagnostic context.
    type Error: Error;

    /// Type of timeout configuration accepted by the query.
    ///
    /// This defines the “shape” of timeouts.
    type Timeout: TimeoutShape;

    /// Performs a one off query against the given socket address.
    ///
    /// Implementations should use their default timeout configuration.
    #[must_use]
    async fn query_addr<A: ToSocketAddr>(addr: A) -> Result<Self::Response, Report<Self::Error>>;

    /// Performs a one off query against the given socket address with the provided timeout configuration.
    ///
    /// The timeout argument is accepted via [`GenericTimeoutExt`], allowing
    /// multiple timeout configuration types to be passed in and
    /// normalized into the marker shape `Self::Timeout`.
    #[must_use]
    async fn query_addr_with_timeout<A: ToSocketAddr>(
        addr: A,
        timeout: impl GenericTimeoutExt<Self::Timeout>,
    ) -> Result<Self::Response, Report<Self::Error>>;
}
