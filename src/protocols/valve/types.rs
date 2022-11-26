
/// The type of the server.
#[derive(Debug)]
pub enum Server {
    Dedicated,
    NonDedicated,
    TV
}

/// The Operating System that the server is on.
#[derive(Debug)]
pub enum Environment {
    Linux,
    Windows,
    Mac
}

/// A query response.
#[derive(Debug)]
pub struct Response {
    pub info: ServerInfo,
    pub players: Option<Vec<ServerPlayer>>,
    pub rules: Option<Vec<ServerRule>>
}

/// General server information's.
#[derive(Debug)]
pub struct ServerInfo {
    /// Protocol used by the server.
    pub protocol: u8,
    /// Name of the server.
    pub name: String,
    /// Map name.
    pub map: String,
    /// Name of the folder containing the game files.
    pub folder: String,
    /// Full name of the game.
    pub game: String,
    /// [Steam Application ID](https://developer.valvesoftware.com/wiki/Steam_Application_ID) of game.
    pub appid: u32,
    /// Number of players on the server.
    pub players: u8,
    /// Maximum number of players the server reports it can hold.
    pub max_players: u8,
    /// Number of bots on the server.
    pub bots: u8,
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
    pub mod_data: Option<ModData>
}

/// A server player.
#[derive(Debug)]
pub struct ServerPlayer {
    /// Player's name.
    pub name: String,
    /// General score.
    pub score: u32,
    /// How long they've been on the server for.
    pub duration: f32,
    /// Only for [the ship](https://developer.valvesoftware.com/wiki/The_Ship): deaths count
    pub deaths: Option<u32>, //the_ship
    /// Only for [the ship](https://developer.valvesoftware.com/wiki/The_Ship): money amount
    pub money: Option<u32>, //the_ship
}

/// A server rule.
#[derive(Debug)]
pub struct ServerRule {
    pub name: String,
    pub value: String
}

/// Only present for [the ship](https://developer.valvesoftware.com/wiki/The_Ship).
#[derive(Debug)]
pub struct TheShip {
    pub mode: u8,
    pub witnesses: u8,
    pub duration: u8
}

/// Some extra data that the server might provide or not.
#[derive(Debug)]
pub struct ExtraData {
    /// The server's game port number.
    pub port: Option<u16>,
    /// Server's SteamID.
    pub steam_id: Option<u64>,
    /// Spectator port number for SourceTV.
    pub tv_port: Option<u16>,
    /// Name of the spectator server for SourceTV.
    pub tv_name: Option<String>,
    /// Tags that describe the game according to the server.
    pub keywords: Option<String>,
    /// The server's 64-bit GameID.
    pub game_id: Option<u64>
}

/// Data related to GoldSrc Mod response.
#[derive(Debug)]
pub struct ModData {
    pub link: String,
    pub download_link: String,
    pub version: u32,
    pub size: u32,
    pub multiplayer_only: bool,
    pub has_own_dll: bool
}

pub fn get_optional_extracted_data(data: Option<ExtraData>) -> (Option<u16>, Option<u64>, Option<u16>, Option<String>, Option<String>) {
    match data {
        None => (None, None, None, None, None),
        Some(ed) => (ed.port, ed.steam_id, ed.tv_port, ed.tv_name, ed.keywords)
    }
}

/// The type of the request, see the [protocol](https://developer.valvesoftware.com/wiki/Server_queries).
#[derive(PartialEq, Clone)]
#[repr(u8)]
pub enum Request {
    /// Known as `A2S_INFO`
    INFO = 0x54,
    /// Known as `A2S_PLAYERS`
    PLAYERS = 0x55,
    /// Known as `A2S_RULES`
    RULES = 0x56
}

/// Supported steam apps id's
#[repr(u32)]
#[derive(PartialEq, Clone)]
pub enum SteamID {
    /// Team Fortress Classic
    TFC = 20,
    /// Day of Defeat
    DOD = 30,
    /// Counter-Strike: Condition Zero
    CSCZ = 80,
    /// Counter-Strike: Source
    CSS = 240,
    /// Day of Defeat: Source
    DODS = 300,
    /// Half-Life 2 Deathmatch
    HL2DM = 320,
    /// Team Fortress 2
    TF2 = 440,
    /// Left 4 Dead
    L4D = 500,
    /// Left 4 Dead
    L4D2 = 550,
    /// Alien Swarm
    ALIENS = 630,
    /// Counter-Strike: Global Offensive
    CSGO = 730,
    /// The Ship
    TS = 2400,
    /// Garry's Mod
    GM = 4000,
    /// Insurgency: Modern Infantry Combat
    INSMIC = 17700,
    /// Insurgency
    INS = 222880,
    /// Sven Co-op
    SC = 225840,
    /// The Forrest
    TF = 556450, //this is the id for the dedicated server, for the game its 242760
    /// 7 Days To Die
    SDTD = 251570,
    /// Unturned
    UNTURNED = 304930,
    /// ARK: Survival Evolved
    ASE = 346110,
    /// Insurgency: Sandstorm
    INSS = 581320,
    /// Alien Swarm: Reactive Drop
    ASRD = 563560,
}

impl SteamID {
    /// Get ID as App (the engine is specified).
    pub fn as_app(&self) -> App {
        match self {
            SteamID::TFC | SteamID::DOD | SteamID::CSCZ | SteamID::SC => App::GoldSrc(false),
            x => App::Source(Some(x.clone() as u32))
        }
    }
}

/// App type
#[derive(PartialEq, Clone)]
pub enum App {
    /// A Source game, the argument represents the wanted response steam app id, if its **None**,
    /// let the query find it, if its **Some**, the query fails if the response id is not the
    /// specified one.
    Source(Option<u32>),
    /// A GoldSrc game, the argument indicates whether to enforce getting the obsolete A2S_INFO
    /// goldsrc response or not.
    GoldSrc(bool)
}

/// What data to gather, purely used only with the query function.
pub struct GatheringSettings {
    pub players: bool,
    pub rules: bool
}

impl Default for GatheringSettings {
    /// Default values are true for both the players and the rules.
    fn default() -> Self {
        Self {
            players: true,
            rules: true
        }
    }
}

/// Generic response types that are used by many games, they are the protocol ones, but without the
/// unnecessary bits (example: the **The Ship**-only fields)
pub mod game {
    use crate::protocols::valve::types::get_optional_extracted_data;
    use super::{Server, ServerRule, ServerPlayer};

    #[derive(Debug)]
    pub struct Player {
        pub name: String,
        pub score: u32,
        pub duration: f32
    }

    impl Player {
        pub fn from_valve_response(player: &ServerPlayer) -> Self {
            Self {
                name: player.name.clone(),
                score: player.score,
                duration: player.duration
            }
        }
    }

    #[derive(Debug)]
    pub struct Response {
        pub protocol: u8,
        pub name: String,
        pub map: String,
        pub game: String,
        pub players: u8,
        pub players_details: Vec<Player>,
        pub max_players: u8,
        pub bots: u8,
        pub server_type: Server,
        pub has_password: bool,
        pub vac_secured: bool,
        pub version: String,
        pub port: Option<u16>,
        pub steam_id: Option<u64>,
        pub tv_port: Option<u16>,
        pub tv_name: Option<String>,
        pub keywords: Option<String>,
        pub rules: Vec<ServerRule>
    }

    impl Response {
        pub fn new_from_valve_response(response: super::Response) -> Self {
            let (port, steam_id, tv_port, tv_name, keywords) = get_optional_extracted_data(response.info.extra_data);

            Self {
                protocol: response.info.protocol,
                name: response.info.name,
                map: response.info.map,
                game: response.info.game,
                players: response.info.players,
                players_details: response.players.unwrap().iter().map(|p| Player::from_valve_response(p)).collect(),
                max_players: response.info.max_players,
                bots: response.info.bots,
                server_type: response.info.server_type,
                has_password: response.info.has_password,
                vac_secured: response.info.vac_secured,
                version: response.info.version,
                port,
                steam_id,
                tv_port,
                tv_name,
                keywords,
                rules: response.rules.unwrap()
            }
        }
    }
}

