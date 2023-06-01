use std::net::{IpAddr, SocketAddr};
use crate::protocols::gamespy;
use crate::protocols::gamespy::three::Response;
use crate::GDResult;

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> {
    gamespy::three::query(&SocketAddr::new(*address, port.unwrap_or(64100)), None)
}
