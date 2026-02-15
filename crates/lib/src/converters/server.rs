/// Represents a generic game server.
#[derive(Debug, Clone)]
pub struct GenericServer {
    pub name: String,
    pub description: Option<String>,

    pub map: Option<String>,
    pub mode: Option<String>,
    pub version: Option<String>,
    pub anti_cheat: Option<bool>,
    pub has_password: Option<bool>,

    pub max_players: u16,
    pub current_players: u16,
    pub players: Option<Vec<super::GenericPlayer>>,
}

/// Represents a generic game server with associated additional data.
#[derive(Debug, Clone)]
pub struct GenericServerWithAdditionalData<S, P> {
    pub name: String,
    pub description: Option<String>,

    pub map: Option<String>,
    pub mode: Option<String>,
    pub version: Option<String>,
    pub anti_cheat: Option<bool>,
    pub has_password: Option<bool>,

    pub max_players: u16,
    pub current_players: u16,
    pub players: Option<Vec<super::GenericPlayerWithAdditionalData<P>>>,

    pub additional_data: S,
}

impl<S, P> From<GenericServerWithAdditionalData<S, P>> for GenericServer {
    fn from(s: GenericServerWithAdditionalData<S, P>) -> Self {
        GenericServer {
            name: s.name,
            description: s.description,
            map: s.map,
            mode: s.mode,
            version: s.version,
            anti_cheat: s.anti_cheat,
            has_password: s.has_password,
            max_players: s.max_players,
            current_players: s.current_players,
            players: s
                .players
                .map(|players| players.into_iter().map(Into::into).collect()),
        }
    }
}

/// Extension trait for types that can be represented as a [`GenericServer`].
pub trait GenericServerExt {
    type AdditionalServerData;
    type AdditionalPlayerData;

    /// Returns a [`GenericServer`] representation of `self`.
    #[must_use]
    fn into_generic_server(self) -> GenericServer { self.into_generic_server_with_data().into() }

    /// Returns a [`GenericServerWithAdditionalData`] representation of `self`.
    #[must_use]
    fn into_generic_server_with_data(
        self,
    ) -> GenericServerWithAdditionalData<Self::AdditionalServerData, Self::AdditionalPlayerData>;
}
