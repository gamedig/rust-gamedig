/// The fixed payload used to request server rules.
///
/// This 5 byte payload is sent to the server to request its list of rules.
/// The payload is structured as follows:
///
/// - **Header:** `0xFF, 0xFF, 0xFF, 0xFF`
/// - **Request Type:** `0x56`
pub const RULES_REQUEST_PAYLOAD: [u8; 5] = [0xFF, 0xFF, 0xFF, 0xFF, 0x56];

/// A single server rule represented as a key value pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    /// The name/key of the rule.
    pub name: String,

    /// The value associated with the rule.
    pub value: String,
}

/// The complete response from a server rules query.
///
/// Contains a list of rules provided by the server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    /// A vector containing all the rules as key value pairs.
    pub rules: Vec<Rule>,
}
