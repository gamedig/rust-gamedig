/// Represents a generic player with a name and associated arbitrary data.
#[derive(Debug, Clone)]
pub struct GenericPlayer {
    pub name: String,
    pub data: Option<super::GenericDataMap>,
}

/// Extension trait for types that can be represented as a [`GenericPlayer`].
pub trait GenericPlayerExt {
    /// Returns a [`GenericPlayer`] representation of `self`.
    #[must_use]
    fn as_generic_player(&self) -> GenericPlayer;
}
