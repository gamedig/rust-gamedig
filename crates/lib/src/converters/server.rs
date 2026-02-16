use super::{GenericPlayer, GenericPlayerExt, GenericPlayerWithAdditionalData};

/// Represents a generic, game agnostic server.
///
/// This type contains only normalized fields that can be shared
/// across different game protocols and implementations.
///
/// Protocol specific metadata is intentionally excluded. if you need
/// to preserve it, use [`GenericServerWithAdditionalData`].
#[derive(Debug)]
#[cfg_attr(
    feature = "attribute_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "attribute_extended_derive",
    derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)
)]
pub struct GenericServer {
    /// Server name.
    pub name: String,

    /// Optional server description or MOTD.
    ///
    /// Empty strings received from a protocol may be normalized to `None`
    /// by the originating implementation.
    pub description: Option<String>,

    /// Current map name, if available.
    pub map: Option<String>,

    /// Current game mode, if available.
    pub mode: Option<String>,

    /// Reported server version / build string, if available.
    pub version: Option<String>,

    /// Whether an anti cheat system is reported as enabled.
    ///
    /// `None` indicates the underlying protocol does not expose this field.
    pub anti_cheat: Option<bool>,

    /// Whether the server requires a password to join.
    ///
    /// `None` indicates the underlying protocol does not expose this field.
    pub has_password: Option<bool>,

    /// Maximum player capacity reported by the server.
    pub max_players: u16,

    /// Current number of connected players reported by the server.
    pub current_players: u16,

    /// Optional list of players currently connected to the server.
    ///
    /// Some protocols do not provide player lists.
    /// When not available this will be `None`.
    pub players: Option<Vec<GenericPlayer>>,
}

/// Represents a generic game server with associated additional data.
///
/// This extends [`GenericServer`] with:
/// - typed server specific metadata (`S`), and
/// - a typed player representation (`P`) that can be normalized via [`GenericPlayerExt`].
///
/// Use this type when preserving protocol specific metadata is required.
#[derive(Debug)]
#[cfg_attr(
    feature = "attribute_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "attribute_extended_derive",
    derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)
)]
pub struct GenericServerWithAdditionalData<S, P> {
    /// Server name.
    pub name: String,

    /// Optional server description or MOTD.
    ///
    /// Empty strings received from a protocol may be normalized to `None`
    /// by the originating implementation.
    pub description: Option<String>,

    /// Current map name, if available.
    pub map: Option<String>,

    /// Current game mode, if available.
    pub mode: Option<String>,

    /// Reported server version / build string, if available.
    pub version: Option<String>,

    /// Whether an anti cheat system is reported as enabled.
    ///
    /// `None` indicates the underlying protocol does not expose this field.
    pub anti_cheat: Option<bool>,

    /// Whether the server requires a password to join.
    ///
    /// `None` indicates the underlying protocol does not expose this field.
    pub has_password: Option<bool>,

    /// Maximum player capacity reported by the server.
    pub max_players: u16,

    /// Current number of connected players reported by the server.
    pub current_players: u16,

    /// Optional list of players currently connected to the server.
    ///
    /// Some protocols do not provide player lists.
    /// When not available this will be `None`.
    pub players: Option<Vec<P>>,

    /// Server specific metadata returned by the underlying protocol.
    ///
    /// The structure of this data depends on the originating
    /// game or protocol implementation.
    pub additional_data: S,
}

impl<S, P: GenericPlayerExt> From<GenericServerWithAdditionalData<S, P>> for GenericServer {
    /// Converts a server with additional data into its plain [`GenericServer`]
    /// representation by discarding the server specific metadata and
    /// normalizing players into [`GenericPlayer`].
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
            players: s.players.map(|players| {
                players
                    .into_iter()
                    .map(|p| p.into_generic_player())
                    .collect()
            }),
        }
    }
}

/// Extension trait for types that can be represented as a [`GenericServer`].
///
/// Implementors typically represent protocol or game specific server response
/// structures that can be normalized into:
///
/// - [`GenericServerWithAdditionalData`], preserving full metadata, or
/// - [`GenericServer`], discarding additional metadata.
pub trait GenericServerExt {
    /// The type containing server specific metadata.
    type AdditionalServerData;

    /// The player type returned by the server query.
    type Player: GenericPlayerExt;

    /// Converts `self` into a plain [`GenericServer`].
    ///
    /// This method discards any additional server/player metadata but provides
    /// a concrete generic server representation.
    #[must_use]
    fn into_generic_server(self) -> GenericServer
    where Self: Sized {
        self.into_generic_server_with_additional_data().into()
    }

    /// Converts `self` into a [`GenericServerWithAdditionalData`],
    /// preserving server and player specific metadata.
    #[must_use]
    fn into_generic_server_with_additional_data(
        self,
    ) -> GenericServerWithAdditionalData<Self::AdditionalServerData, Self::Player>;

    /// Converts `self` into a [`GenericServerWithAdditionalData`] where:
    /// - server additional data is stored as a [`serde_json::Value`], and
    /// - players are converted into [`GenericPlayerWithAdditionalData`] with JSON additional data.
    ///
    /// This is useful for exposing the full protocol payload to consumers
    /// without requiring them to depend on the concrete protocol types.
    #[cfg(feature = "attribute_serde")]
    #[must_use]
    fn into_generic_server_with_additional_data_as_json(
        self,
    ) -> GenericServerWithAdditionalData<
        serde_json::Value,
        GenericPlayerWithAdditionalData<serde_json::Value>,
    >
    where
        Self: Sized,
        Self::AdditionalServerData: serde::Serialize,
        <Self::Player as GenericPlayerExt>::AdditionalPlayerData: serde::Serialize, {
        let s = self.into_generic_server_with_additional_data();

        GenericServerWithAdditionalData {
            name: s.name,
            description: s.description,
            map: s.map,
            mode: s.mode,
            version: s.version,
            anti_cheat: s.anti_cheat,
            has_password: s.has_password,
            max_players: s.max_players,
            current_players: s.current_players,
            players: s.players.map(|players| {
                players
                    .into_iter()
                    .map(|p| p.into_generic_player_with_additional_data_as_json())
                    .collect()
            }),
            additional_data: serde_json::to_value(s.additional_data).unwrap(),
        }
    }
}
