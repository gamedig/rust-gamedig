use crate::fivem::{FiveMRequestSettings, Info, Player, Response};
use crate::http::HttpClient;
use crate::{GDResult, TimeoutSettings};
use std::net::{IpAddr, SocketAddr};

/// Query a FiveM server.
#[inline]
pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, &None) }

/// Query a FiveM server.
#[inline]
pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: &Option<TimeoutSettings>,
) -> GDResult<Response> {
    query_with_timeout_and_extra_settings(address, port, timeout_settings, None)
}

/// Query a FiveM server.
pub fn query_with_timeout_and_extra_settings(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: &Option<TimeoutSettings>,
    extra_settings: Option<FiveMRequestSettings>,
) -> GDResult<Response> {
    let address = &SocketAddr::new(*address, port.unwrap_or(30120));
    let mut client = HttpClient::new(
        address,
        timeout_settings,
        extra_settings.unwrap_or_default().into(),
    )?;

    let info_response = client.get_json::<Info>("/info.json", None)?;
    let player_response = client.get_json::<Vec<Player>>("/players.json", None)?;

    let response: Response = (info_response, player_response).into();

    Ok(response)
}
