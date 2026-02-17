use {
    super::error::{Report, ResultExt},
    std::{
        fmt::Debug,
        net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum ToSocketAddrError {
    #[error("[GameDig]::[DNS::RESOLUTION] Failed to resolve the provided address")]
    Resolution,

    #[error("[GameDig]::[DNS::NO_RECORDS] Address resolution returned no usable records")]
    NoRecords,
}

/// A trait for types that can be resolved into a single [`SocketAddr`].
///
/// This trait provides a unified abstraction over inputs that represent a
/// network endpoint, such as:
///
/// - [`SocketAddr`]
/// - `&SocketAddr`
/// - `&str` / `String` (e.g. `"example.com:27015"`)
/// - `(&str, u16)`
///
/// Implementations may perform DNS resolution if required.
///
/// If the input resolves to multiple address records, implementations
/// apply a deterministic selection by returning the first resolved address.
#[maybe_async::maybe_async]
pub trait ToSocketAddr: Debug {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>>;
}

#[maybe_async::maybe_async]
impl ToSocketAddr for SocketAddr {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> { Ok(*self) }
}

#[maybe_async::maybe_async]
impl ToSocketAddr for SocketAddrV4 {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        Ok((*self).into())
    }
}

#[maybe_async::maybe_async]
impl ToSocketAddr for SocketAddrV6 {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        Ok((*self).into())
    }
}

#[maybe_async::maybe_async]
impl ToSocketAddr for (IpAddr, u16) {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        Ok((*self).into())
    }
}

#[maybe_async::maybe_async]
impl ToSocketAddr for (Ipv4Addr, u16) {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        Ok((*self).into())
    }
}

#[maybe_async::maybe_async]
impl ToSocketAddr for (Ipv6Addr, u16) {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        Ok((*self).into())
    }
}

#[maybe_async::sync_impl]
impl ToSocketAddr for str {
    fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        use std::net::ToSocketAddrs;

        self.to_socket_addrs()
            .change_context(ToSocketAddrError::Resolution)?
            .next()
            .ok_or_else(|| Report::new(ToSocketAddrError::NoRecords))
    }
}

#[cfg(feature = "_RT_TOKIO")]
#[maybe_async::async_impl]
impl ToSocketAddr for str {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        use tokio::net::lookup_host;

        lookup_host(self)
            .await
            .change_context(ToSocketAddrError::Resolution)?
            .next()
            .ok_or_else(|| Report::new(ToSocketAddrError::NoRecords))
    }
}

#[maybe_async::maybe_async]
impl ToSocketAddr for String {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        (&**self).to_socket_addr().await
    }
}

#[maybe_async::sync_impl]
impl ToSocketAddr for (&str, u16) {
    fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        use std::net::ToSocketAddrs;

        self.to_socket_addrs()
            .change_context(ToSocketAddrError::Resolution)?
            .next()
            .ok_or_else(|| Report::new(ToSocketAddrError::NoRecords))
    }
}

#[cfg(feature = "_RT_TOKIO")]
#[maybe_async::async_impl]
impl ToSocketAddr for (&str, u16) {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        use tokio::net::lookup_host;

        lookup_host(self)
            .await
            .change_context(ToSocketAddrError::Resolution)?
            .next()
            .ok_or_else(|| Report::new(ToSocketAddrError::NoRecords))
    }
}

#[maybe_async::maybe_async]
impl ToSocketAddr for (String, u16) {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        (&*self.0, self.1).to_socket_addr().await
    }
}

#[maybe_async::maybe_async]
impl<T: ToSocketAddr + ?Sized + Sync> ToSocketAddr for &T {
    async fn to_socket_addr(&self) -> Result<SocketAddr, Report<ToSocketAddrError>> {
        (**self).to_socket_addr().await
    }
}
