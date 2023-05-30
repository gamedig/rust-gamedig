use std::net::IpAddr;
use crate::GDResult;
use crate::protocols::quake;
use crate::protocols::quake::Response;
use crate::protocols::quake::one::Player;

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response<Player>> {
    quake::one::query(address, port.unwrap_or(27500), None)
}
