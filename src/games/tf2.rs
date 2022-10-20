use crate::errors::GDError;
use crate::valve;
use crate::valve::{ValveProtocol, App, GatheringSettings};

#[derive(Debug)]
pub struct Response {

}

impl Response {
    pub fn new_from_valve_response(response: valve::Response) -> Self {

    }
}

pub fn query(address: &str, port: Option<u16>) -> Result<Response, GDError> {
    let valve_response = ValveProtocol::query(App::TF2, address, match port {
        None => 27015,
        Some(port) => port
    }, GatheringSettings {
        players: true,
        rules: true
    })?;

    Ok(Response::new_from_valve_response(valve_response))
}
