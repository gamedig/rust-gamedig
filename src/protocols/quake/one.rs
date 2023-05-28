use std::net::Ipv4Addr;
use std::slice::Iter;
use crate::bufferer::Bufferer;
use crate::{GDError, GDResult};
use crate::protocols::quake::types::{QuakeClient, Response, client_query};
use crate::protocols::types::TimeoutSettings;

#[derive(Debug)]
pub struct Player {
    pub frags: u8,
    pub ping: u8,
    pub name: String
}

struct QuakeOne;
impl QuakeClient for QuakeOne {
    type Player = Player;

    fn get_send_header() -> String {
        "status".to_string()
    }

    fn validate_received_data(bufferer: &mut Bufferer) -> GDResult<()> {
        if bufferer.get_string_utf8_newline()? == "print".to_string() {
            Ok(())
        } else {
            Err(GDError::PacketBad)
        }
    }

    fn parse_player_string(data: Iter<&str>) -> GDResult<Self::Player> {
        let mut data_iter = data;

        let frags = match data_iter.next() {
            None => Err(GDError::PacketBad)?,
            Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
        };

        let ping = match data_iter.next() {
            None => Err(GDError::PacketBad)?,
            Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
        };

        let name = match data_iter.next() {
            None => Err(GDError::PacketBad)?,
            Some(v) => match v.starts_with("\"") && v.ends_with("\"") {
                false => v,
                true => &v[1..v.len() - 1]
            }.to_string()
        };

        Ok(Player {
            frags,
            ping,
            name
        })
    }
}

pub fn query(address: &Ipv4Addr, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response<Player>> {
    client_query::<QuakeOne>(address, port, timeout_settings)
}
