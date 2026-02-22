use std::collections::HashMap;

use crate::protocols::types::{CommonPlayer, CommonResponse, ExtraRequestSettings, GatherToggle, GenericPlayer};
use crate::GDErrorKind::UnknownEnumCast;
use crate::GDResult;
use crate::{buffer::Buffer, protocols::GenericResponse};
use byteorder::LittleEndian;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The type of the server.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Server {
    Dedicated,
    NonDedicated,
    TV,
}

impl Server {
    pub(crate) fn from_gldsrc(value: u8) -> GDResult<Self> {
        Ok(match value.to_ascii_lowercase() {
            100 => Self::Dedicated,    //'d'
            108 => Self::NonDedicated, //'l'
            112 => Self::TV,           //'p'
            _ => Err(UnknownEnumCast)?,
        })
    }
}

/// The Operating System that the server is on.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Environment {
    Linux,
    Windows,
    Mac,
}

impl Environment {
    pub(crate) fn from_gldsrc(value: u8) -> GDResult<Self> {
        Ok(match value.to_ascii_lowercase() {
            108 => Self::Linux,     //'l'
            119 => Self::Windows,   //'w'
            109 | 111 => Self::Mac, //'m' or 'o'
            _ => Err(UnknownEnumCast)?,
        })
    }
}

/// A query response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub info: ServerInfo,
    pub players: Option<Vec<ServerPlayer>>,
    pub rules: Option<HashMap<String, String>>,
}

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse<'_> { GenericResponse::Valve(self) }

    fn name(&self) -> Option<&str> { Some(&self.info.name) }
    fn game_mode(&self) -> Option<&str> { Some(&self.info.game_mode) }
    fn game_version(&self) -> Option<&str> { Some(&self.info.game_version) }
    fn map(&self) -> Option<&str> { Some(&self.info.map) }
    fn players_maximum(&self) -> u32 { self.info.players_maximum.into() }
    fn players_online(&self) -> u32 { self.info.players_online.into() }
    fn players_bots(&self) -> Option<u32> { Some(self.info.players_bots.into()) }
    fn has_password(&self) -> Option<bool> { Some(self.info.has_password) }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        self.players
            .as_ref()
            .map(|p| p.iter().map(|p| p as &dyn CommonPlayer).collect())
    }
}

/// General server information's.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ServerInfo {
    /// Protocol used by the server.
    pub protocol_version: u8,
    /// Name of the server.
    pub name: String,
    /// Map name.
    pub map: String,
    /// Name of the folder containing the game files.
    pub folder: String,
    /// The server-declared name of the game/game mode.
    pub game_mode: String,
    /// [Steam Application ID](https://developer.valvesoftware.com/wiki/Steam_Application_ID) of game.
    pub appid: u32,
    /// Number of players on the server.
    pub players_online: u8,
    /// Maximum number of players the server reports it can hold.
    pub players_maximum: u8,
    /// Number of bots on the server.
    pub players_bots: u8,
    /// Dedicated, NonDedicated or SourceTV
    pub server_type: Server,
    /// The Operating System that the server is on.
    pub environment_type: Environment,
    /// Indicates whether the server requires a password.
    pub has_password: bool,
    /// Indicates whether the server uses VAC.
    pub vac_secured: bool,
    /// [The ship](https://developer.valvesoftware.com/wiki/The_Ship) extra data
    pub the_ship: Option<TheShip>,
    /// Version of the game installed on the server.
    pub game_version: String,
    /// Some extra data that the server might provide or not.
    pub extra_data: Option<ExtraData>,
    /// GoldSrc only: Indicates whether the hosted game is a mod.
    pub is_mod: bool,
    /// GoldSrc only: If the game is a mod, provide additional data.
    pub mod_data: Option<ModData>,
}

/// A server player.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ServerPlayer {
    /// Player's name.
    pub name: String,
    /// General score.
    pub score: i32,
    /// How long a player has been in the server (seconds).
    pub duration: f32,
    /// Only for [the ship](https://developer.valvesoftware.com/wiki/The_Ship): deaths count
    pub deaths: Option<u32>, // the_ship
    /// Only for [the ship](https://developer.valvesoftware.com/wiki/The_Ship): money amount
    pub money: Option<u32>, // the_ship
}

impl CommonPlayer for ServerPlayer {
    fn as_original(&self) -> GenericPlayer<'_> { GenericPlayer::Valve(self) }
    fn name(&self) -> &str { &self.name }
    fn score(&self) -> Option<i32> { Some(self.score) }
}

/// Only present for [the ship](https://developer.valvesoftware.com/wiki/The_Ship).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TheShip {
    pub mode: u8,
    pub witnesses: u8,
    pub duration: u8,
}

/// Some extra data that the server might provide or not.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtraData {
    /// The server's game port number.
    pub port: Option<u16>,
    /// Server's SteamID.
    pub steam_id: Option<u64>,
    /// SourceTV's port.
    pub tv_port: Option<u16>,
    /// SourceTV's name.
    pub tv_name: Option<String>,
    /// Keywords that describe the server according to it.
    pub keywords: Option<String>,
    /// The server's 64-bit GameID.
    pub game_id: Option<u64>,
}

/// Data related to GoldSrc Mod response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModData {
    pub link: String,
    pub download_link: String,
    pub version: u32,
    pub size: u32,
    pub multiplayer_only: bool,
    pub has_own_dll: bool,
}

pub(crate) type ExtractedData = (
    Option<u16>,
    Option<u64>,
    Option<u16>,
    Option<String>,
    Option<String>,
);

pub(crate) fn get_optional_extracted_data(data: Option<ExtraData>) -> ExtractedData {
    match data {
        None => (None, None, None, None, None),
        Some(ed) => (ed.port, ed.steam_id, ed.tv_port, ed.tv_name, ed.keywords),
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Packet {
    pub header: u32,
    pub kind: u8,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(kind: u8, payload: Vec<u8>) -> Self {
        Self {
            header: u32::MAX, // FF FF FF FF
            kind,
            payload,
        }
    }

    pub fn new_from_bufferer(buffer: &mut Buffer<LittleEndian>) -> GDResult<Self> {
        Ok(Self {
            header: buffer.read::<u32>()?,
            kind: buffer.read::<u8>()?,
            payload: buffer.remaining_bytes().to_vec(),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::from(self.header.to_be_bytes());

        buf.push(self.kind);
        buf.extend(&self.payload);

        buf
    }
}

/// The type of the request, see the [protocol](https://developer.valvesoftware.com/wiki/Server_queries).
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum Request {
    /// Known as `A2S_INFO`
    Info = 0x54,
    /// Known as `A2S_PLAYERS`
    Players = 0x55,
    /// Known as `A2S_RULES`
    Rules = 0x56,
}

impl Request {
    pub fn get_default_payload(self) -> Vec<u8> {
        match self {
            Self::Info => String::from("Source Engine Query\0").into_bytes(),
            _ => vec![0xFF, 0xFF, 0xFF, 0xFF],
        }
    }
}

/// Every supported Valve game references this enum, represents the behaviour
/// of server requests and responses.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Engine {
    /// A Source game, the argument represents the possible steam app ids.
    /// If its **None**, let the query find it (could come with some drawbacks,
    /// some games do not respond on certain protocol versions (CSS on 7),
    /// some have additional data (The Ship).
    /// If its **Some**, the first value is the main steam app id, the second
    /// could be a secondly used id, as some games use a different one for
    /// dedicated servers. Beware if **check_app_id** is set to true in
    /// [GatheringSettings], as the query will fail if the server doesnt respond
    /// with the expected ids.
    Source(Option<(u32, Option<u32>)>),
    /// A GoldSrc game, the argument indicates whether to enforce
    /// requesting the obsolete A2S_INFO response or not.
    GoldSrc(bool),
}

impl Engine {
    pub const fn new(appid: u32) -> Self { Self::Source(Some((appid, None))) }

    pub const fn new_gold_src(force: bool) -> Self { Self::GoldSrc(force) }

    pub const fn new_with_dedicated(appid: u32, dedicated_appid: u32) -> Self {
        Self::Source(Some((appid, Some(dedicated_appid))))
    }
}

/// What data to gather, purely used only with the query function.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GatheringSettings {
    pub players: GatherToggle,
    pub rules: GatherToggle,
    pub check_app_id: bool,
}

impl GatheringSettings {
    /// Default values are try to gather but don't fail on timeout for both
    /// players and rules.
    pub const fn default() -> Self {
        Self {
            players: GatherToggle::Try,
            rules: GatherToggle::Try,
            check_app_id: true,
        }
    }

    pub const fn into_extra(self) -> ExtraRequestSettings {
        ExtraRequestSettings {
            hostname: None,
            protocol_version: None,
            gather_players: Some(self.players),
            gather_rules: Some(self.rules),
            check_app_id: Some(self.check_app_id),
        }
    }
}

impl Default for GatheringSettings {
    fn default() -> Self { Self::default() }
}

impl From<ExtraRequestSettings> for GatheringSettings {
    fn from(value: ExtraRequestSettings) -> Self {
        let default = Self::default();
        Self {
            players: value.gather_players.unwrap_or(default.players),
            rules: value.gather_rules.unwrap_or(default.rules),
            check_app_id: value.check_app_id.unwrap_or(default.check_app_id),
        }
    }
}

/// Generic response types that are used by many games, they are the protocol
/// ones, but without the unnecessary bits (example: the **The Ship**-only
/// fields).
pub mod game {
    use super::{Server, ServerPlayer};
    use crate::protocols::valve::types::get_optional_extracted_data;
    use std::collections::HashMap;

    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};

    /// A player's details.
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct Player {
        /// Player's name.
        pub name: String,
        /// Player's score.
        pub score: i32,
        /// How long a player has been in the server (seconds).
        pub duration: f32,
    }

    impl Player {
        pub fn from_valve_response(player: &ServerPlayer) -> Self {
            Self {
                name: player.name.clone(),
                score: player.score,
                duration: player.duration,
            }
        }
    }

    /// The query response.
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq)]
    pub struct Response {
        /// Protocol used by the server.
        pub protocol: u8,
        /// Name of the server.
        pub name: String,
        /// Map name.
        pub map: String,
        /// The name of the game.
        pub game: String,
        /// Server's app id.
        pub appid: u32,
        /// Number of players on the server.
        pub players_online: u8,
        /// Details about the server's players (not all players necessarily).
        pub players_details: Vec<Player>,
        /// Maximum number of players the server reports it can hold.
        pub players_maximum: u8,
        /// Number of bots on the server.
        pub players_bots: u8,
        /// Dedicated, NonDedicated or SourceTV
        pub server_type: Server,
        /// Indicates whether the server requires a password.
        pub has_password: bool,
        /// Indicated whether the server uses VAC.
        pub vac_secured: bool,
        /// Version of the game installed on the server.
        pub version: String,
        /// The server's reported connection port.
        pub port: Option<u16>,
        /// Server's SteamID.
        pub steam_id: Option<u64>,
        /// SourceTV's connection port.
        pub tv_port: Option<u16>,
        /// SourceTV's name.
        pub tv_name: Option<String>,
        /// Keywords that describe the server according to it.
        pub keywords: Option<String>,
        /// Server's rules.
        pub rules: HashMap<String, String>,
    }

    impl Response {
        pub fn new_from_valve_response(response: super::Response) -> Self {
            let (port, steam_id, tv_port, tv_name, keywords) = get_optional_extracted_data(response.info.extra_data);

            Self {
                protocol: response.info.protocol_version,
                name: response.info.name,
                map: response.info.map,
                game: response.info.game_mode,
                appid: response.info.appid,
                players_online: response.info.players_online,
                players_details: response
                    .players
                    .unwrap_or_default()
                    .iter()
                    .map(Player::from_valve_response)
                    .collect(),
                players_maximum: response.info.players_maximum,
                players_bots: response.info.players_bots,
                server_type: response.info.server_type,
                has_password: response.info.has_password,
                vac_secured: response.info.vac_secured,
                version: response.info.game_version,
                port,
                steam_id,
                tv_port,
                tv_name,
                keywords,
                rules: response.rules.unwrap_or_default(),
            }
        }
    }
}
