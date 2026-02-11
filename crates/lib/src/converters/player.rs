/// Represents a generic player with a name and associated arbitrary data.
#[derive(Debug, Clone)]
pub struct GenericPlayer {
    pub name: String,
    
    pub additional_data: Option<super::GenericDataMap>,
}

/// Extension trait for types that can be represented as a [`GenericPlayer`].
pub trait GenericPlayerExt {
    /// Returns a [`GenericPlayer`] representation of `self`.
    #[must_use]
    fn into_generic_player(self) -> GenericPlayer;
}
