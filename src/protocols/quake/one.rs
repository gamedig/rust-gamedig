use std::net::IpAddr;
use std::slice::Iter;
use crate::{GDError, GDResult};
use crate::protocols::quake::Response;
use crate::protocols::quake::client::{QuakeClient, client_query, remove_wrapping_quotes};
use crate::protocols::types::TimeoutSettings;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
    pub color_secondary: u8
}

pub(crate) struct QuakeOne;
impl QuakeClient for QuakeOne {
    type Player = Player;

    fn get_send_header<'a>() -> &'a str {
        "status"
    }

    fn get_response_header<'a>() -> &'a str {
        "n"
    }

    fn parse_player_string(mut data: Iter<&str>) -> GDResult<Self::Player> {
        Ok(Player {
            id: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
            },
            score: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
            },
            time: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
            },
            ping: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
            },
            name: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => remove_wrapping_quotes(v).to_string()
            },
            skin: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => remove_wrapping_quotes(v).to_string()
            },
            color_primary: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
            },
            color_secondary: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
            },
        })
    }
}

pub fn query(address: &IpAddr, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response<Player>> {
    client_query::<QuakeOne>(address, port, timeout_settings)
}
