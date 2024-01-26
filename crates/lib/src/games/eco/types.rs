use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;

use crate::protocols::types::CommonPlayer;
use crate::protocols::types::CommonResponse;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "Info")]
    pub info: Info,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(rename = "External")]
    pub external: bool,
    #[serde(rename = "GamePort")]
    pub game_port: u32,
    #[serde(rename = "WebPort")]
    pub web_port: u32,
    #[serde(rename = "IsLAN")]
    pub is_lan: bool,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "DetailedDescription")]
    pub detailed_description: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "OnlinePlayers")]
    pub online_players: u32,
    #[serde(rename = "TotalPlayers")]
    pub total_players: u32,
    #[serde(rename = "OnlinePlayersNames")]
    pub online_players_names: Vec<String>,
    #[serde(rename = "AdminOnline")]
    pub admin_online: bool,
    #[serde(rename = "TimeSinceStart")]
    pub time_since_start: f64,
    #[serde(rename = "TimeLeft")]
    pub time_left: f64,
    #[serde(rename = "Animals")]
    pub animals: u32,
    #[serde(rename = "Plants")]
    pub plants: u32,
    #[serde(rename = "Laws")]
    pub laws: u32,
    #[serde(rename = "WorldSize")]
    pub world_size: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "EconomyDesc")]
    pub economy_desc: String,
    #[serde(rename = "SkillSpecializationSetting")]
    pub skill_specialization_setting: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "HasPassword")]
    pub has_password: bool,
    #[serde(rename = "HasMeteor")]
    pub has_meteor: bool,
    #[serde(rename = "DistributionStationItems")]
    pub distribution_station_items: String,
    #[serde(rename = "Playtimes")]
    pub playtimes: String,
    #[serde(rename = "DiscordAddress")]
    pub discord_address: String,
    #[serde(rename = "IsPaused")]
    pub is_paused: bool,
    #[serde(rename = "ActiveAndOnlinePlayers")]
    pub active_and_online_players: u32,
    #[serde(rename = "PeakActivePlayers")]
    pub peak_active_players: u32,
    #[serde(rename = "MaxActivePlayers")]
    pub max_active_players: u32,
    #[serde(rename = "ShelfLifeMultiplier")]
    pub shelf_life_multiplier: f64,
    #[serde(rename = "ExhaustionAfterHours")]
    pub exhaustion_after_hours: f64,
    #[serde(rename = "IsLimitingHours")]
    pub is_limiting_hours: bool,
    #[serde(rename = "ServerAchievementsDict")]
    pub server_achievements_dict: HashMap<String, String>,
    #[serde(rename = "RelayAddress")]
    pub relay_address: String,
    #[serde(rename = "Access")]
    pub access: String,
    #[serde(rename = "JoinUrl")]
    pub join_url: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
}

impl CommonPlayer for Player {
    fn as_original(&self) -> crate::protocols::types::GenericPlayer {
        crate::protocols::types::GenericPlayer::Eco(self)
    }

    fn name(&self) -> &str { &self.name }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub external: bool,
    pub port: u32,
    pub query_port: u32,
    pub is_lan: bool,
    pub description: String, // this and other fields require some text filtering
    pub description_detailed: String,
    pub description_economy: String,
    pub category: String,
    pub players_online: u32,
    pub players_maximum: u32,
    pub players: Vec<Player>,
    pub admin_online: bool,
    pub time_since_start: f64,
    pub time_left: f64,
    pub animals: u32,
    pub plants: u32,
    pub laws: u32,
    pub world_size: String,
    pub game_version: String,
    pub skill_specialization_setting: String,
    pub language: String,
    pub has_password: bool,
    pub has_meteor: bool,
    pub distribution_station_items: String,
    pub playtimes: String,
    pub discord_address: String,
    pub is_paused: bool,
    pub active_and_online_players: u32,
    pub peak_active_players: u32,
    pub max_active_players: u32,
    pub shelf_life_multiplier: f64,
    pub exhaustion_after_hours: f64,
    pub is_limiting_hours: bool,
    pub server_achievements_dict: HashMap<String, String>,
    pub relay_address: String,
    pub access: String,
    pub connect: String,
}

impl From<Root> for Response {
    fn from(root: Root) -> Self {
        let value = root.info;
        Self {
            external: value.external,
            port: value.game_port,
            query_port: value.web_port,
            is_lan: value.is_lan,
            description: value.description,
            description_detailed: value.detailed_description,
            description_economy: value.economy_desc,
            category: value.category,
            players_online: value.online_players,
            players_maximum: value.total_players,
            players: value
                .online_players_names
                .iter()
                .map(|player| {
                    Player {
                        name: player.clone(),
                    }
                })
                .collect(),
            admin_online: value.admin_online,
            time_since_start: value.time_since_start,
            time_left: value.time_left,
            animals: value.animals,
            plants: value.plants,
            laws: value.laws,
            world_size: value.world_size,
            game_version: value.version,
            skill_specialization_setting: value.skill_specialization_setting,
            language: value.language,
            has_password: value.has_password,
            has_meteor: value.has_meteor,
            distribution_station_items: value.distribution_station_items,
            playtimes: value.playtimes,
            discord_address: value.discord_address,
            is_paused: value.is_paused,
            active_and_online_players: value.active_and_online_players,
            peak_active_players: value.peak_active_players,
            max_active_players: value.max_active_players,
            shelf_life_multiplier: value.shelf_life_multiplier,
            exhaustion_after_hours: value.exhaustion_after_hours,
            is_limiting_hours: value.is_limiting_hours,
            server_achievements_dict: value.server_achievements_dict,
            relay_address: value.relay_address,
            access: value.access,
            connect: value.join_url,
        }
    }
}

impl CommonResponse for Response {
    fn as_original(&self) -> crate::protocols::GenericResponse { crate::protocols::GenericResponse::Eco(self) }

    fn players_online(&self) -> u32 { self.players_online }

    fn players_maximum(&self) -> u32 { self.players_maximum }

    fn description(&self) -> Option<&str> { Some(&self.description) }

    fn game_version(&self) -> Option<&str> { Some(&self.game_version) }

    fn has_password(&self) -> Option<bool> { Some(self.has_password) }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> { Some(self.players.iter().map(|p| p as _).collect()) }
}
