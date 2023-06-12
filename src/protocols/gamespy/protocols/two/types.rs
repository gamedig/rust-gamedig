#[derive(Debug)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub has_password: bool,
    pub max_players: u8,
}
