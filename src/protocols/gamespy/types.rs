pub mod one {
    use std::collections::HashMap;

    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};

    /// A player’s details.
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Player {
        pub name: String,
        pub team: u8,
        /// The ping from the server's perspective.
        pub ping: u16,
        pub face: String,
        pub skin: String,
        pub mesh: String,
        pub frags: u32,
        pub deaths: Option<u32>,
        pub health: Option<u32>,
        pub secret: bool,
    }

    /// A query response.
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Response {
        pub name: String,
        pub map: String,
        pub map_title: Option<String>,
        pub admin_contact: Option<String>,
        pub admin_name: Option<String>,
        pub has_password: bool,
        pub game_type: String,
        pub game_version: String,
        pub players_maximum: usize,
        pub players_online: usize,
        pub players_minimum: u8,
        pub players: Vec<Player>,
        pub tournament: bool,
        pub unused_entries: HashMap<String, String>,
    }
}

pub mod two {
    pub(crate) struct RequestPacket {
        /// The header is a 64-bit signed integer, but we only need the
        /// first 16 bits as the header should always be
        /// `0xFEFD`.
        pub(crate) header: u16,
        /// The delimiter, which is always `0x00`.
        pub(crate) delimiter: u8,
        /// The ping value, This can be anything you want, use it to make
        /// sure the response is valid.
        pub(crate) ping_value: [u8; 4],
        /// Whether to return the server info and rules.
        /// `0x00` = No
        /// `0xFF` = Yes
        pub(crate) server_info_and_rules: u8,
        /// Whether to return the player info.
        /// `0x00` = No
        /// `0xFF` = Yes
        pub(crate) player_info: u8,
        /// Whether to return the team info.
        /// `0x00` = No
        /// `0xFF` = Yes
        pub(crate) team_info: u8,
    }

    impl RequestPacket {
        /// Converts the request packet to a byte array of 10 bytes.
        /// This is the format that the server expects.
        #[allow(dead_code)]
        const fn to_u8_array(&self) -> [u8; 10] {
            let header_byte: [u8; 2] = self.header.to_le_bytes();

            [
                header_byte[0],
                header_byte[1],
                self.delimiter,
                self.ping_value[0],
                self.ping_value[1],
                self.ping_value[2],
                self.ping_value[3],
                self.server_info_and_rules,
                self.player_info,
                self.team_info,
            ]
        }
    }

    pub(crate) const REQUEST_PACKET_BYTES: [u8; 10] = RequestPacket {
        header: 0xFEFD,
        delimiter: 0x00,
        // The ping value is referred to as "CORY" because it was originally documented by Cory,
        // who created a C library a long time ago. His documentation has since been adopted by many
        // other libraries and is still in use today, as it provides valuable information.
        ping_value: [b'C', b'O', b'R', b'Y'],
        server_info_and_rules: 0xFF,
        player_info: 0xFF,
        team_info: 0xFF,
    }
    .to_u8_array();

    /// A Flag is a representation of a boolean value from a string.
    #[derive(Debug, PartialEq)]
    pub enum Flag {
        Enabled,
        Disabled,
        Unknown,
    }

    impl Flag {
        pub(crate) fn from_str(s: &str) -> Self {
            match s.trim().to_lowercase().as_str() {
                "yes" | "on" | "1" => Flag::Enabled,
                "no" | "off" | "0" => Flag::Disabled,
                _ => Flag::Unknown,
            }
        }

        pub fn to_bool(&self) -> bool {
            match self {
                Flag::Enabled => true,
                Flag::Disabled => false,
                Flag::Unknown => false,
            }
        }
    }

    pub mod server_info {
        use super::Flag;

        #[derive(Debug, PartialEq)]
        pub struct ServerConnection {
            pub hostname: String,
            pub hostport: u16,
            pub password: Flag,
        }

        #[derive(Debug, PartialEq)]
        pub struct GameInfo {
            pub map_name: String,
            pub game_type: String,
            pub num_players: u8,
            pub max_players: u8,
            pub game_mode: String,
            pub game_version: String,
            pub status: u8,
            pub game_id: String,
            pub map_id: String,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct GameplaySettings {
            pub punkbuster: Option<Flag>,
            pub time_limit: Option<String>,
            pub num_rounds: Option<u8>,
            pub spawn_wave_time: Option<u8>,
            pub spawn_delay: Option<u8>,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct FriendlyFireSettings {
            pub soldier_friendly_fire: Option<Flag>,
            pub vehicle_friendly_fire: Option<Flag>,
            pub soldier_friendly_fire_on_splash: Option<String>,
            pub vehicle_friendly_fire_on_splash: Option<String>,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct TeamSettings {
            pub us_team_ratio: Option<u8>,
            pub nva_team_ratio: Option<u8>,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct CameraSettings {
            pub name_tag_distance: Option<u16>,
            pub name_tag_distance_scope: Option<u16>,
            pub allow_nose_cam: Option<String>,
            pub external_view: Option<String>,
            pub free_camera: Option<Flag>,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct GameStartSettings {
            pub game_start_delay: Option<u8>,
            pub ticket_ratio: Option<String>,
            pub kickback: Option<String>,
            pub kickback_on_splash: Option<String>,
            pub auto_balance: Option<Flag>,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct ServerConfig {
            pub dedicated: Option<Flag>,
            pub cpu: Option<usize>,
            // ! Type is unknown, set as string as its unknown if it's a number or a percentage
            pub bot_skill: Option<String>,
            pub reserved_slots: Option<u16>,
            // ! Type is unknown (probably a key-value pair)
            pub active_mods: Option<String>,
        }

        #[derive(Debug, PartialEq)]
        pub struct ServerInfo {
            pub connection: ServerConnection,
            pub game_info: GameInfo,
            pub gameplay_settings: GameplaySettings,
            pub friendly_fire_settings: FriendlyFireSettings,
            pub team_settings: TeamSettings,
            pub camera_settings: CameraSettings,
            pub game_start_settings: GameStartSettings,
            pub config: ServerConfig,
        }
    }

    pub mod player_info {
        pub struct Player {
            pub name: String,
            pub score: Option<u16>,
            pub deaths: Option<u16>,
            pub ping: Option<u32>,
            pub team: Option<u8>,
            pub kills: Option<u16>,
        }

        pub struct PlayerInfo {
            pub players: Vec<Player>,
            pub player_count: u8,
        }
    }

    pub mod team_info {
        pub struct TeamInfo {
            // ! Team info structure is not not known yet
            pub raw: String,
        }
    }

    pub struct Response {
        pub server_info: server_info::ServerInfo,
        pub player_info: player_info::PlayerInfo,
        pub team_info: team_info::TeamInfo,
    }
}
