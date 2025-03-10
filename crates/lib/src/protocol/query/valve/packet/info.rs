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

/// Describes the type of server as returned in the server information response.
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
    ///
    /// Returns `None` if the value does not match any known server type.
    #[inline]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            b'd' | b'D' => Some(Self::Dedicated),
            b'l' | b'L' => Some(Self::NonDedicated),
            b'p' | b'P' => Some(Self::SourceTV),
            _ => None,
        }
    }
}

/// Specifies the operating system environment on which the server is running.
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
    ///
    /// Returns `None` if the byte does not correspond to any supported environment.
    #[inline]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            b'l' | b'L' => Some(Self::Linux),
            b'w' | b'W' => Some(Self::Windows),
            b'm' | b'M' | b'o' | b'O' => Some(Self::Mac),
            _ => None,
        }
    }
}

/// Flags indicating which optional extra fields are included in the server response.
///
/// Each flag corresponds to an optional field in the [`ExtraData`] struct.
pub enum ExtraDataFlags {
    /// The server's 64-bit GameID.
    ///
    /// If set, the [`ExtraData::game_id`] field is present.
    GameID = 0x01,

    /// The server's SteamID.
    ///
    /// If set, the [`ExtraData::steam_id`] field is present.
    SteamID = 0x10,

    /// Keywords or tags describing the game.
    ///
    /// If set, the [`ExtraData::keywords`] field is present.
    Keywords = 0x20,

    /// Information about the server’s SourceTV configuration.
    ///
    /// If set, the [`ExtraData::source_tv`] field is present.
    SourceTV = 0x40,

    /// The server’s game port number.
    ///
    /// If set, the [`ExtraData::port`] field is present.
    Port = 0x80,
}

/// A wrapper for a byte representing a combination of extra data flags.
///
/// This structure provides a [`ExtraDataFlag::contains`] method to check if a specific flag is present.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraDataFlag(pub u8);

impl ExtraDataFlag {
    /// Checks whether the provided `flag` is present in the [`ExtraDataFlag`].
    #[inline]
    pub const fn contains(&self, flag: ExtraDataFlags) -> bool { self.0 & flag as u8 != 0 }
}

/// Information about SourceTV, used for live game spectating.
///
/// This struct holds the configuration of the SourceTV service if available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceTV {
    /// The port number used for SourceTV spectating.
    pub port: u16,

    /// The name of the SourceTV server.
    pub name: String,
}

/// Contains optional extra information provided by the server response.
///
/// The presence of each field is determined by the bits set in the
/// [`Response::edf`] (Extra Data Flag).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraData {
    /// The server's 64-bit GameID. This value is more precise than the low 24-bit AppID.
    ///
    /// Present if [`ExtraDataFlags::GameID`] is set.
    pub game_id: Option<u64>,

    /// The server's SteamID.
    ///
    /// Present if [`ExtraDataFlags::SteamID`] is set.
    pub steam_id: Option<u64>,

    /// Keywords or tags that describe the game.
    ///
    /// Present if [`ExtraDataFlags::Keywords`] is set.
    pub keywords: Option<String>,

    /// SourceTV configuration information.
    ///
    /// Present if [`ExtraDataFlags::SourceTV`] is set.
    pub source_tv: Option<SourceTV>,

    /// The server’s game port number.
    ///
    /// Present if [`ExtraDataFlags::Port`] is set.
    pub port: Option<u16>,
}

/// Enumerates the game modes available in the game `The Ship`.
///
/// Some servers running `The Ship` provide additional information about the game mode.
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
    ///
    /// Returns `None` if the value does not correspond to any defined mode.
    #[inline]
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

/// Additional game mode information specific to servers running `The Ship`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheShip {
    /// The game mode used by the server.
    pub mode: Option<TheShipMode>,

    /// The number of witnesses required for a player to be arrested.
    pub witnesses: u8,

    /// Time (in seconds) before a player is arrested once witnessed.
    pub duration: u8,
}

/// Information about a mod running on a GoldSrc engine server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldSrcMod {
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
pub struct ServerInfo {
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
    pub password: bool,

    /// If the server uses VAC (Valve Anti Cheat).
    pub vac: bool,

    /// Version of the game installed on the server.
    pub version: String,

    /// Extra Data Flag that indicates which optional fields are present in [`Response::extra_data`].
    pub edf: ExtraDataFlag,

    /// Extra server information based on the flags set in [`Response::edf`].
    pub extra_data: Option<ExtraData>,

    /// Additional data for servers running `The Ship`.
    pub the_ship: Option<TheShip>,

    /// Mod information for `GoldSrc` servers running a mod.
    pub gold_src_mod: Option<GoldSrcMod>,
}
