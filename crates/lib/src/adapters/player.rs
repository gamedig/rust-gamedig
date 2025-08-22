use super::data::GenericDataHashMap;

/// Represents a generic player with a name and associated arbitrary data.
#[derive(Debug, Clone)]
pub struct GenericPlayer {
    pub name: String,
    pub data: Option<GenericDataHashMap>,
}

/// A trait for converting player structs into a [`GenericPlayer`].
pub trait IntoGenericPlayer: Sized {
    fn into_generic_player(&self) -> GenericPlayer;
}
