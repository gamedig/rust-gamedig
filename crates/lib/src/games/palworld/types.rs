use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {}

pub enum Endpoint {
    Info,
    Players,
    Settings,
    Metrics,
}

impl<'a> Into<&'a str> for Endpoint {
    fn into(self) -> &'a str {
        match self {
            Endpoint::Info => "info",
            Endpoint::Players => "players",
            Endpoint::Settings => "settings",
            Endpoint::Metrics => "metrics",
        }
    }
}
