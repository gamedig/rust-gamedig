use {
    super::{data::GenericDataMap, player::GenericPlayer},
    std::net::SocketAddr,
};

/// Represents a generic game server with associated metadata and connected players.
#[derive(Debug, Clone)]
pub struct GenericServer {
    pub addr: SocketAddr,
    pub data: Option<GenericDataMap>,
    pub players: Option<Vec<GenericPlayer>>,
}

/// Extension trait for types that can be represented as a [`GenericServer`].
pub trait GenericServerExt {
    /// Returns a [`GenericServer`] representation of `self`.
    #[must_use]
    fn as_generic_server(&self) -> GenericServer;
}
