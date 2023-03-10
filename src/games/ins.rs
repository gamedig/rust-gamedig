use crate::protocols::valve;
use crate::protocols::valve::{game, SteamApp};
use crate::GDResult;

pub fn query(address: &str, port: Option<u16>) -> GDResult<game::Response> {
    let valve_response = valve::query(
        address,
        port.unwrap_or(27015),
        SteamApp::INS.as_engine(),
        None,
        None,
    )?;

    Ok(game::Response::new_from_valve_response(valve_response))
}
