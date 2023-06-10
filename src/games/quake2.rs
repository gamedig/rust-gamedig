use crate::protocols::quake;
use crate::protocols::quake::two::Player;
use crate::protocols::quake::Response;
use crate::GDResult;
use std::net::{IpAddr, SocketAddr};

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response<Player>> {
    quake::two::query(&SocketAddr::new(*address, port.unwrap_or(27910)), None)
}
