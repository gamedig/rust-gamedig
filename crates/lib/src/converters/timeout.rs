use std::time::Duration;

pub(crate) mod marker {
    use super::Duration;

    /// Marker type for TCP timeout shapes.
    ///
    /// Output is `(connect, read, write)`.
    pub enum TcpMarker {}

    /// Marker type for UDP timeout shapes.
    ///
    /// Output is `(read, write)`.
    pub enum UdpMarker {}

    /// Marker type for HTTP timeout shapes.
    ///
    /// Output is a single global timeout.
    pub enum HttpMarker {}

    /// Marker type for full dictionary / config timeout shapes.
    ///
    /// Output is the entire [`super::TimeoutConfig`].
    pub enum DictMarker {}

    /// Defines the output shape produced for a marker.
    ///
    /// This trait is used to express, at the type level, what a given query
    /// implementation expects its timeout configuration to look like.
    pub trait TimeoutShape {
        /// The normalized timeout representation for this marker.
        type Out;
    }

    impl TimeoutShape for TcpMarker {
        /// `(connect, read, write)`
        type Out = (Option<Duration>, Option<Duration>, Option<Duration>);
    }

    impl TimeoutShape for UdpMarker {
        /// `(read, write)`
        type Out = (Option<Duration>, Option<Duration>);
    }

    impl TimeoutShape for HttpMarker {
        /// `global`
        type Out = Option<Duration>;
    }

    impl TimeoutShape for DictMarker {
        /// Full configuration.
        type Out = super::TimeoutConfig;
    }
}

/// Extension trait for timeout values that can be normalized into a marker shape.
///
/// Implementors define how they convert themselves into the timeout output
/// associated with marker `K`.
pub trait GenericTimeoutExt<K: marker::TimeoutShape>: Send + Sync {
    /// Converts `self` into the marker output type.
    #[must_use]
    fn into_marker(&self) -> K::Out;
}

/// TCP timeout configuration.
///
/// All fields are optional to allow protocol implementations to:
/// - rely on internal defaults, or
/// - selectively override specific operations.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(
    feature = "attribute_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "attribute_extended_derive",
    derive(PartialEq, Eq, PartialOrd, Ord, Hash)
)]
pub struct TcpTimeout {
    /// Timeout for establishing the TCP connection.
    pub connect: Option<Duration>,

    /// Timeout for reading from the socket.
    pub read: Option<Duration>,

    /// Timeout for writing to the socket.
    pub write: Option<Duration>,
}

impl GenericTimeoutExt<marker::TcpMarker> for TcpTimeout {
    /// Normalizes TCP timeouts into `(connect, read, write)`.
    fn into_marker(&self) -> <marker::TcpMarker as marker::TimeoutShape>::Out {
        (self.connect, self.read, self.write)
    }
}

/// UDP timeout configuration.
/// 
/// All fields are optional to allow protocol implementations to:
/// - rely on internal defaults, or
/// - selectively override specific operations.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(
    feature = "attribute_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "attribute_extended_derive",
    derive(PartialEq, Eq, PartialOrd, Ord, Hash)
)]
pub struct UdpTimeout {
    /// Timeout for receiving a UDP packet.
    pub read: Option<Duration>,

    /// Timeout for sending a UDP packet.
    pub write: Option<Duration>,
}

impl GenericTimeoutExt<marker::UdpMarker> for UdpTimeout {
    /// Normalizes UDP timeouts into `(read, write)`.
    fn into_marker(&self) -> <marker::UdpMarker as marker::TimeoutShape>::Out {
        (self.read, self.write)
    }
}

/// HTTP timeout configuration.
///
/// All fields are optional to allow protocol implementations to:
/// - rely on internal defaults, or
/// - selectively override specific operations.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(
    feature = "attribute_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "attribute_extended_derive",
    derive(PartialEq, Eq, PartialOrd, Ord, Hash)
)]
pub struct HttpTimeout {
    /// Global timeout applied to the HTTP request.
    pub global: Option<Duration>,
}

impl GenericTimeoutExt<marker::HttpMarker> for HttpTimeout {
    /// Normalizes HTTP timeouts into `global`.
    fn into_marker(&self) -> <marker::HttpMarker as marker::TimeoutShape>::Out { self.global }
}

/// Full timeout configuration for multiple transports.
///
/// This is convenient for consumers that want to provide a single config
/// and let each query type extract what it needs.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(
    feature = "attribute_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "attribute_extended_derive",
    derive(PartialEq, Eq, PartialOrd, Ord, Hash)
)]
pub struct TimeoutConfig {
    /// TCP timeout configuration.
    pub tcp: TcpTimeout,

    /// UDP timeout configuration.
    pub udp: UdpTimeout,

    /// HTTP timeout configuration.
    pub http: HttpTimeout,
}

impl GenericTimeoutExt<marker::TcpMarker> for TimeoutConfig {
    /// Extracts and normalizes TCP timeouts from the full config.
    fn into_marker(&self) -> <marker::TcpMarker as marker::TimeoutShape>::Out {
        self.tcp.into_marker()
    }
}

impl GenericTimeoutExt<marker::UdpMarker> for TimeoutConfig {
    /// Extracts and normalizes UDP timeouts from the full config.
    fn into_marker(&self) -> <marker::UdpMarker as marker::TimeoutShape>::Out {
        self.udp.into_marker()
    }
}

impl GenericTimeoutExt<marker::HttpMarker> for TimeoutConfig {
    /// Extracts and normalizes HTTP timeouts from the full config.
    fn into_marker(&self) -> <marker::HttpMarker as marker::TimeoutShape>::Out {
        self.http.into_marker()
    }
}

impl GenericTimeoutExt<marker::DictMarker> for TimeoutConfig {
    /// Returns the full configuration as-is.
    fn into_marker(&self) -> <marker::DictMarker as marker::TimeoutShape>::Out { *self }
}

impl<K: marker::TimeoutShape, T: GenericTimeoutExt<K> + ?Sized + Send + Sync> GenericTimeoutExt<K>
    for &T
{
    /// Allows passing references to timeout configs wherever a timeout is accepted.
    fn into_marker(&self) -> K::Out { (**self).into_marker() }
}
