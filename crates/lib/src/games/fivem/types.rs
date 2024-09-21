use serde::{Deserialize, Serialize};

use crate::http::{HttpProtocol, HttpSettings};
use crate::ExtraRequestSettings;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "enhancedHostSupport")]
    pub enhanced_host_support: bool,
    pub icon: String,
    #[serde(rename = "requestSteamTicket")]
    pub request_steam_ticket: String,
    pub resources: Vec<String>,
    pub server: String,
    pub vars: Variables,
    pub version: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variables {
    pub banner_connecting: String,
    pub banner_detail: String,
    pub gamename: String,
    pub locale: String,
    pub onesync_enabled: String,
    #[serde(rename = "sv_disableClientReplays")]
    pub sv_disable_client_replays: String,
    #[serde(rename = "sv_enforceGameBuild")]
    pub sv_enforce_game_build: String,
    #[serde(rename = "sv_enhancedHostSupport")]
    pub sv_enhanced_host_support: String,
    pub sv_lan: String,
    #[serde(rename = "sv_licenseKeyToken")]
    pub sv_license_key_token: String,
    #[serde(rename = "sv_maxClients")]
    pub sv_max_clients: String,
    #[serde(rename = "sv_projectDesc")]
    pub sv_project_desc: String,
    #[serde(rename = "sv_projectName")]
    pub sv_project_name: String,
    #[serde(rename = "sv_pureLevel")]
    pub sv_pure_level: String,
    #[serde(rename = "sv_scriptHookAllowed")]
    pub sv_script_hook_allowed: String,
    pub tags: String,
    #[serde(rename = "txAdmin-version")]
    pub tx_admin_version: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    endpoint: String,
    id: i32,
    identifiers: Vec<String>,
    name: String,
    ping: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub info: Info,
    #[serde(default)]
    pub players: Vec<Player>,
}

impl Into<Response> for (Info, Vec<Player>) {
    fn into(self) -> Response {
        Response {
            info: self.0,
            players: self.1,
        }
    }
}

impl Into<Response> for Info {
    fn into(self) -> Response {
        Response {
            info: self,
            players: Vec::new(),
        }
    }
}

/// Extra request settings for FiveM queries.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct FiveMRequestSettings {
    hostname: Option<String>,
}

impl From<ExtraRequestSettings> for FiveMRequestSettings {
    fn from(value: ExtraRequestSettings) -> Self {
        Self {
            hostname: value.hostname,
        }
    }
}

impl From<FiveMRequestSettings> for HttpSettings<String> {
    fn from(value: FiveMRequestSettings) -> Self {
        Self {
            protocol: HttpProtocol::Http,
            hostname: value.hostname,
            headers: Vec::with_capacity(0),
        }
    }
}
