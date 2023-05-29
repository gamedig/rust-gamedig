use std::net::IpAddr;
use std::slice::Iter;
use crate::{GDError, GDResult};
use crate::protocols::quake::one::QuakeOne;
use crate::protocols::quake::types::{QuakeClient, Response, client_query};
use crate::protocols::types::TimeoutSettings;

#[derive(Debug)]
pub struct Player {
    pub frags: u8,
    pub ping: u8,
    pub name: String
}

pub(crate) struct QuakeTwo;
impl QuakeClient for QuakeTwo {
    type Player = Player;

    fn get_send_header<'a>() -> &'a str {
        QuakeOne::get_send_header()
    }

    fn get_response_header<'a>() -> &'a [u8] {
        &[0x70, 0x72, 0x69, 0x6E, 0x74, 0x0A]
    }

    fn parse_player_string(mut data: Iter<&str>) -> GDResult<Self::Player> {
        Ok(Player {
            frags: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
            },
            ping: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
            },
            name: match data.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => match v.starts_with('\"') && v.ends_with('\"') {
                    false => v,
                    true => &v[1..v.len() - 1]
                }.to_string()
            }
        })
    }
}

pub fn query(address: &IpAddr, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response<Player>> {
    client_query::<QuakeTwo>(address, port, timeout_settings)
}
