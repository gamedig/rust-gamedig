/// The fixed payload used to request server rules.
///
/// This 5 byte payload is sent to the server to request its list of rules.
/// The payload is structured as follows:
///
/// - **Header:** `0xFF, 0xFF, 0xFF, 0xFF`
/// - **Request Type:** `0x56`
pub const RULES_REQUEST_PAYLOAD: [u8; 5] = [0xFF, 0xFF, 0xFF, 0xFF, 0x56];

/// Represents a list of server rules in key value format.
pub type Rules = std::collections::HashMap<String, String>;
