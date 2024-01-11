use crate::{
    protocols::types::{CommonResponse, GenericResponse},
    GDErrorKind,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Mindustry sever data
///
/// [Reference](https://github.com/Anuken/Mindustry/blob/a2e5fbdedb2fc1c8d3c157bf344d10ad6d321442/core/src/mindustry/net/NetworkIO.java#L122-L135)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ServerData {
    pub host: String,
    pub map: String,
    pub players: i32,
    pub wave: i32,
    pub version: i32,
    pub version_type: String,
    pub gamemode: GameMode,
    pub player_limit: i32,
    pub description: String,
    pub mode_name: Option<String>,
}

/// Mindustry game mode
///
/// [Reference](https://github.com/Anuken/Mindustry/blob/a2e5fbdedb2fc1c8d3c157bf344d10ad6d321442/core/src/mindustry/game/Gamemode.java)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum GameMode {
    Survival,
    Sandbox,
    Attack,
    PVP,
    Editor,
}

impl TryFrom<u8> for GameMode {
    type Error = GDErrorKind;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GameMode::*;
        Ok(match value {
            0 => Survival,
            1 => Sandbox,
            2 => Attack,
            3 => PVP,
            4 => Editor,
            _ => return Err(GDErrorKind::TypeParse),
        })
    }
}

impl GameMode {
    fn as_str(&self) -> &'static str {
        use GameMode::*;
        match self {
            Survival => "survival",
            Sandbox => "sandbox",
            Attack => "attack",
            PVP => "pvp",
            Editor => "editor",
        }
    }
}

impl CommonResponse for ServerData {
    fn as_original(&self) -> GenericResponse { GenericResponse::Mindustry(self) }

    fn players_online(&self) -> u32 { self.players.try_into().unwrap_or(0) }
    fn players_maximum(&self) -> u32 { self.player_limit.try_into().unwrap_or(0) }

    fn game_mode(&self) -> Option<&str> { Some(self.gamemode.as_str()) }

    fn map(&self) -> Option<&str> { Some(&self.map) }
    fn description(&self) -> Option<&str> { Some(&self.description) }
}

#[cfg(test)]
mod test {
    use crate::protocols::types::CommonResponse;

    use super::ServerData;

    #[test]
    fn common_impl() {
        let data = ServerData {
            host: String::from("host"),
            map: String::from("map"),
            players: 5,
            wave: 2,
            version: 142,
            version_type: String::from("steam"),
            gamemode: super::GameMode::PVP,
            player_limit: 20,
            description: String::from("description"),
            mode_name: Some(String::from("campaign")),
        };

        let common: &dyn CommonResponse = &data;

        assert_eq!(common.players_online(), 5);
        assert_eq!(common.players_maximum(), 20);
        assert_eq!(common.game_mode(), Some("pvp"));
        assert_eq!(common.map(), Some("map"));
        assert_eq!(common.description(), Some("description"));
    }
}
