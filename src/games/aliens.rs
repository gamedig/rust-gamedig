use crate::{
    protocols::valve::{self, game, SteamApp},
    GDResult, GameInfo, GenericResponse,
};
use std::net::IpAddr;

pub struct AliensInfo;
impl GameInfo for AliensInfo {
    fn name(&self) -> &'static str {
        "Aliens"
    }
    fn protocol(&self) -> &'static str {
        "Valve"
    }
    fn query(&self, address: &IpAddr, port: Option<u16>) -> GDResult<Box<dyn GenericResponse>> {
        Ok(query(address, port).map(|r| Box::new(r))?)
    }
}
pub static INFO: &'static AliensInfo = &AliensInfo;

/*
pub const INFO: GameInfo = GameInfo {
    name: "Aliens",
    protocol: "valve",
    query: query,
};
*/

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<game::Response> {
    let valve_response = valve::query(
        address,
        port.unwrap_or(27015),
        SteamApp::ALIENS.as_engine(),
        None,
        None,
    )?;

    Ok(game::Response::new_from_valve_response(valve_response))
}
