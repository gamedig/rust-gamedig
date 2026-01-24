/// Represents a generic game server with associated metadata and connected players.
#[derive(Debug, Clone)]
pub struct GenericServer {
    pub data: Option<super::GenericDataMap>,
    pub players: Option<Vec<super::GenericPlayer>>,
}

/// Extension trait for types that can be represented as a [`GenericServer`].
pub trait GenericServerExt {
    /// Returns a [`GenericServer`] representation of `self`.
    #[must_use]
    fn as_generic_server(self) -> GenericServer;
}
