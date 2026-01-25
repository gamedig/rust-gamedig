#[derive(Debug, thiserror::Error)]
pub enum EpicApiClientError {
    #[error("[GameDig]::[EpicAPI::HTTP_CLIENT_INIT]: Failed to initialize HTTP client")]
    HttpClientInit,

    #[error("[GameDig]::[EpicAPI::OAUTH_TOKEN_REQUEST]: Failed to request OAuth token")]
    OAuthTokenRequest,

    #[error("[GameDig]::[EpicAPI::MATCHMAKING_REQUEST]: Failed to request matchmaking")]
    MatchmakingRequest,
}