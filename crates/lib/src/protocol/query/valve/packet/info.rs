/// The fixed payload used to request server information.
///
/// This 25 byte payload is sent to the server to request details about the server’s current state.
/// The payload is structured as follows:
///
/// - **Header:** `0xFF, 0xFF, 0xFF, 0xFF`
/// - **Request Type:** `0x54`
/// - **String:** `"Source Engine Query\0"`
pub const INFO_REQUEST_PAYLOAD: [u8; 25] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0x54, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x45, 0x6E, 0x67, 0x69,
    0x6E, 0x65, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x00,
];

/// Represents the type of server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerType {
    /// A dedicated server.
    Dedicated,

    /// A non dedicated (listen) server.
    NonDedicated,

    /// A SourceTV server.
    SourceTV,
}

impl ServerType {
    /// Converts a `u8` value into a [`ServerType`].
    ///
    /// The conversion uses the following mappings:
    ///
    /// - `b'd'` or `b'D'` &rarr; [`ServerType::Dedicated`]
    /// - `b'l'` or `b'L'` &rarr; [`ServerType::NonDedicated`]
    /// - `b'p'` or `b'P'` &rarr; [`ServerType::SourceTV`]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            b'd' | b'D' => Some(Self::Dedicated),
            b'l' | b'L' => Some(Self::NonDedicated),
            b'p' | b'P' => Some(Self::SourceTV),
            _ => None,
        }
    }
}

/// Represents the operating environment of the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerEnvironment {
    /// Server running on Linux.
    Linux,

    /// Server running on Windows.
    Windows,

    /// Server running on macOS.
    Mac,
}

impl ServerEnvironment {
    /// Converts a `u8` value into a [`ServerEnvironment`].
    ///
    /// The conversion uses the following mappings:
    ///
    /// - `b'l'` or `b'L'` &rarr; [`ServerEnvironment::Linux`]
    /// - `b'w'` or `b'W'` &rarr; [`ServerEnvironment::Windows`]
    /// - `b'm'`, `b'M'`, `b'o'`, or `b'O'` &rarr; [`ServerEnvironment::Mac`]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            b'l' | b'L' => Some(Self::Linux),
            b'w' | b'W' => Some(Self::Windows),
            b'm' | b'M' | b'o' | b'O' => Some(Self::Mac),
            _ => None,
        }
    }
}

/// Game modes specific to `The Ship`.
///
/// Some servers running the game `The Ship` include extra data about the game mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheShipMode {
    /// Hunt mode.
    Hunt = 0,
    /// Elimination mode.
    Elimination = 1,
    /// Duel mode.
    Duel = 2,
    /// Deathmatch mode.
    Deathmatch = 3,
    /// VIP Team mode.
    VIPTeam = 4,
    /// Team Elimination mode.
    TeamElimination = 5,
}

impl TheShipMode {
    /// Converts a `u8` value into a [`TheShipMode`].
    ///
    /// The conversion uses the following mappings:
    ///
    /// - `0` &rarr; [`TheShipMode::Hunt`]
    /// - `1` &rarr; [`TheShipMode::Elimination`]
    /// - `2` &rarr; [`TheShipMode::Duel`]
    /// - `3` &rarr; [`TheShipMode::Deathmatch`]
    /// - `4` &rarr; [`TheShipMode::VIPTeam`]
    /// - `5` &rarr; [`TheShipMode::TeamElimination`]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Hunt),
            1 => Some(Self::Elimination),
            2 => Some(Self::Duel),
            3 => Some(Self::Deathmatch),
            4 => Some(Self::VIPTeam),
            5 => Some(Self::TeamElimination),
            _ => None,
        }
    }
}

/// Additional game mode information for servers running `The Ship`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheShip {
    /// The game mode used by the server.
    pub mode: Option<TheShipMode>,

    /// The number of witnesses required for a player to be arrested.
    pub witnesses: u8,

    /// Time (in seconds) before a player is arrested once witnessed.
    pub duration: u8,
}

/// Flags that indicate which optional fields are present in the server response.
pub enum ExtraDataFlags {
    /// The server's game port number.
    ///
    /// Provided in [`Extended::port`].
    Port = 0x80,

    /// The server's SteamID.
    ///
    /// Provided in [`Extended::steam_id`].
    SteamID = 0x10,

    /// Information about SourceTV.
    ///
    /// Provided in [`Response::source_tv`].
    SourceTV = 0x40,

    /// Tags that describe the game.
    ///
    /// Provided in [`Extended::keywords`].
    Keywords = 0x20,

    /// The server's 64 bit GameID.
    ///
    /// Provided in [`Extended::game_id`].
    GameID = 0x01,
}

/// Extra data flag that indicates which optional fields are present in the server response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraDataFlag(pub u8);

impl ExtraDataFlag {
    pub const fn contains(&self, flag: ExtraDataFlags) -> bool { self.0 & flag as u8 != 0 }
}

/// Extended server information available when certain flags are set.
///
/// The [`Response::edf`] (Extra Data Flag) in the server response determines which fields are available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extended {
    /// The server's game port number.
    ///
    /// Available if `edf & 0x80 != 0`.
    pub port: Option<u16>,

    /// The server's SteamID.
    ///
    /// Available if `edf & 0x10 != 0`.
    pub steam_id: Option<u64>,

    /// Tags that describe the game.
    ///
    /// Available if `edf & 0x20 != 0`.
    pub keywords: Option<String>,

    /// The server's 64 bit GameID. When present, it indicates a more accurate AppID then the low 24 bits.
    ///
    /// Available if `edf & 0x01 != 0`.
    pub game_id: Option<u64>,
}

/// Information specific to SourceTV.
///
/// SourceTV is a system that allows users to spectate live games.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceTV {
    /// Spectator port number for SourceTV.
    pub port: u16,

    /// Name of the SourceTV server.
    pub name: String,
}

/// Information about a mod running on the server (`GoldSrc` engines).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mod {
    /// URL to mod website.
    pub link: String,

    /// URL to download the mod.
    pub download_link: String,

    /// Version of the mod.
    pub version: u32,

    /// Size of the mod in bytes.
    pub size: u32,

    /// Indicates if the mod is multiplayer only.
    pub multiplayer_only: bool,

    /// Indicates if the mod has its own DLL.
    pub dll: bool,
}

/// The complete response from a server information query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    /// Protocol version used by the server.
    pub protocol: u8,

    /// Server name.
    pub name: String,

    /// Current map loaded on the server.
    pub map: String,

    /// Folder containing the game files.
    pub folder: String,

    /// Full game name.
    pub game: String,

    /// Steam Application ID of the game.
    pub app_id: u16,

    /// Current number of players on the server.
    pub players: u8,

    /// Maximum number of players allowed.
    pub max_players: u8,

    /// Number of bots currently on the server.
    pub bots: u8,

    /// Type of server (dedicated, listen, or SourceTV).
    pub server_type: ServerType,

    /// Operating environment on which the server is running.
    pub server_environment: ServerEnvironment,

    /// Indicates if the server requires a password to join.
    pub password_protected: bool,

    /// `true` if the server uses VAC (Valve Anti Cheat).
    pub vac: bool,

    /// Additional data for servers running `The Ship`.
    pub the_ship: Option<TheShip>,

    /// Version of the game installed on the server.
    pub version: String,

    /// Extra Data Flag that indicates which optional fields are present.
    pub edf: ExtraDataFlag,

    /// Extended server information based on the `edf` flag.
    pub extended_info: Option<Extended>,

    /// Information about SourceTV (if available).
    ///
    /// Present if `edf & 0x40 != 0`.
    pub source_tv: Option<SourceTV>,

    /// Mod information (for `GoldSrc` servers running a mod).
    pub r#mod: Option<Mod>,
}
