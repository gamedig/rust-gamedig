use {
    super::model::{Credentials, OAuthToken, RoutingScope},
    crate::core::{
        HttpClient,
        Payload,
        error::{Report, ResultExt},
    },
    base64::{Engine, prelude::BASE64_STANDARD},
    serde::de::DeserializeOwned,
    serde_json::json,
    std::{net::SocketAddr, time::Duration},
};

#[derive(Debug, thiserror::Error)]
pub enum EpicApiClientError {
    #[error("[GameDig]::[EpicAPI::HTTP_CLIENT_INIT]: Failed to initialize HTTP client")]
    HttpClientInit,

    #[error("[GameDig]::[EpicAPI::OAUTH_TOKEN_REQUEST]: Failed to request OAuth token")]
    OAuthTokenRequest,

    #[error("[GameDig]::[EpicAPI::MATCHMAKING_REQUEST]: Failed to request matchmaking")]
    MatchmakingRequest,
}

pub struct EpicApiClient {
    net: HttpClient,
    credentials: Credentials,
    authentication: Option<OAuthToken>,
}

#[maybe_async::maybe_async]
impl EpicApiClient {
    pub async fn new(credentials: Credentials) -> Result<Self, Report<EpicApiClientError>> {
        Ok(Self {
            net: HttpClient::new(None)
                .await
                .change_context(EpicApiClientError::HttpClientInit)?,

            credentials,
            authentication: None,
        })
    }

    pub async fn new_with_timeout(
        credentials: Credentials,
        timeout: Duration,
    ) -> Result<Self, Report<EpicApiClientError>> {
        Ok(Self {
            net: HttpClient::new(Some(timeout))
                .await
                .change_context(EpicApiClientError::HttpClientInit)?,

            credentials,
            authentication: None,
        })
    }

    async fn authenticate(&mut self) -> Result<(), Report<EpicApiClientError>> {
        if self
            .authentication
            .as_ref()
            .is_some_and(|token| token.is_valid())
        {
            return Ok(());
        }

        let auth_header_value = format!(
            "Basic {}",
            BASE64_STANDARD.encode(format!(
                "{}:{}",
                self.credentials.id, self.credentials.secret
            ))
        );

        self.authentication = Some(
            self.net
                .post::<OAuthToken>(
                    "https://api.epicgames.dev/auth/v1/oauth/token",
                    None,
                    Some(&[("Authorization", &auth_header_value)]),
                    Some(Payload::Form(&[
                        ("grant_type", "client_credentials"),
                        ("deployment_id", self.credentials.deployment),
                    ])),
                )
                .await
                .change_context(EpicApiClientError::OAuthTokenRequest)?,
        );

        Ok(())
    }

    pub async fn query_as<T: DeserializeOwned>(
        &mut self,
        addr: &SocketAddr,
    ) -> Result<T, Report<EpicApiClientError>> {
        self.authenticate().await?;

        let url = format!(
            "https://api.epicgames.dev{}/matchmaking/v1/{}/filter",
            match self.credentials.routing_scope {
                RoutingScope::Default => "",
                RoutingScope::Wildcard => "/wildcard",
            },
            self.credentials.deployment,
        );

        let auth_token = format!(
            "Bearer {}",
            // safe unwrap because we just authenticated above
            self.authentication.as_ref().unwrap().access_token
        );

        Ok(self
            .net
            .post::<T>(
                &url,
                None,
                Some(&[("Authorization", &auth_token)]),
                Some(Payload::Json(&json!({
                    "criteria": [
                        {
                            "key": "attributes.ADDRESSBOUND_s",
                            "op": "EQUAL",
                            "value": addr.to_string(),
                        }
                    ]
                }))),
            )
            .await
            .change_context(EpicApiClientError::MatchmakingRequest)?)
    }
}
