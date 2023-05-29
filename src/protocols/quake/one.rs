use std::net::Ipv4Addr;
use std::slice::Iter;
use crate::{GDError, GDResult};
use crate::protocols::quake::types::{QuakeClient, Response, client_query};
use crate::protocols::types::TimeoutSettings;

#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub score: u8,
    pub time: u8,
    pub ping: u8,
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

    fn get_response_header<'a>() -> &'a [u8] {
        &[0x6E]
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
                Some(v) => v.to_string()
            },
            skin: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.to_string()
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

pub fn query(address: &Ipv4Addr, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response<Player>> {
    client_query::<QuakeOne>(address, port, timeout_settings)
}
