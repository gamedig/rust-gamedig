use std::net::IpAddr;
use std::slice::Iter;
use crate::GDResult;
use crate::protocols::quake::two::{Player, QuakeTwo};
use crate::protocols::quake::Response;
use crate::protocols::quake::client::{QuakeClient, client_query};
use crate::protocols::types::TimeoutSettings;

struct QuakeThree;
impl QuakeClient for QuakeThree {
    type Player = Player;

    fn get_send_header<'a>() -> &'a str {
        "getstatus"
    }

    fn get_response_header<'a>() -> &'a [u8] {
        &[0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6F, 0x6E, 0x73, 0x65, 0x0A]
    }

    fn parse_player_string(data: Iter<&str>) -> GDResult<Self::Player> {
        QuakeTwo::parse_player_string(data)
    }
}

pub fn query(address: &IpAddr, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response<Player>> {
    client_query::<QuakeThree>(address, port, timeout_settings)
}
