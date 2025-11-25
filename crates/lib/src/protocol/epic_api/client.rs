use {
    super::model::{Credentials, FilteredServers, OAuthToken, RoutingScope},
    crate::{
        core::{HttpClient, Payload},
        error::Result,
    },
    base64::{Engine, prelude::BASE64_STANDARD},
    serde_json::json,
    std::{net::SocketAddr, time::Duration},
};

pub struct EpicApiClient {
    net: HttpClient,
    credentials: Credentials,
    authentication: Option<OAuthToken>,
}

#[maybe_async::maybe_async]
impl EpicApiClient {
    pub async fn new(credentials: Credentials) -> Result<Self> {
        Ok(Self {
            net: HttpClient::new(Duration::from_secs(10)).await?,
            credentials,
            authentication: None,
        })
    }

    pub async fn new_with_timeout(credentials: Credentials, timeout: Duration) -> Result<Self> {
        Ok(Self {
            net: HttpClient::new(timeout).await?,
            credentials,
            authentication: None,
        })
    }

    async fn authenticate(&mut self) -> Result<()> {
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
                    Some(&[("Authorization", auth_header_value.as_str())]),
                    Some(Payload::Form(&[
                        ("grant_type", "client_credentials"),
                        ("deployment_id", &self.credentials.deployment),
                    ])),
                )
                .await?,
        );

        Ok(())
    }

    pub async fn query(&mut self, addr: &SocketAddr) -> Result<FilteredServers> {
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
            // safe because we just authenticated above
            self.authentication.as_ref().unwrap().access_token.as_str()
        );

        let filtered = self
            .net
            .post::<FilteredServers>(
                &url,
                None,
                Some(&[("Authorization", &auth_token)]),
                Some(Payload::Json(&json!({
                    "criteria": [
                        {
                            "key": "attributes.ADDRESS_s",
                            "op": "EQUAL",
                            "value": addr.ip().to_string(),
                        }
                    ]
                }))),
            )
            .await?;

        // todo: parse filtered into Server structs
        Ok(filtered)
    }
}
