use crate::GDResult;
use crate::protocols::valve;
use crate::protocols::valve::{game, SteamID};

pub fn query(address: &str, port: Option<u16>) -> GDResult<game::Response> {
    let valve_response = valve::query(address, match port {
        None => 27131,
        Some(port) => port
    }, SteamID::INSS.app(), None)?;

    Ok(game::Response::new_from_valve_response(valve_response))
}
