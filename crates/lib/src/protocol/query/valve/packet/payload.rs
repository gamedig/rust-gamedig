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

/// The fixed payload used to request player information.
///
/// This 5 byte payload is sent to the server to obtain a list of connected players
/// and their respective details.
/// The payload structure is:
///
/// - **Header:** `0xFF, 0xFF, 0xFF, 0xFF`
/// - **Request Type:** `0x55`
pub const PLAYER_REQUEST_PAYLOAD: [u8; 5] = [0xFF, 0xFF, 0xFF, 0xFF, 0x55];

/// The fixed payload used to request server rules.
///
/// This 5 byte payload is sent to the server to request its list of rules.
/// The payload is structured as follows:
///
/// - **Header:** `0xFF, 0xFF, 0xFF, 0xFF`
/// - **Request Type:** `0x56`
pub const RULES_REQUEST_PAYLOAD: [u8; 5] = [0xFF, 0xFF, 0xFF, 0xFF, 0x56];
