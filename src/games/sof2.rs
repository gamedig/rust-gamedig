use std::net::{IpAddr, SocketAddr};
use crate::GDResult;
use crate::protocols::quake;
use crate::protocols::quake::Response;
use crate::protocols::quake::two::Player;

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response<Player>> {
    quake::three::query(&SocketAddr::new(*address, port.unwrap_or(20100)), None)
}
