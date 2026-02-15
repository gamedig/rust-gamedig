/// Represents a generic player.
#[derive(Debug, Clone)]
pub struct GenericPlayer {
    pub name: String,
}

/// Represents a generic player with associated additional data.
#[derive(Debug, Clone)]
pub struct GenericPlayerWithAdditionalData<P> {
    pub name: String,

    pub additional_data: P,
}

impl<P> From<GenericPlayerWithAdditionalData<P>> for GenericPlayer {
    fn from(p: GenericPlayerWithAdditionalData<P>) -> Self { GenericPlayer { name: p.name } }
}

/// Extension trait for types that can be represented as a [`GenericPlayer`] or [`GenericPlayerWithAdditionalData`].
pub trait GenericPlayerExt {
    type AdditionalPlayerData;

    /// Returns a [`GenericPlayer`] representation of `self`.
    #[must_use]
    fn into_generic_player(self) -> GenericPlayer { self.into_generic_player_with_data().into() }

    /// Returns a [`GenericPlayerWithAdditionalData`] representation of `self`.
    #[must_use]
    fn into_generic_player_with_data(
        self,
    ) -> GenericPlayerWithAdditionalData<Self::AdditionalPlayerData>;
}
