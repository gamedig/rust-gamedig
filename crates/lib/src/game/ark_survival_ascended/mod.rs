mod client;
mod error;
mod model;

#[cfg(feature = "ext_converters")]
mod ext;

// Public
pub use {
    client::ArkSurvivalAscendedClient,
    error::ArkSurvivalAscendedClientError,
    model::{MatchmakingSession, MatchmakingSessionAttributes, MatchmakingSessionSettings},
};

// Private
pub(crate) use model::Matchmaking;
