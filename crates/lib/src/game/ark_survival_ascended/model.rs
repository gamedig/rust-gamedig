use {
    serde::Deserialize,
    std::net::{IpAddr, SocketAddr},
};

#[derive(Debug, Clone, Deserialize)]
pub struct Matchmaking {
    #[serde(
        rename = "sessions",
        deserialize_with = "serde_derive_ext::de_single_matchmaking_session"
    )]
    pub session: MatchmakingSession,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MatchmakingSession {
    #[serde(rename = "totalPlayers")]
    pub total_players: u32,
    pub settings: MatchmakingSessionSettings,
    pub attributes: MatchmakingSessionAttributes,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MatchmakingSessionSettings {
    #[serde(rename = "allowInvites")]
    pub allow_invites: bool,

    #[serde(rename = "maxPublicPlayers")]
    pub max_public_players: u32,

    #[serde(rename = "allowJoinInProgress")]
    pub allow_join_in_progress: bool,

    #[serde(rename = "allowJoinViaPresence")]
    pub allow_join_via_presence: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MatchmakingSessionAttributes {
    // Network / Addressing
    #[serde(
        rename = "ADDRESS_s",
        deserialize_with = "serde_derive_ext::de_string_to_ip_addr"
    )]
    pub address: IpAddr,

    #[serde(
        rename = "ADDRESSBOUND_s",
        deserialize_with = "serde_derive_ext::de_string_to_socket_addr"
    )]
    pub address_bound: SocketAddr,

    // Identity / Naming
    #[serde(rename = "MAPNAME_s")]
    pub map_name: String,

    #[serde(rename = "SESSIONNAME_s")]
    pub session_name: String,

    #[serde(rename = "CUSTOMSERVERNAME_s")]
    pub server_name: String,

    // Build / Versioning
    #[serde(
        rename = "BUILDID_s",
        deserialize_with = "serde_derive_ext::de_string_to_u16"
    )]
    pub build_id_major: u16,

    #[serde(
        rename = "MINORBUILDID_s",
        deserialize_with = "serde_derive_ext::de_string_to_u16"
    )]
    pub build_id_minor: u16,

    // Gameplay / Mode
    #[serde(
        rename = "DAYTIME_s",
        deserialize_with = "serde_derive_ext::de_string_to_u32"
    )]
    pub day_time: u32,

    #[serde(
        rename = "ENABLEDMODS_s",
        deserialize_with = "serde_derive_ext::de_comma_num_string_to_vec_u32"
    )]
    pub enabled_mods: Vec<u32>,

    #[serde(
        rename = "SESSIONISPVE_l",
        deserialize_with = "serde_derive_ext::de_num_to_bool"
    )]
    pub session_is_pve: bool,

    #[serde(rename = "SOTFMATCHSTARTED_b")]
    pub sotf_match_started: bool,

    // Server Rules / Permissions
    #[serde(
        rename = "ALLOWDOWNLOADCHARS_l",
        deserialize_with = "serde_derive_ext::de_num_to_bool"
    )]
    pub allow_download_chars: bool,

    #[serde(
        rename = "ALLOWDOWNLOADDINOS_l",
        deserialize_with = "serde_derive_ext::de_num_to_bool"
    )]
    pub allow_download_dinos: bool,

    #[serde(
        rename = "ALLOWDOWNLOADITEMS_l",
        deserialize_with = "serde_derive_ext::de_num_to_bool"
    )]
    pub allow_download_items: bool,

    #[serde(rename = "SERVERPASSWORD_b")]
    pub server_password: bool,

    // Platform / Anti-cheat
    #[serde(
        rename = "SERVERPLATFORMTYPE_s",
        deserialize_with = "serde_derive_ext::de_plus_string_to_vec_string"
    )]
    pub server_platform_type: Vec<String>,

    #[serde(rename = "SERVERUSESBATTLEYE_b")]
    pub server_uses_battleye: bool,

    // Metrics
    #[serde(rename = "EOSSERVERPING_l")]
    pub eos_server_ping: u16,
}

mod serde_derive_ext {
    use {
        super::MatchmakingSession,
        serde::{
            de::Error as _,
            {Deserialize, Deserializer},
        },
        std::net::{IpAddr, SocketAddr},
    };

    pub fn de_single_matchmaking_session<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<MatchmakingSession, D::Error> {
        let mut sessions = Vec::<MatchmakingSession>::deserialize(deserializer)?;

        match sessions.len() {
            1 => Ok(sessions.remove(0)),
            0 => {
                Err(serde::de::Error::custom(
                    "Expected exactly one matchmaking session, got zero",
                ))
            }
            num => {
                Err(serde::de::Error::custom(format!(
                    "Expected exactly one matchmaking session, got {num}"
                )))
            }
        }
    }

    pub fn de_string_to_ip_addr<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<IpAddr, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<IpAddr>().map_err(serde::de::Error::custom)
    }

    pub fn de_string_to_socket_addr<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<SocketAddr, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<SocketAddr>().map_err(serde::de::Error::custom)
    }

    pub fn de_string_to_u16<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u16, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<u16>().map_err(serde::de::Error::custom)
    }

    pub fn de_string_to_u32<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<u32>().map_err(serde::de::Error::custom)
    }

    pub fn de_comma_num_string_to_vec_u32<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<u32>, D::Error> {
        let s = String::deserialize(deserializer)?;

        if s.trim().is_empty() {
            return Ok(Vec::new());
        }

        s.split(',')
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| {
                v.parse::<u32>().map_err(|e| {
                    D::Error::custom(format!("Invalid number in comma separated list: {e}"))
                })
            })
            .collect()
    }

    pub fn de_num_to_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(false),
            1 => Ok(true),

            other => {
                Err(D::Error::custom(format!(
                    "Invalid boolean (expected 0 or 1, got {other})"
                )))
            }
        }
    }

    pub fn de_plus_string_to_vec_string<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<String>, D::Error> {
        let s = String::deserialize(deserializer)?;

        if s.trim().is_empty() {
            return Ok(Vec::new());
        }

        Ok(s.split('+')
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
            .collect())
    }
}
