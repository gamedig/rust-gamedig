use std::time::Duration;

pub(crate) mod marker {
    use super::Duration;

    pub enum TcpMarker {}
    pub enum UdpMarker {}
    pub enum HttpMarker {}

    pub enum DictMarker {}

    pub trait TimeoutShape {
        type Out;
    }

    impl TimeoutShape for TcpMarker {
        // Connect, Read, Write
        type Out = (Option<Duration>, Option<Duration>, Option<Duration>);
    }

    impl TimeoutShape for UdpMarker {
        // Read, Write
        type Out = (Option<Duration>, Option<Duration>);
    }

    impl TimeoutShape for HttpMarker {
        // Global
        type Out = Option<Duration>;
    }

    impl TimeoutShape for DictMarker {
        // Full config
        type Out = super::TimeoutConfig;
    }
}

pub trait GenericTimeoutExt<K: marker::TimeoutShape>: Send + Sync {
    #[must_use]
    fn into_marker(&self) -> K::Out;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TcpTimeout {
    pub connect: Option<Duration>,
    pub read: Option<Duration>,
    pub write: Option<Duration>,
}

impl GenericTimeoutExt<marker::TcpMarker> for TcpTimeout {
    fn into_marker(&self) -> <marker::TcpMarker as marker::TimeoutShape>::Out {
        (self.connect, self.read, self.write)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UdpTimeout {
    pub read: Option<Duration>,
    pub write: Option<Duration>,
}

impl GenericTimeoutExt<marker::UdpMarker> for UdpTimeout {
    fn into_marker(&self) -> <marker::UdpMarker as marker::TimeoutShape>::Out {
        (self.read, self.write)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HttpTimeout {
    pub global: Option<Duration>,
}

impl GenericTimeoutExt<marker::HttpMarker> for HttpTimeout {
    fn into_marker(&self) -> <marker::HttpMarker as marker::TimeoutShape>::Out { self.global }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TimeoutConfig {
    pub tcp: TcpTimeout,
    pub udp: UdpTimeout,
    pub http: HttpTimeout,
}

impl GenericTimeoutExt<marker::TcpMarker> for TimeoutConfig {
    fn into_marker(&self) -> <marker::TcpMarker as marker::TimeoutShape>::Out {
        self.tcp.into_marker()
    }
}

impl GenericTimeoutExt<marker::UdpMarker> for TimeoutConfig {
    fn into_marker(&self) -> <marker::UdpMarker as marker::TimeoutShape>::Out {
        self.udp.into_marker()
    }
}

impl GenericTimeoutExt<marker::HttpMarker> for TimeoutConfig {
    fn into_marker(&self) -> <marker::HttpMarker as marker::TimeoutShape>::Out {
        self.http.into_marker()
    }
}

impl GenericTimeoutExt<marker::DictMarker> for TimeoutConfig {
    fn into_marker(&self) -> <marker::DictMarker as marker::TimeoutShape>::Out { *self }
}

impl<K: marker::TimeoutShape, T: GenericTimeoutExt<K> + ?Sized + Send + Sync> GenericTimeoutExt<K>
    for &T
{
    fn into_marker(&self) -> K::Out { (**self).into_marker() }
}
