/// Represents a generic game server with associated metadata and connected players.
#[derive(Debug, Clone)]
pub struct GenericServer {
    pub name: String,
    pub description: Option<String>,

    pub map: Option<String>,
    pub mode: Option<String>,
    pub version: Option<String>,
    pub anti_cheat: Option<String>,
    pub has_password: Option<bool>,
    
    pub max_players: u32,
    pub current_players: u32,
    pub players: Option<Vec<super::GenericPlayer>>,

    pub additional_data: Option<super::GenericDataMap>,
}

/// Extension trait for types that can be represented as a [`GenericServer`].
pub trait GenericServerExt {
    /// Returns a [`GenericServer`] representation of `self`.
    #[must_use]
    fn into_generic_server(self) -> GenericServer;
}
