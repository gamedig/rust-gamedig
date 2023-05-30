use std::collections::HashMap;

use crate::bufferer::Bufferer;
use crate::GDError::UnknownEnumCast;
use crate::GDResult;
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
            100 => Server::Dedicated,    //'d'
            108 => Server::NonDedicated, //'l'
            112 => Server::TV,           //'p'
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
            108 => Environment::Linux,     //'l'
            119 => Environment::Windows,   //'w'
            109 | 111 => Environment::Mac, //'m' or 'o'
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

/// General server information's.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ServerInfo {
    /// Protocol used by the server.
    pub protocol: u8,
    /// Name of the server.
    pub name: String,
    /// Map name.
    pub map: String,
    /// Name of the folder containing the game files.
    pub folder: String,
    /// The name of the game.
    pub game: String,
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
    pub version: String,
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
    pub score: u32,
    /// How long a player has been in the server (seconds).
    pub duration: f32,
    /// Only for [the ship](https://developer.valvesoftware.com/wiki/The_Ship): deaths count
    pub deaths: Option<u32>, // the_ship
    /// Only for [the ship](https://developer.valvesoftware.com/wiki/The_Ship): money amount
    pub money: Option<u32>, // the_ship
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
            header: 4294967295, // FF FF FF FF
            kind,
            payload,
        }
    }

    pub fn new_from_bufferer(buffer: &mut Bufferer) -> GDResult<Self> {
        Ok(Self {
            header: buffer.get_u32()?,
            kind: buffer.get_u8()?,
            payload: buffer.remaining_data_vec(),
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
    pub fn get_default_payload(&self) -> Vec<u8> {
        match self {
            Request::Info => String::from("Source Engine Query\0").into_bytes(),
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
    L4D,
    /// Left 4 Dead
    L4D2,
    /// Alien Swarm
    ALIENS,
    /// Counter-Strike: Global Offensive
    CSGO,
    /// The Ship
    TS,
    /// Garry's Mod
    GM,
    /// Age of Chivalry
    AOC,
    /// Insurgency: Modern Infantry Combat
    INSMIC,
    /// ARMA 2: Operation Arrowhead
    ARMA2OA,
    /// Project Zomboid
    PZ,
    /// Insurgency
    INS,
    /// Sven Co-op
    SC,
    /// 7 Days To Die
    SDTD,
    /// Rust
    RUST,
    /// Vallistic Overkill
    BO,
    /// Don't Starve Together
    DST,
    /// BrainBread 2
    BB2,
    /// Codename CURE
    CCURE,
    /// Black Mesa
    BM,
    /// Colony Survival
    COSU,
    /// Avorion
    AVORION,
    /// Day of Infamy
    DOI,
    /// The Forest
    TF,
    /// Unturned
    UNTURNED,
    /// ARK: Survival Evolved
    ASE,
    /// Battalion 1944
    BAT1944,
    /// Insurgency: Sandstorm
    INSS,
    /// Alien Swarm: Reactive Drop
    ASRD,
    /// Risk of Rain 2
    ROR2,
    /// Operation: Harsh Doorstop
    OHD,
    /// Onset
    ONSET,
    /// V Rising
    VR,
    /// Hell Let Loose
    HLL,
}

impl SteamApp {
    /// Get the specified app as engine.
    pub fn as_engine(&self) -> Engine {
        match self {
            SteamApp::CS => Engine::GoldSrc(false),   // 10
            SteamApp::TFC => Engine::GoldSrc(false),  // 20
            SteamApp::DOD => Engine::GoldSrc(false),  // 30
            SteamApp::CSCZ => Engine::GoldSrc(false), // 80
            SteamApp::CSS => Engine::new_source(240),
            SteamApp::DODS => Engine::new_source(300),
            SteamApp::HL2DM => Engine::new_source(320),
            SteamApp::HLDMS => Engine::new_source(360),
            SteamApp::TF2 => Engine::new_source(440),
            SteamApp::L4D => Engine::new_source(500),
            SteamApp::L4D2 => Engine::new_source(550),
            SteamApp::ALIENS => Engine::new_source(630),
            SteamApp::CSGO => Engine::new_source(730),
            SteamApp::TS => Engine::new_source(2400),
            SteamApp::GM => Engine::new_source(4000),
            SteamApp::AOC => Engine::new_source(17510),
            SteamApp::INSMIC => Engine::new_source(17700),
            SteamApp::ARMA2OA => Engine::new_source(33930),
            SteamApp::PZ => Engine::new_source(108600),
            SteamApp::INS => Engine::new_source(222880),
            SteamApp::SC => Engine::GoldSrc(false), // 225840
            SteamApp::SDTD => Engine::new_source(251570),
            SteamApp::RUST => Engine::new_source(252490),
            SteamApp::BO => Engine::new_source(296300),
            SteamApp::DST => Engine::new_source(322320),
            SteamApp::BB2 => Engine::new_source(346330),
            SteamApp::CCURE => Engine::new_source(355180),
            SteamApp::BM => Engine::new_source(362890),
            SteamApp::COSU => Engine::new_source(366090),
            SteamApp::AVORION => Engine::new_source(445220),
            SteamApp::DOI => Engine::new_source(447820),
            SteamApp::TF => Engine::new_source(556450),
            SteamApp::UNTURNED => Engine::new_source(304930),
            SteamApp::ASE => Engine::new_source(346110),
            SteamApp::BAT1944 => Engine::new_source(489940),
            SteamApp::INSS => Engine::new_source(581320),
            SteamApp::ASRD => Engine::new_source(563560),
            SteamApp::ROR2 => Engine::new_source(632360),
            SteamApp::OHD => Engine::new_source_with_dedicated(736590, 950900),
            SteamApp::ONSET => Engine::new_source(1105810),
            SteamApp::VR => Engine::new_source(1604030),
            SteamApp::HLL => Engine::new_source(686810),
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
    pub fn new_source(appid: u32) -> Self { Engine::Source(Some((appid, None))) }

    pub fn new_source_with_dedicated(appid: u32, dedicated_appid: u32) -> Self {
        Engine::Source(Some((appid, Some(dedicated_appid))))
    }
}

/// What data to gather, purely used only with the query function.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GatheringSettings {
    pub players: bool,
    pub rules: bool,
}

impl Default for GatheringSettings {
    /// Default values are true for both the players and the rules.
    fn default() -> Self {
        Self {
            players: true,
            rules: true,
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
        pub score: u32,
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
                protocol: response.info.protocol,
                name: response.info.name,
                map: response.info.map,
                game: response.info.game,
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
                version: response.info.version,
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
