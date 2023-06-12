#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub score: u16,
}

#[derive(Debug)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub has_password: bool,
    pub max_players: u8,
    pub teams: Vec<Team>,
}
