use crate::http::HttpClient;
use crate::protocols::epic::Response;
use crate::GDErrorKind::{JsonParse, PacketBad};
use crate::GDResult;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::Deserialize;
use serde_json::Value;
use std::any::Any;

const EPIC_API_ENDPOINT: &'static str = "https://api.epicgames.dev";

pub struct EpicProtocol {
    client: HttpClient,
    deployment: String,
    id: String,
    secret: String,
}

#[derive(Deserialize)]
pub struct ClientTokenResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
struct QueryResponse {
    sessions: Value,
}

macro_rules! extract_field {
    ($value:expr, $fields:expr, $map_func:expr) => {
        $fields
            .iter()
            .fold(Some(&$value), |acc, &key| acc.and_then(|val| val.get(key)))
            .map($map_func)
            .ok_or(PacketBad.context("Missing field!"))?
            .ok_or(PacketBad.context("Field is not parsable."))?
    };
}

impl EpicProtocol {
    pub fn new(deployment: String, id: String, secret: String) -> GDResult<Self> {
        Ok(Self {
            client: HttpClient::from_url(EPIC_API_ENDPOINT, &None, None)?,
            deployment,
            id,
            secret,
        })
    }

    pub fn auth_by_external(&self) -> GDResult<String> { Ok(String::new()) }

    pub fn auth_by_client(&mut self) -> GDResult<String> {
        let body = [
            ("grant_type", "client_credentials"),
            ("deployment_id", self.deployment.as_str()),
        ];

        let auth_format = format!("{}:{}", self.id, self.secret);
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

    pub fn query_raw(&mut self, address: String, port: u16) -> GDResult<Value> {
        let body = format!(
            "{{\"criteria\":[{{\"key\":\"attributes.ADDRESS_s\",\"op\":\"EQUAL\",\"value\":\"{}\"}}]}}",
            address
        );
        let body = serde_json::from_str::<Value>(body.as_str()).map_err(|e| JsonParse.context(e))?;

        let token = self.auth_by_client()?;
        let authorization = format!("Bearer {}", token);
        let headers = [
            ("Content-Type", "application/json"),
            ("Accept", "application/json"),
            ("Authorization", authorization.as_str()),
        ];

        let url = format!("/matchmaking/v1/{}/filter", self.deployment);
        let response: QueryResponse = self.client.post_json(url.as_str(), Some(&headers), body)?;

        if let Value::Array(sessions) = response.sessions {
            if sessions.is_empty() {
                return Err(PacketBad.context("No servers provided."));
            }

            for session in sessions.into_iter() {
                let attributes = session
                    .get("attributes")
                    .ok_or(PacketBad.context("Expected attributes field missing in sessions."))?;
                if attributes
                    .get("ADDRESSBOUND_s")
                    .and_then(Value::as_str)
                    .map_or(false, |v| {
                        v.contains(&address) || v.contains(&port.to_string())
                    })
                    || attributes
                        .get("ADDRESS_s")
                        .and_then(Value::as_str)
                        .map_or(false, |v| v.contains(&address))
                {
                    return Ok(session);
                }
            }

            return Err(PacketBad.context("Servers were provided but the specified one couldn't be find amonst them."));
        }

        Err(PacketBad.context("Expected session field to be an array."))
    }

    pub fn query(&mut self, address: String, port: u16) -> GDResult<Response> {
        let value = self.query_raw(address, port)?;

        Ok(Response {
            name: extract_field!(value, ["attributes", "CUSTOMSERVERNAME_s"], Value::as_str).to_string(),
            map: extract_field!(value, ["attributes", "MAPNAME_s"], Value::as_str).to_string(),
            has_password: extract_field!(value, ["attributes", "SERVERPASSWORD_b"], Value::as_bool),
            players_online: extract_field!(value, ["totalPlayers"], Value::as_u64) as u32,
            players_maxmimum: extract_field!(value, ["settings", "maxPublicPlayers"], Value::as_u64) as u32,
            players: vec![],
        })
    }
}
