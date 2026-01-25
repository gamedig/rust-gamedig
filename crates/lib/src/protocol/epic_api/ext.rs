use {
    super::EpicApiClientError,
    crate::converters::{ErrorCategory, ErrorCategoryExt},
};

impl ErrorCategoryExt for EpicApiClientError {
    fn category(&self) -> ErrorCategory {
        match self {
            EpicApiClientError::HttpClientInit => ErrorCategory::Client,
            EpicApiClientError::OAuthTokenRequest => ErrorCategory::Networking,
            EpicApiClientError::MatchmakingRequest => ErrorCategory::Networking,
        }
    }
}
