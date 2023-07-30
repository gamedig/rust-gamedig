use crate::protocols::quake::client::{client_query, remove_wrapping_quotes, QuakeClient};
use crate::protocols::quake::Response;
use crate::protocols::types::{CommonPlayer, GenericPlayer, TimeoutSettings};
use crate::GDError::TypeParse;
use crate::{GDError, GDResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::slice::Iter;

use super::QuakePlayerType;

/// Quake 1 player data.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    /// Player's server id.
    pub id: u8,
    pub score: u16,
    pub time: u16,
    pub ping: u16,
    pub name: String,
    pub skin: String,
    pub color_primary: u8,
    pub color_secondary: u8,
}

impl QuakePlayerType for Player {
    fn version(response: &Response<Self>) -> super::VersionedResponse { super::VersionedResponse::One(response) }
}

impl CommonPlayer for Player {
    fn as_original(&self) -> GenericPlayer { GenericPlayer::QuakeOne(self) }

    fn name(&self) -> &str { &self.name }
    fn score(&self) -> Option<u32> { Some(self.score.into()) }
}

pub(crate) struct QuakeOne;
impl QuakeClient for QuakeOne {
    type Player = Player;

    fn get_send_header<'a>() -> &'a str { "status" }

    fn get_response_header<'a>() -> &'a str { "n" }

    fn parse_player_string(mut data: Iter<&str>) -> GDResult<Self::Player> {
        Ok(Player {
            id: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|e| TypeParse.rich(e))?,
            },
            score: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|e| TypeParse.rich(e))?,
            },
            time: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|e| TypeParse.rich(e))?,
            },
            ping: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|e| TypeParse.rich(e))?,
            },
            name: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => remove_wrapping_quotes(v).to_string(),
            },
            skin: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => remove_wrapping_quotes(v).to_string(),
            },
            color_primary: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|e| TypeParse.rich(e))?,
            },
            color_secondary: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|e| TypeParse.rich(e))?,
            },
        })
    }
}

pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response<Player>> {
    client_query::<QuakeOne>(address, timeout_settings)
}
