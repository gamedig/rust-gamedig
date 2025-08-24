use {
    super::{data::GenericDataHashMap, player::GenericPlayer},
    std::{collections::HashSet, net::SocketAddr},
};

/// Represents a generic game server with associated metadata and connected players.
#[derive(Debug, Clone)]
pub struct GenericServer {
    pub addr: SocketAddr,
    pub data: Option<GenericDataHashMap>,
    pub players: Option<HashSet<GenericPlayer>>,
}

/// A trait for converting server structs into a [`GenericServer`].
pub trait IntoGenericServer: Sized {
    fn into_generic_server(&self) -> GenericServer;
}
