use crate::protocols::gamespy;
use crate::protocols::gamespy::two::Response;
use crate::GDResult;
use std::net::{IpAddr, SocketAddr};

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> {
    gamespy::two::query(&SocketAddr::new(*address, port.unwrap_or(2302)), None)
}
