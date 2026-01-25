mod client;
mod error;
mod ext;
mod model;

// Public
pub use {
    client::ArkSurvivalAscendedClient,
    error::ArkSurvivalAscendedClientError,
    model::{MatchmakingSession, MatchmakingSessionAttributes, MatchmakingSessionSettings},
};

// Private
pub(crate) use model::Matchmaking;
