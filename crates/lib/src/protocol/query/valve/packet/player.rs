/// The fixed payload used to request player information.
///
/// This 5 byte payload is sent to the server to obtain a list of connected players
/// and their respective details.
/// The payload structure is:
///
/// - **Header:** `0xFF, 0xFF, 0xFF, 0xFF`
/// - **Request Type:** `0x55`
pub const PLAYER_REQUEST_PAYLOAD: [u8; 5] = [0xFF, 0xFF, 0xFF, 0xFF, 0x55];

/// Additional player statistics specific to "The Ship".
///
/// Some servers running "The Ship" provide extra data about each player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheShipPlayer {
    /// Number of times the player has died.
    pub deaths: u32,

    /// The amount of in game money the player has.
    pub money: u32,
}

/// Represents an individual player in the server.
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    /// Index of the player in the response (starting from 0).
    pub index: u8,

    /// Player’s display name.
    pub name: String,

    /// Player’s score.
    pub score: i32,

    /// Duration (in seconds) that the player has been connected to the server.
    pub duration: f32,

    /// Optional additional information for players on `The Ship`.
    pub the_ship: Option<TheShipPlayer>,
}


