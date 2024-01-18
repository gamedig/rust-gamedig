use crate::games::theship::types::Response;
use crate::protocols::types::TimeoutSettings;
use crate::protocols::valve;
use crate::protocols::valve::Engine;
use crate::GDResult;
use std::net::{IpAddr, SocketAddr};

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, None) }

pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response> {
    let valve_response = valve::query(
        &SocketAddr::new(*address, port.unwrap_or(27015)),
        Engine::new(2400),
        None,
        timeout_settings,
    )?;

    Response::new_from_valve_response(valve_response)
}
