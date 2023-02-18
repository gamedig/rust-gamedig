use crate::GDResult;
use crate::protocols::valve;
use crate::protocols::valve::{game, SteamApp};

pub fn query(address: &str, port: Option<u16>) -> GDResult<game::Response> {
    let valve_response = valve::query(address, match port {
        None => 2304,
        Some(port) => port
    }, SteamApp::ARMA2OA.as_engine(), None, None)?;

    Ok(game::Response::new_from_valve_response(valve_response))
}
