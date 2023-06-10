use crate::protocols::quake::client::{client_query, remove_wrapping_quotes, QuakeClient};
use crate::protocols::quake::one::QuakeOne;
use crate::protocols::quake::Response;
use crate::protocols::types::TimeoutSettings;
use crate::{GDError, GDResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::slice::Iter;

/// Quake 2 player data.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub frags: i16,
    pub ping: u16,
    pub name: String,
}

pub(crate) struct QuakeTwo;
impl QuakeClient for QuakeTwo {
    type Player = Player;

    fn get_send_header<'a>() -> &'a str { QuakeOne::get_send_header() }

    fn get_response_header<'a>() -> &'a str { "print\n" }

    fn parse_player_string(mut data: Iter<&str>) -> GDResult<Self::Player> {
        Ok(Player {
            frags: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?,
            },
            ping: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?,
            },
            name: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => remove_wrapping_quotes(v).to_string(),
            },
        })
    }
}

pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response<Player>> {
    client_query::<QuakeTwo>(address, timeout_settings)
}
