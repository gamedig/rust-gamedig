use crate::buffer::Buffer;
use crate::errors::GDErrorKind::PacketBad;
use crate::protocols::types::{CommonPlayer, CommonResponse, ExtraRequestSettings, GenericPlayer};
use crate::protocols::GenericResponse;
use crate::{GDError, GDResult};

use super::Unreal2StringDecoder;

use std::collections::{HashMap, HashSet};

use byteorder::ByteOrder;

/// Unreal 2 packet types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PacketKind {
    ServerInfo = 0,
    MutatorsAndRules = 1,
    Players = 2,
}

impl TryFrom<u8> for PacketKind {
    type Error = GDError;
    fn try_from(value: u8) -> GDResult<Self> {
        match value {
            0 => Ok(Self::ServerInfo),
            1 => Ok(Self::MutatorsAndRules),
            2 => Ok(Self::Players),
            _ => Err(PacketBad.context("Unknown packet type")),
        }
    }
}

/// Unreal 2 server info.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ServerInfo {
    pub server_id: u32,
    pub ip: String,
    pub game_port: u32,
    pub query_port: u32,
    pub name: String,
    pub map: String,
    pub game_type: String,
    pub num_players: u32,
    pub max_players: u32,
}

impl ServerInfo {
    pub fn parse<B: ByteOrder>(buffer: &mut Buffer<B>) -> GDResult<Self> {
        Ok(ServerInfo {
            server_id: buffer.read()?,
            ip: buffer.read_string::<Unreal2StringDecoder>(None)?,
            game_port: buffer.read()?,
            query_port: buffer.read()?,
            name: buffer.read_string::<Unreal2StringDecoder>(None)?,
            map: buffer.read_string::<Unreal2StringDecoder>(None)?,
            game_type: buffer.read_string::<Unreal2StringDecoder>(None)?,
            num_players: buffer.read()?,
            max_players: buffer.read()?,
        })
    }
}

/// Unreal 2 mutators and rules.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MutatorsAndRules {
    pub mutators: HashSet<String>,
    pub rules: HashMap<String, Vec<String>>,
}

impl MutatorsAndRules {
    pub fn parse<B: ByteOrder>(&mut self, buffer: &mut Buffer<B>) -> GDResult<()> {
        while buffer.remaining_length() > 0 {
            let key = buffer.read_string::<Unreal2StringDecoder>(None)?;
            let value = buffer.read_string::<Unreal2StringDecoder>(None).ok();

            if key.eq_ignore_ascii_case("mutator") {
                if let Some(value) = value {
                    self.mutators.insert(value);
                }
            } else {
                let rule_vec = self.rules.get_mut(&key);

                let rule_vec = if let Some(rule_vec) = rule_vec {
                    rule_vec
                } else {
                    self.rules.insert(key.clone(), Vec::default());
                    self.rules
                        .get_mut(&key)
                        .expect("Value should be in HashMap after we inserted")
                };

                if let Some(value) = value {
                    rule_vec.push(value);
                }
            }
        }
        Ok(())
    }
}

/// Unreal 2 players and bots.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Players {
    /// List of players returned by server (without 0 ping).
    pub players: Vec<Player>,
    /// List of bots returned by server (players with 0 ping).
    pub bots: Vec<Player>,
}

impl Players {
    /// Pre-allocate the vectors inside the players struct based on the provided
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Players {
            players: Vec::with_capacity(capacity),
            // Allocate half as many bots as we don't expect there to be as many
            bots: Vec::with_capacity(capacity / 2),
        }
    }

    /// Parse a raw buffer of players into the current struct.
    pub fn parse<B: ByteOrder>(&mut self, buffer: &mut Buffer<B>) -> GDResult<()> {
        while buffer.remaining_length() > 0 {
            let player = Player {
                id: buffer.read()?,
                name: buffer.read_string::<Unreal2StringDecoder>(None)?,
                ping: buffer.read()?,
                score: buffer.read()?,
                stats_id: buffer.read()?,
            };

            // If ping is 0 the player is a bot
            if player.ping == 0 {
                self.bots.push(player);
            } else {
                self.players.push(player);
            }
        }

        Ok(())
    }

    /// Length of both players and bots.
    pub fn total_len(&self) -> usize { self.players.len() + self.bots.len() }
}

/// Unreal 2 player info.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub ping: u32,
    pub score: i32,
    pub stats_id: u32,
}

impl CommonPlayer for Player {
    fn name(&self) -> &str { &self.name }

    fn score(&self) -> Option<i32> { Some(self.score) }

    fn as_original(&self) -> GenericPlayer { GenericPlayer::Unreal2(self) }
}

/// Unreal 2 response.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Response {
    pub server_info: ServerInfo,
    pub mutators_and_rules: MutatorsAndRules,
    pub players: Players,
}

impl CommonResponse for Response {
    fn map(&self) -> Option<&str> { Some(&self.server_info.map) }

    fn name(&self) -> Option<&str> { Some(&self.server_info.name) }

    fn game_mode(&self) -> Option<&str> { Some(&self.server_info.game_type) }

    fn players_online(&self) -> u32 { self.server_info.num_players }

    fn players_maximum(&self) -> u32 { self.server_info.max_players }

    fn players(&self) -> Option<Vec<&dyn crate::protocols::types::CommonPlayer>> {
        Some(
            self.players
                .players
                .iter()
                .map(|player| player as _)
                .collect(),
        )
    }

    fn as_original(&self) -> GenericResponse { GenericResponse::Unreal2(self) }
}

/// What data to gather, purely used only with the query function.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GatheringSettings {
    pub players: bool,
    pub mutators_and_rules: bool,
}

impl GatheringSettings {
    /// Default values are true for both the players and the rules.
    pub const fn default() -> Self {
        Self {
            players: true,
            mutators_and_rules: true,
        }
    }

    pub const fn into_extra(self) -> ExtraRequestSettings {
        ExtraRequestSettings {
            hostname: None,
            protocol_version: None,
            gather_players: Some(self.players),
            gather_rules: Some(self.mutators_and_rules),
            check_app_id: None,
        }
    }
}

impl Default for GatheringSettings {
    fn default() -> Self { GatheringSettings::default() }
}

impl From<ExtraRequestSettings> for GatheringSettings {
    fn from(value: ExtraRequestSettings) -> Self {
        let default = Self::default();
        Self {
            players: value.gather_players.unwrap_or(default.players),
            mutators_and_rules: value.gather_rules.unwrap_or(default.mutators_and_rules),
        }
    }
}

// TODO: Add tests
