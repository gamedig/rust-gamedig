use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::http::{HttpProtocol, HttpSettings};
use crate::protocols::types::{CommonPlayer, CommonResponse, GatherToggle};
use crate::ExtraRequestSettings;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "enhancedHostSupport")]
    pub enhanced_host_support: bool,
    pub icon: Option<String>,
    #[serde(rename = "requestSteamTicket")]
    pub request_steam_ticket: String,
    pub resources: Vec<String>,
    pub server: String,
    #[serde(default)]
    pub vars: Variables,
    pub version: i32,
    #[serde(rename = "enforceSteamAuth")]
    pub enforce_steam_auth: Option<bool>,
}

/// Known server-information convars.
///
/// FiveM servers decide which convars to expose, so every known value is
/// optional. Unknown convars are retained in [`Variables::additional`].
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Variables {
    pub banner_connecting: Option<String>,
    pub banner_detail: Option<String>,
    pub gamename: Option<String>,
    pub locale: Option<String>,
    pub onesync_enabled: Option<String>,
    #[serde(rename = "sv_disableClientReplays")]
    pub sv_disable_client_replays: Option<String>,
    #[serde(rename = "sv_enforceGameBuild")]
    pub sv_enforce_game_build: Option<String>,
    #[serde(rename = "sv_enhancedHostSupport")]
    pub sv_enhanced_host_support: Option<String>,
    pub sv_lan: Option<String>,
    #[serde(rename = "sv_licenseKeyToken")]
    pub sv_license_key_token: Option<String>,
    #[serde(rename = "sv_maxClients")]
    pub sv_max_clients: Option<String>,
    #[serde(rename = "sv_projectDesc")]
    pub sv_project_desc: Option<String>,
    #[serde(rename = "sv_projectName")]
    pub sv_project_name: Option<String>,
    #[serde(rename = "sv_pureLevel")]
    pub sv_pure_level: Option<String>,
    #[serde(rename = "sv_scriptHookAllowed")]
    pub sv_script_hook_allowed: Option<String>,
    pub tags: Option<String>,
    #[serde(rename = "txAdmin-version")]
    pub tx_admin_version: Option<String>,
    #[serde(flatten)]
    pub additional: HashMap<String, String>,
}

/// The response returned by `/dynamic.json`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicInfo {
    pub hostname: String,
    pub gametype: String,
    pub mapname: String,
    #[serde(deserialize_with = "deserialize_u32_from_number_or_string")]
    pub clients: u32,
    #[serde(rename = "iv")]
    pub info_version: String,
    #[serde(
        rename = "sv_maxclients",
        deserialize_with = "deserialize_u32_from_number_or_string"
    )]
    pub max_clients: u32,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum NumberOrString {
    Number(u32),
    String(String),
}

fn deserialize_u32_from_number_or_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where D: Deserializer<'de> {
    match NumberOrString::deserialize(deserializer)? {
        NumberOrString::Number(value) => Ok(value),
        NumberOrString::String(value) => value.parse().map_err(serde::de::Error::custom),
    }
}

/// A player returned by `/players.json`.
///
/// Current FXServer versions anonymize these values unless the request uses a
/// valid players token.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub endpoint: String,
    pub id: i32,
    pub identifiers: Vec<String>,
    pub name: String,
    pub ping: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub info: Info,
    pub dynamic: Option<DynamicInfo>,
    /// `None` when player gathering was skipped or the optional request failed.
    pub players: Option<Vec<Player>>,
}

impl From<(Info, Option<DynamicInfo>, Option<Vec<Player>>)> for Response {
    fn from(value: (Info, Option<DynamicInfo>, Option<Vec<Player>>)) -> Self {
        Self {
            info: value.0,
            dynamic: value.1,
            players: value.2,
        }
    }
}

impl CommonPlayer for Player {
    fn as_original(&self) -> crate::protocols::types::GenericPlayer<'_> {
        crate::protocols::types::GenericPlayer::FiveM(self)
    }

    fn name(&self) -> &str { &self.name }
}

impl CommonResponse for Response {
    fn as_original(&self) -> crate::protocols::GenericResponse<'_> { crate::protocols::GenericResponse::FiveM(self) }

    fn name(&self) -> Option<&str> {
        self.dynamic
            .as_ref()
            .map(|dynamic| dynamic.hostname.as_str())
            .filter(|name| !name.is_empty())
            .or_else(|| {
                self.info
                    .vars
                    .sv_project_name
                    .as_deref()
                    .filter(|name| !name.is_empty())
            })
    }

    fn description(&self) -> Option<&str> {
        self.info
            .vars
            .sv_project_desc
            .as_deref()
            .filter(|description| !description.is_empty())
    }

    fn game_mode(&self) -> Option<&str> {
        self.dynamic
            .as_ref()
            .map(|dynamic| dynamic.gametype.as_str())
            .filter(|game_mode| !game_mode.is_empty())
            .or_else(|| {
                self.info
                    .vars
                    .gamename
                    .as_deref()
                    .filter(|game_mode| !game_mode.is_empty())
            })
    }

    fn game_version(&self) -> Option<&str> { Some(self.info.server.as_str()).filter(|version| !version.is_empty()) }

    fn map(&self) -> Option<&str> {
        self.dynamic
            .as_ref()
            .map(|dynamic| dynamic.mapname.as_str())
            .filter(|map| !map.is_empty())
    }

    fn players_online(&self) -> u32 {
        self.dynamic
            .as_ref()
            .map(|dynamic| dynamic.clients)
            .or_else(|| self.players.as_ref().map(|players| players.len() as u32))
            .unwrap_or_default()
    }

    fn players_maximum(&self) -> u32 {
        self.dynamic
            .as_ref()
            .map(|dynamic| dynamic.max_clients)
            .or_else(|| {
                self.info
                    .vars
                    .sv_max_clients
                    .as_deref()
                    .and_then(|maximum| maximum.parse().ok())
            })
            .unwrap_or_default()
    }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        self.players
            .as_ref()
            .map(|players| players.iter().map(|player| player as _).collect())
    }
}

/// Extra request settings for FiveM queries.
#[derive(Clone, Eq, PartialEq)]
pub struct FiveMRequestSettings {
    hostname: Option<String>,
    gather_players: GatherToggle,
    players_token: Option<String>,
}

impl FiveMRequestSettings {
    /// Default to a best-effort player query without authentication.
    pub const fn default() -> Self {
        Self {
            hostname: None,
            gather_players: GatherToggle::Try,
            players_token: None,
        }
    }

    /// Override the HTTP host header.
    pub fn set_hostname(mut self, hostname: String) -> Self {
        self.hostname = Some(hostname);
        self
    }

    /// Choose whether player information should be requested.
    pub const fn set_gather_players(mut self, gather_players: GatherToggle) -> Self {
        self.gather_players = gather_players;
        self
    }

    /// Authenticate the `/players.json` request with `X-Players-Token`.
    pub fn set_players_token(mut self, players_token: String) -> Self {
        self.players_token = Some(players_token);
        self
    }

    pub(crate) const fn gather_players(&self) -> GatherToggle { self.gather_players }

    pub(crate) fn players_token(&self) -> Option<&str> { self.players_token.as_deref() }
}

impl Default for FiveMRequestSettings {
    fn default() -> Self { Self::default() }
}

impl fmt::Debug for FiveMRequestSettings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FiveMRequestSettings")
            .field("hostname", &self.hostname)
            .field("gather_players", &self.gather_players)
            .field(
                "players_token",
                &self.players_token.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

impl From<ExtraRequestSettings> for FiveMRequestSettings {
    fn from(value: ExtraRequestSettings) -> Self {
        Self {
            hostname: value.hostname,
            gather_players: value.gather_players.unwrap_or(GatherToggle::Try),
            players_token: None,
        }
    }
}

impl From<FiveMRequestSettings> for HttpSettings<String> {
    fn from(value: FiveMRequestSettings) -> Self {
        Self {
            protocol: HttpProtocol::Http,
            hostname: value.hostname,
            headers: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPARSE_INFO: &str = r#"{
        "enhancedHostSupport": true,
        "requestSteamTicket": "unset",
        "resources": ["hardcap"],
        "server": "FXServer-master v1.0.0.12345",
        "vars": {
            "Discord": "https://example.invalid",
            "sv_maxClients": "48",
            "sv_projectDesc": "A test server",
            "sv_projectName": "Test FiveM",
            "gamename": "gta5"
        },
        "version": 12345
    }"#;

    fn sparse_info() -> Info { serde_json::from_str(SPARSE_INFO).expect("fixture must be valid") }

    #[test]
    fn parses_sparse_info_and_retains_unknown_variables() {
        let info = sparse_info();

        assert_eq!(info.icon, None);
        assert_eq!(info.enforce_steam_auth, None);
        assert_eq!(info.vars.tags, None);
        assert_eq!(info.vars.banner_connecting, None);
        assert_eq!(info.vars.sv_max_clients.as_deref(), Some("48"));
        assert_eq!(
            info.vars.additional.get("Discord").map(String::as_str),
            Some("https://example.invalid")
        );
    }

    #[test]
    fn parses_dynamic_counts_from_strings_or_numbers() {
        let dynamic: DynamicInfo = serde_json::from_str(
            r#"{
                "hostname": "Test FiveM",
                "gametype": "Freeroam",
                "mapname": "San Andreas",
                "clients": 12,
                "iv": "12345",
                "sv_maxclients": "48"
            }"#,
        )
        .expect("fixture must be valid");

        assert_eq!(dynamic.clients, 12);
        assert_eq!(dynamic.max_clients, 48);
    }

    #[test]
    fn common_response_uses_dynamic_metadata_without_player_details() {
        let response = Response {
            info: sparse_info(),
            dynamic: Some(DynamicInfo {
                hostname: "Dynamic FiveM".to_owned(),
                gametype: "Freeroam".to_owned(),
                mapname: "San Andreas".to_owned(),
                clients: 12,
                info_version: "12345".to_owned(),
                max_clients: 64,
            }),
            players: None,
        };

        assert_eq!(response.name(), Some("Dynamic FiveM"));
        assert_eq!(response.description(), Some("A test server"));
        assert_eq!(response.game_mode(), Some("Freeroam"));
        assert_eq!(
            response.game_version(),
            Some("FXServer-master v1.0.0.12345")
        );
        assert_eq!(response.map(), Some("San Andreas"));
        assert_eq!(response.players_online(), 12);
        assert_eq!(response.players_maximum(), 64);
        assert!(response.players().is_none());
    }

    #[test]
    fn common_response_falls_back_to_info_and_players() {
        let response = Response {
            info: sparse_info(),
            dynamic: None,
            players: Some(vec![Player {
                endpoint: "127.0.0.1".to_owned(),
                id: 0,
                identifiers: Vec::new(),
                name: "Player".to_owned(),
                ping: 0,
            }]),
        };

        assert_eq!(response.name(), Some("Test FiveM"));
        assert_eq!(response.game_mode(), Some("gta5"));
        assert_eq!(response.players_online(), 1);
        assert_eq!(response.players_maximum(), 48);
        assert_eq!(response.players().map(|players| players.len()), Some(1));
    }

    #[test]
    fn request_settings_redact_the_players_token() {
        let settings = FiveMRequestSettings::default()
            .set_hostname("play.example.invalid".to_owned())
            .set_gather_players(GatherToggle::Enforce)
            .set_players_token("super-secret".to_owned());
        let debug = format!("{settings:?}");

        assert!(!debug.contains("super-secret"));
        assert!(debug.contains("[REDACTED]"));
    }
}
