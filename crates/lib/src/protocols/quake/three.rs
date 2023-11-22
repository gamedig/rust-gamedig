use crate::protocols::quake::client::{client_query, QuakeClient};
use crate::protocols::quake::two::QuakeTwo;
use crate::protocols::quake::Response;
use crate::{GDResult, TimeoutSettings};

use std::net::SocketAddr;
use std::slice::Iter;

pub use crate::protocols::quake::two::Player;

struct QuakeThree;
impl QuakeClient for QuakeThree {
    type Player = Player;

    fn get_send_header<'a>() -> &'a str { "getstatus" }

    fn get_response_header<'a>() -> &'a str { "statusResponse\n" }

    fn parse_player_string(data: Iter<&str>) -> GDResult<Self::Player> { QuakeTwo::parse_player_string(data) }
}

pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response<Player>> {
    client_query::<QuakeThree>(address, timeout_settings)
}
