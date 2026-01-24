#[derive(Debug, thiserror::Error)]
pub enum ArkSurvivalAscendedClientError {
    #[error("[GameDig]::[ArkSurvivalAscended::INIT]: Failed to initialize protocol client")]
    Init,

    #[error(
        "[GameDig]::[ArkSurvivalAscended::MATCHMAKING_SESSION]: Failed to query matchmaking \
         session"
    )]
    MatchmakingSession,
}
