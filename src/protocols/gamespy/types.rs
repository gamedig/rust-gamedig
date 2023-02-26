use std::collections::HashMap;

/// A query response.
#[derive(Debug)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub players_maximum: usize,
    pub unused_entries: HashMap<String, String>
}
