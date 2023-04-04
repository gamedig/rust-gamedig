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

    pub mod server_info {

        /// A Flag is a representation of a boolean value from a string.
        #[derive(Debug, PartialEq)]
        pub enum Flag {
            Enabled,
            Disabled,
            Unknown,
        }

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
            pub num_players: usize,
            pub max_players: usize,
            pub game_mode: String,
            pub game_version: String,
            pub status: usize,
            pub game_id: String,
            pub map_id: String,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct GameplaySettings {
            pub sv_punkbuster: Option<Flag>,
            pub time_limit: Option<usize>,
            pub num_rounds: Option<usize>,
            pub spawn_wave_time: Option<usize>,
            pub spawn_delay: Option<usize>,
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
            pub us_team_ratio: Option<usize>,
            pub nva_team_ratio: Option<usize>,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct CameraSettings {
            pub name_tag_distance: Option<usize>,
            pub name_tag_distance_scope: Option<usize>,
            pub allow_nose_cam: Option<String>,
            pub external_view: Option<String>,
            pub free_camera: Option<Flag>,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct GameStartSettings {
            pub game_start_delay: Option<usize>,
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
            pub reserved_slots: Option<usize>,
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
        }

        /// The bytes of the request packet to be sent to the server
        #[allow(dead_code)]

        impl Flag {
            fn from_str(s: &str) -> Self {
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

        impl ServerConnection {
            pub fn new(hostname: String, hostport: u16, password: Flag) -> Self {
                ServerConnection {
                    hostname,
                    hostport,
                    password,
                }
            }
        }

        impl GameInfo {
            pub fn new(
                map_name: String,
                game_type: String,
                num_players: usize,
                max_players: usize,
                game_mode: String,
                game_version: String,
                status: usize,
                game_id: String,
                map_id: String,
            ) -> Self {
                GameInfo {
                    map_name,
                    game_type,
                    num_players,
                    max_players,
                    game_mode,
                    game_version,
                    status,
                    game_id,
                    map_id,
                }
            }
        }

        impl GameplaySettings {
            pub fn new(
                sv_punkbuster: Option<Flag>,
                time_limit: Option<usize>,
                num_rounds: Option<usize>,
                spawn_wave_time: Option<usize>,
                spawn_delay: Option<usize>,
            ) -> Self {
                GameplaySettings {
                    sv_punkbuster,
                    time_limit,
                    num_rounds,
                    spawn_wave_time,
                    spawn_delay,
                }
            }
        }

        impl FriendlyFireSettings {
            pub fn new(
                soldier_friendly_fire: Option<Flag>,
                vehicle_friendly_fire: Option<Flag>,
                soldier_friendly_fire_on_splash: Option<String>,
                vehicle_friendly_fire_on_splash: Option<String>,
            ) -> Self {
                FriendlyFireSettings {
                    soldier_friendly_fire,
                    vehicle_friendly_fire,
                    soldier_friendly_fire_on_splash,
                    vehicle_friendly_fire_on_splash,
                }
            }
        }

        impl TeamSettings {
            pub fn new(us_team_ratio: Option<usize>, nva_team_ratio: Option<usize>) -> Self {
                TeamSettings {
                    us_team_ratio,
                    nva_team_ratio,
                }
            }
        }

        impl CameraSettings {
            pub fn new(
                name_tag_distance: Option<usize>,
                name_tag_distance_scope: Option<usize>,
                allow_nose_cam: Option<String>,
                external_view: Option<String>,
                free_camera: Option<Flag>,
            ) -> Self {
                CameraSettings {
                    name_tag_distance,
                    name_tag_distance_scope,
                    allow_nose_cam,
                    external_view,
                    free_camera,
                }
            }
        }

        impl GameStartSettings {
            pub fn new(
                game_start_delay: Option<usize>,
                ticket_ratio: Option<String>,
                kickback: Option<String>,
                kickback_on_splash: Option<String>,
                auto_balance: Option<Flag>,
            ) -> Self {
                GameStartSettings {
                    game_start_delay,
                    ticket_ratio,
                    kickback,
                    kickback_on_splash,
                    auto_balance,
                }
            }
        }

        impl ServerConfig {
            pub fn new(
                dedicated: Option<Flag>,
                cpu: Option<usize>,
                bot_skill: Option<String>,
                reserved_slots: Option<usize>,
                active_mods: Option<String>,
            ) -> Self {
                ServerConfig {
                    dedicated,
                    cpu,
                    bot_skill,
                    reserved_slots,
                    active_mods,
                }
            }
        }

        impl ServerInfo {
            pub fn new(
                connection: ServerConnection,
                game_info: GameInfo,
                gameplay_settings: GameplaySettings,
                friendly_fire_settings: FriendlyFireSettings,
                team_settings: TeamSettings,
                camera_settings: CameraSettings,
                game_start_settings: GameStartSettings,
                config: ServerConfig,
            ) -> Self {
                ServerInfo {
                    connection,
                    game_info,
                    gameplay_settings,
                    friendly_fire_settings,
                    team_settings,
                    camera_settings,
                    game_start_settings,
                    config,
                }
            }
        }
    }
}
