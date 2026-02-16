/// Represents a generic, game agnostic player.
///
/// This type contains only the normalized fields that can be shared
/// across different game protocols and implementations.
#[derive(Debug)]
#[cfg_attr(
    feature = "attribute_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "attribute_extended_derive",
    derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)
)]
pub struct GenericPlayer {
    /// Identifier for the player within the current server query.
    ///
    /// This represents the player’s index in the server player list.
    ///
    /// This value is not guaranteed to be stable and may change
    /// between queries as players join, leave, or the server
    /// reorders its internal player list.
    pub id: u16,

    /// The player’s display name, if available.
    ///
    /// Empty strings received from the protocol are normalized to `None`.
    pub name: Option<String>,
}

/// Represents a generic player with additional, game specific metadata.
///
/// This extends [`GenericPlayer`] with typed protocol or game specific data.
///
/// Use this type when preserving full player metadata is required.
#[derive(Debug)]
#[cfg_attr(
    feature = "attribute_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    feature = "attribute_extended_derive",
    derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)
)]
pub struct GenericPlayerWithAdditionalData<P> {
    /// Unique identifier for the player within the server context.
    ///
    /// In most cases this represents:
    /// - the player’s index in the server player list, or
    /// - a protocol specific player identifier.
    ///
    /// This field should be treated as the canonical identity of the player.
    pub id: u16,

    /// The player’s display name, if available.
    ///
    /// Empty strings received from the protocol are normalized to `None`.
    pub name: Option<String>,

    /// Game specific player metadata.
    ///
    /// The structure data depends on the
    /// originating game or protocol implementation.
    pub additional_data: P,
}

impl<P> From<GenericPlayerWithAdditionalData<P>> for GenericPlayer {
    /// Converts a player with additional data into its plain
    /// [`GenericPlayer`] representation by discarding the
    /// additional metadata.
    fn from(p: GenericPlayerWithAdditionalData<P>) -> Self {
        GenericPlayer {
            id: p.id,
            name: p.name,
        }
    }
}

/// Extension trait for converting player data into generic representations
///
/// Implementors typically represent protocol or game specific player
/// structures that can be normalized into:
///
/// - [`GenericPlayerWithAdditionalData`], preserving full metadata, or
/// - [`GenericPlayer`], discarding additional metadata.
pub trait GenericPlayerExt {
    /// The type containing player specific metadata.
    type AdditionalPlayerData;

    /// Converts `self` into a plain [`GenericPlayer`].
    ///
    /// This method discards any additional metadata but provides
    /// a concrete generic player representation.
    #[must_use]
    fn into_generic_player(self) -> GenericPlayer
    where Self: Sized {
        self.into_generic_player_with_additional_data().into()
    }

    /// Converts `self` into a [`GenericPlayerWithAdditionalData`],
    /// preserving player specific metadata.
    #[must_use]
    fn into_generic_player_with_additional_data(
        self,
    ) -> GenericPlayerWithAdditionalData<Self::AdditionalPlayerData>;

    /// Converts `self` into a [`GenericPlayerWithAdditionalData`] where the additional
    /// data is stored as a [`serde_json::Value`].
    #[cfg(feature = "attribute_serde")]
    #[must_use]
    fn into_generic_player_with_additional_data_as_json(
        self,
    ) -> GenericPlayerWithAdditionalData<serde_json::Value>
    where
        Self: Sized,
        Self::AdditionalPlayerData: serde::Serialize, {
        let p = self.into_generic_player_with_additional_data();

        GenericPlayerWithAdditionalData {
            id: p.id,
            name: p.name,
            additional_data: serde_json::to_value(p.additional_data).unwrap(),
        }
    }
}
