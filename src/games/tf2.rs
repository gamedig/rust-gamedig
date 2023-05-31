use crate::{
    protocols::valve::{self, game, SteamApp},
    GDResult, GameInfo, GenericResponse,
};
use std::net::IpAddr;

pub struct TF2Info;
impl GameInfo for TF2Info {
    fn name(&self) -> &'static str {
        "Team Fortress 2"
    }
    fn protocol(&self) -> &'static str {
        "Valve"
    }
    fn query(&self, address: &IpAddr, port: Option<u16>) -> GDResult<Box<dyn GenericResponse>> {
        Ok(query(address, port).map(|r| Box::new(r))?)
    }
}
pub static INFO: &'static TF2Info = &TF2Info;

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<game::Response> {
    let valve_response = valve::query(
        address,
        port.unwrap_or(27015),
        SteamApp::TF2.as_engine(),
        None,
        None,
    )?;

    Ok(game::Response::new_from_valve_response(valve_response))
}
