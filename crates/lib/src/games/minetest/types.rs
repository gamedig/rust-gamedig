use crate::minetest_master_server::Server;
use crate::protocols::types::{CommonPlayer, CommonResponse, GenericPlayer};
use crate::protocols::GenericResponse;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
}

impl CommonPlayer for Player {
    fn as_original(&self) -> GenericPlayer { GenericPlayer::Minetest(self) }

    fn name(&self) -> &str { &self.name }
}

impl CommonResponse for Server {
    fn as_original(&self) -> GenericResponse { GenericResponse::Minetest(self) }

    fn name(&self) -> Option<&str> { Some(&self.name) }

    fn description(&self) -> Option<&str> { Some(&self.description) }

    fn game_version(&self) -> Option<&str> { Some(&self.version) }

    fn players_maximum(&self) -> u32 { self.clients_max }

    fn players_online(&self) -> u32 { self.clients }

    fn has_password(&self) -> Option<bool> { Some(self.password) }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        None
        // Some(
        // self.clients_list
        // .iter()
        // .map(|p| Player { name: p.clone() })
        // .map(|p| p as &dyn CommonPlayer)
        // .collect(),
        // )
    }
}
