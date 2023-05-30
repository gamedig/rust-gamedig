use std::net::IpAddr;
use crate::GDResult;
use crate::protocols::quake;
use crate::protocols::quake::Response;
use crate::protocols::quake::two::Player;

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response<Player>> {
    quake::three::query(address, port.unwrap_or(27960), None)
}
