pub(crate) struct Fragment {
    pub(crate) number: u8,
    pub(crate) payload: Vec<u8>,
}

/// Describes the type of server as returned in the server information response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerType {
    /// A dedicated server.
    Dedicated,

    /// A non dedicated (listen) server.
    NonDedicated,

    /// A SourceTV server (HLTV will be identified as this type as there is no way to distinguish them).
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
    #[inline]
    pub const fn from_u8(value: u8) -> Self {
        match value {
            b'd' | b'D' => Self::Dedicated,
            b'l' | b'L' => Self::NonDedicated,
            b'p' | b'P' => Self::SourceTV,
            _ => unreachable!(),
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
    pub const fn from_u8(value: u8) -> Self {
        match value {
            b'l' | b'L' => Self::Linux,
            b'w' | b'W' => Self::Windows,
            b'm' | b'M' | b'o' | b'O' => Self::Mac,
            _ => unreachable!(),
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
    pub game_app_id_64: Option<u64>,

    /// The server's SteamID.
    ///
    /// Present if [`ExtraDataFlags::SteamID`] is set.
    pub server_steam_id: Option<u64>,

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

/// Additional player statistics specific to "The Ship".
///
/// Some servers running "The Ship" provide extra data about each player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheShipPlayer {
    /// Number of times the player has died.
    pub deaths: i32,

    /// The amount of in game money the player has.
    pub money: i32,
}

/// Represents an individual player in the server.
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    /// Player’s display name.
    pub name: String,

    /// Player’s score.
    pub score: i32,

    /// Duration (in seconds) that the player has been connected to the server.
    pub duration: f32,

    /// Optional additional information for players on `The Ship`.
    pub the_ship: Option<TheShipPlayer>,
}

/// Enumerates the game modes available in the game `The Ship`.
///
/// Some servers running `The Ship` provide additional information about the game mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheShipMode {
    /// Hunt mode.
    Hunt,

    /// Elimination mode.
    Elimination,

    /// Duel mode.
    Duel,

    /// Deathmatch mode.
    Deathmatch,

    /// VIP Team mode.
    VIPTeam,

    /// Team Elimination mode.
    TeamElimination,
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
