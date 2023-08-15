use crate::protocols::quake::client::{client_query, remove_wrapping_quotes, QuakeClient};
use crate::protocols::quake::one::QuakeOne;
use crate::protocols::quake::Response;
use crate::protocols::types::{CommonPlayer, GenericPlayer, TimeoutSettings};
use crate::GDErrorKind::TypeParse;
use crate::{GDErrorKind, GDResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::slice::Iter;

use super::QuakePlayerType;

/// Quake 2 player data.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub score: i32,
    pub ping: u16,
    pub name: String,
    pub address: Option<String>,
}

impl QuakePlayerType for Player {
    fn version(response: &Response<Self>) -> super::VersionedResponse {
        super::VersionedResponse::TwoAndThree(response)
    }
}

impl CommonPlayer for Player {
    fn as_original(&self) -> GenericPlayer { GenericPlayer::QuakeTwo(self) }

    fn name(&self) -> &str { &self.name }

    fn score(&self) -> Option<i32> { Some(self.score) }
}

pub(crate) struct QuakeTwo;
impl QuakeClient for QuakeTwo {
    type Player = Player;

    fn get_send_header<'a>() -> &'a str { QuakeOne::get_send_header() }

    fn get_response_header<'a>() -> &'a str { "print\n" }

    fn parse_player_string(mut data: Iter<&str>) -> GDResult<Self::Player> {
        Ok(Player {
            score: match data.next() {
                None => Err(GDErrorKind::PacketBad)?,
                Some(v) => v.parse().map_err(|e| TypeParse.context(e))?,
            },
            ping: match data.next() {
                None => Err(GDErrorKind::PacketBad)?,
                Some(v) => v.parse().map_err(|e| TypeParse.context(e))?,
            },
            name: match data.next() {
                None => Err(GDErrorKind::PacketBad)?,
                Some(v) => remove_wrapping_quotes(v).to_string(),
            },
            address: data.next().map(|v| remove_wrapping_quotes(v).to_string()),
        })
    }
}

pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response<Player>> {
    client_query::<QuakeTwo>(address, timeout_settings)
}
