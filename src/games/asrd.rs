use std::net::{IpAddr, SocketAddr};
use crate::{
    protocols::valve::{self, game, SteamApp},
    GDResult,
};

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<game::Response> {
    let valve_response = valve::query(
        &SocketAddr::new(*address, port.unwrap_or(2304)),
        SteamApp::ASRD.as_engine(),
        None,
        None,
    )?;

    Ok(game::Response::new_from_valve_response(valve_response))
}
