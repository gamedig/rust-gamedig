use crate::buffer::Buffer;
use crate::errors::GDErrorKind::PacketBad;
use crate::protocols::types::{CommonPlayer, CommonResponse, GenericPlayer};
use crate::protocols::GenericResponse;
use crate::{GDError, GDResult};

use super::Unreal2StringDecoder;

use std::collections::HashMap;

use byteorder::ByteOrder;

/// Unreal 2 packet types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
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
#[derive(Clone, Debug, PartialEq)]
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
        let server_id = buffer.read()?;
        let ip = buffer.read_string::<Unreal2StringDecoder>(None)?;
        let game_port = buffer.read()?;
        let query_port = buffer.read()?;
        let name = buffer.read_string::<Unreal2StringDecoder>(None)?;
        let map = buffer.read_string::<Unreal2StringDecoder>(None)?;
        let game_type = buffer.read_string::<Unreal2StringDecoder>(None)?;
        let num_players = buffer.read()?;
        let max_players = buffer.read()?;

        Ok(ServerInfo {
            server_id,
            ip,
            game_port,
            query_port,
            name,
            map,
            game_type,
            num_players,
            max_players,
        })
    }
}

/// Unreal 2 mutators and rules.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MutatorsAndRules {
    pub mutators: Vec<String>,
    pub rules: HashMap<String, Option<String>>,
}

impl MutatorsAndRules {
    pub fn parse<B: ByteOrder>(&mut self, buffer: &mut Buffer<B>) -> GDResult<()> {
        while buffer.remaining_length() > 0 {
            let key = buffer.read_string::<Unreal2StringDecoder>(None)?;
            let value = buffer.read_string::<Unreal2StringDecoder>(None).ok();

            if key.eq_ignore_ascii_case("mutator") {
                if let Some(value) = value {
                    self.mutators.push(value);
                }
            } else {
                // TODO: Node combines multiple rule occurences with ,
                self.rules.insert(key, value);
            }
        }
        Ok(())
    }
}

/// Unreal 2 players.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Players {
    pub players: Vec<Player>,
    pub bots: Vec<Player>,
}

impl Players {
    pub fn with_capacity(capacity: usize) -> Self {
        Players {
            players: Vec::with_capacity(capacity),
            bots: Vec::with_capacity(capacity),
        }
    }

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
}

/// Unreal 2 player info.
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
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

// TODO: Add tests
