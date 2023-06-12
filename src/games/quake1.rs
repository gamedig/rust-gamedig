use crate::protocols::quake;
use crate::protocols::quake::one::Player;
use crate::protocols::quake::Response;
use crate::GDResult;
use std::net::{IpAddr, SocketAddr};

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response<Player>> {
    quake::one::query(&SocketAddr::new(*address, port.unwrap_or(27500)), None)
}
