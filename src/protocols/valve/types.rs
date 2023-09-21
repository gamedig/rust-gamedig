use std::collections::HashMap;

use crate::protocols::types::{CommonPlayer, CommonResponse, ExtraRequestSettings, GenericPlayer};
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
        Ok(match value {
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
        Ok(match value {
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
    fn as_original(&self) -> GenericResponse { GenericResponse::Valve(self) }

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
    fn as_original(&self) -> GenericPlayer { GenericPlayer::Valve(self) }
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

/// Supported steam apps
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SteamApp {
    /// Counter-Strike
    CS,
    /// Creativerse
    CREATIVERSE,
    /// Team Fortress Classic
    TFC,
    /// Day of Defeat
    DOD,
    /// Counter-Strike: Condition Zero
    CSCZ,
    /// Counter-Strike: Source
    CSS,
    /// Day of Defeat: Source
    DODS,
    /// Half-Life 2 Deathmatch
    HL2DM,
    /// Half-Life Deathmatch: Source
    HLDMS,
    /// Team Fortress 2
    TF2,
    /// Left 4 Dead
    LEFT4DEAD,
    /// Left 4 Dead
    LEFT4DEAD2,
    /// Alien Swarm
    ALIENSWARM,
    /// Counter-Strike: Global Offensive
    CSGO,
    /// The Ship
    SHIP,
    /// Garry's Mod
    GARRYSMOD,
    /// Age of Chivalry
    AGEOFCHIVALRY,
    /// Insurgency: Modern Infantry Combat
    INSURGENCYMIC,
    /// ARMA 2: Operation Arrowhead
    ARMA2OA,
    /// Project Zomboid
    PRZOMBOID,
    /// Insurgency
    INSURGENCY,
    /// Sven Co-op
    SVEENCOOP,
    /// 7 Days To Die
    SD2D,
    /// Rust
    RUST,
    /// Vallistic Overkill
    BALLISTICOVERKILL,
    /// Don't Starve Together
    DST,
    /// BrainBread 2
    BRAINBREAD2,
    /// Codename CURE
    CODENAMECURE,
    /// Black Mesa
    BLACKMESA,
    /// Colony Survival
    COLONYSURVIVAL,
    /// Avorion
    AVORION,
    /// Day of Infamy
    DOI,
    /// The Forest
    THEFOREST,
    /// Unturned
    UNTURNED,
    /// ARK: Survival Evolved
    ARKSE,
    /// Battalion 1944
    BAT1944,
    /// Insurgency: Sandstorm
    INSURGENCYSANDSTORM,
    /// Alien Swarm: Reactive Drop
    ASRD,
    /// Risk of Rain 2
    ROR2,
    /// Operation: Harsh Doorstop
    OHD,
    /// Onset
    ONSET,
    /// V Rising
    VRISING,
    /// Hell Let Loose
    HLL,
}

impl SteamApp {
    /// Get the specified app as engine.
    pub const fn as_engine(&self) -> Engine {
        match self {
            Self::CSS => Engine::new_source(240),
            Self::DODS => Engine::new_source(300),
            Self::HL2DM => Engine::new_source(320),
            Self::HLDMS => Engine::new_source(360),
            Self::TF2 => Engine::new_source(440),
            Self::LEFT4DEAD => Engine::new_source(500),
            Self::LEFT4DEAD2 => Engine::new_source(550),
            Self::ALIENSWARM => Engine::new_source(630),
            Self::CSGO => Engine::new_source(730),
            Self::SHIP => Engine::new_source(2400),
            Self::GARRYSMOD => Engine::new_source(4000),
            Self::AGEOFCHIVALRY => Engine::new_source(17510),
            Self::INSURGENCYMIC => Engine::new_source(17700),
            Self::ARMA2OA => Engine::new_source(33930),
            Self::PRZOMBOID => Engine::new_source(108_600),
            Self::INSURGENCY => Engine::new_source(222_880),
            Self::SD2D => Engine::new_source(251_570),
            Self::RUST => Engine::new_source(252_490),
            Self::CREATIVERSE => Engine::new_source(280_790),
            Self::BALLISTICOVERKILL => Engine::new_source(296_300),
            Self::DST => Engine::new_source(322_320),
            Self::BRAINBREAD2 => Engine::new_source(346_330),
            Self::CODENAMECURE => Engine::new_source(355_180),
            Self::BLACKMESA => Engine::new_source(362_890),
            Self::COLONYSURVIVAL => Engine::new_source(366_090),
            Self::AVORION => Engine::new_source(445_220),
            Self::DOI => Engine::new_source(447_820),
            Self::THEFOREST => Engine::new_source(556_450),
            Self::UNTURNED => Engine::new_source(304_930),
            Self::ARKSE => Engine::new_source(346_110),
            Self::BAT1944 => Engine::new_source(489_940),
            Self::INSURGENCYSANDSTORM => Engine::new_source(581_320),
            Self::ASRD => Engine::new_source(563_560),
            Self::ROR2 => Engine::new_source(632_360),
            Self::OHD => Engine::new_source_with_dedicated(736_590, 950_900),
            Self::ONSET => Engine::new_source(1_105_810),
            Self::VRISING => Engine::new_source(1_604_030),
            Self::HLL => Engine::new_source(686_810),
            _ => Engine::GoldSrc(false), // CS - 10, TFC - 20, DOD - 30, CSCZ - 80, SC - 225840
        }
    }
}

/// Engine type.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Engine {
    /// A Source game, the argument represents the possible steam app ids, if
    /// its **None**, let the query find it, if its **Some**, the query
    /// fails if the response id is not the first one, which is the game app
    /// id, or the other one, which is the dedicated server app id.
    Source(Option<(u32, Option<u32>)>),
    /// A GoldSrc game, the argument indicates whether to enforce
    /// requesting the obsolete A2S_INFO response or not.
    GoldSrc(bool),
}

impl Engine {
    pub const fn new_source(appid: u32) -> Self { Self::Source(Some((appid, None))) }

    pub const fn new_source_with_dedicated(appid: u32, dedicated_appid: u32) -> Self {
        Self::Source(Some((appid, Some(dedicated_appid))))
    }
}

/// What data to gather, purely used only with the query function.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GatheringSettings {
    pub players: bool,
    pub rules: bool,
    pub check_app_id: bool,
}

impl Default for GatheringSettings {
    /// Default values are true for both the players and the rules.
    fn default() -> Self {
        Self {
            players: true,
            rules: true,
            check_app_id: true,
        }
    }
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
