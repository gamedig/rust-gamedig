use crate::http::HttpClient;
use crate::protocols::epic::Response;
use crate::GDErrorKind::{JsonParse, PacketBad};
use crate::{GDResult, TimeoutSettings};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;
use serde_json::Value;
use std::net::SocketAddr;

const EPIC_API_ENDPOINT: &str = "https://api.epicgames.dev";

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credentials {
    #[cfg_attr(feature = "serde", serde(skip_deserializing, skip_serializing))]
    pub deployment: &'static str,
    #[cfg_attr(feature = "serde", serde(skip_deserializing, skip_serializing))]
    pub id: &'static str,
    #[cfg_attr(feature = "serde", serde(skip_deserializing, skip_serializing))]
    pub secret: &'static str,
    pub auth_by_external: bool,
}

pub struct EpicProtocol {
    client: HttpClient,
    credentials: Credentials,
}

#[derive(Deserialize)]
struct ClientTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct QueryResponse {
    sessions: Value,
}

macro_rules! extract_optional_field {
    ($value:expr, $fields:expr, $map_func:expr) => {
        $fields
            .iter()
            .fold(Some(&$value), |acc, &key| acc.and_then(|val| val.get(key)))
            .map($map_func)
            .flatten()
    };
}

macro_rules! extract_field {
    ($value:expr, $fields:expr, $map_func:expr) => {
        extract_optional_field!($value, $fields, $map_func)
            .ok_or(PacketBad.context("Field is missing or is not parsable."))?
    };
}

impl EpicProtocol {
    pub fn new(credentials: Credentials, timeout_settings: TimeoutSettings) -> GDResult<Self> {
        Ok(Self {
            client: HttpClient::from_url(EPIC_API_ENDPOINT, &Some(timeout_settings), None)?,
            credentials,
        })
    }

    pub fn auth_by_external(&self) -> GDResult<String> { Ok(String::new()) }

    pub fn auth_by_client(&mut self) -> GDResult<String> {
        let body = [
            ("grant_type", "client_credentials"),
            ("deployment_id", self.credentials.deployment),
        ];

        let auth_format = format!("{}:{}", self.credentials.id, self.credentials.secret);
        let auth_base = BASE64_STANDARD.encode(auth_format);
        let auth = format!("Basic {}", auth_base.as_str());
        let authorization = auth.as_str();

        let headers = [
            ("Authorization", authorization),
            ("Content-Type", "application/x-www-form-urlencoded"),
        ];

        let response =
            self.client
                .post_json_with_form::<ClientTokenResponse>("/auth/v1/oauth/token", Some(&headers), &body)?;
        Ok(response.access_token)
    }

    pub fn query_raw(&mut self, address: &SocketAddr) -> GDResult<Value> {
        let port = address.port();
        let address = address.ip().to_string();

        let body = format!(
            "{{\"criteria\":[{{\"key\":\"attributes.ADDRESS_s\",\"op\":\"EQUAL\",\"value\":\"{}\"}}]}}",
            address
        );
        let body = serde_json::from_str::<Value>(body.as_str()).map_err(|e| JsonParse.context(e))?;

        let token = if self.credentials.auth_by_external {
            self.auth_by_external()?
        } else {
            self.auth_by_client()?
        };
        let authorization = format!("Bearer {}", token);
        let headers = [
            ("Content-Type", "application/json"),
            ("Accept", "application/json"),
            ("Authorization", authorization.as_str()),
        ];

        let url = format!("/matchmaking/v1/{}/filter", self.credentials.deployment);
        let response: QueryResponse = self.client.post_json(url.as_str(), Some(&headers), body)?;

        if let Value::Array(sessions) = response.sessions {
            if sessions.is_empty() {
                return Err(PacketBad.context("No servers provided."));
            }

            for session in sessions.into_iter() {
                let attributes = session
                    .get("attributes")
                    .ok_or(PacketBad.context("Expected attributes field missing in sessions."))?;

                let address_match = attributes
                    .get("ADDRESSBOUND_s")
                    .and_then(Value::as_str)
                    .map_or(false, |v| v == address || v == format!("0.0.0.0:{}", port))
                    || attributes
                        .get("ADDRESS_s")
                        .and_then(Value::as_str)
                        .map_or(false, |v| v == address || v == format!("0.0.0.0:{}", port));

                if address_match {
                    return Ok(session);
                }
            }

            return Err(
                PacketBad.context("Servers were provided but the specified one couldn't be found amongst them.")
            );
        }

        Err(PacketBad.context("Expected session field to be an array."))
    }

    pub fn query(&mut self, address: &SocketAddr) -> GDResult<Response> {
        let value = self.query_raw(address)?;

        let build_version = extract_optional_field!(value, ["attributes", "BUILDID_s"], Value::as_str);
        let minor_version = extract_optional_field!(value, ["attributes", "MINORBUILDID_s"], Value::as_str);

        let game_version = match (build_version, minor_version) {
            (Some(b), Some(m)) => Some(format!("{b}.{m}")),
            _ => None,
        };

        Ok(Response {
            name: extract_field!(value, ["attributes", "CUSTOMSERVERNAME_s"], Value::as_str).to_string(),
            map: extract_field!(value, ["attributes", "MAPNAME_s"], Value::as_str).to_string(),
            has_password: extract_field!(value, ["attributes", "SERVERPASSWORD_b"], Value::as_bool),
            players_online: extract_field!(value, ["totalPlayers"], Value::as_u64) as u32,
            players_maxmimum: extract_field!(value, ["settings", "maxPublicPlayers"], Value::as_u64) as u32,
            players: vec![],
            game_version,
            raw: value,
        })
    }
}

pub fn query(credentials: Credentials, address: &SocketAddr) -> GDResult<Response> {
    query_with_timeout(credentials, address, None)
}

pub fn query_with_timeout(
    credentials: Credentials,
    address: &SocketAddr,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response> {
    let mut client = EpicProtocol::new(credentials, timeout_settings.unwrap_or_default())?;
    client.query(address)
}
