use crate::eco::{EcoRequestSettings, Response, Root};
use crate::http::HttpClient;
use crate::{GDResult, TimeoutSettings};
use std::net::{IpAddr, SocketAddr};

/// Query a eco server.
#[inline]
pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, &None) }

/// Query a eco server.
#[inline]
pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: &Option<TimeoutSettings>,
) -> GDResult<Response> {
    query_with_timeout_and_extra_settings(address, port, timeout_settings, None)
}

/// Query a eco server.
pub fn query_with_timeout_and_extra_settings(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: &Option<TimeoutSettings>,
    extra_settings: Option<EcoRequestSettings>,
) -> GDResult<Response> {
    let address = &SocketAddr::new(*address, port.unwrap_or(3001));
    let mut client = HttpClient::new(
        address,
        timeout_settings,
        extra_settings.unwrap_or_default().into(),
    )?;

    let response = client.get_json::<Root>("/frontpage")?;

    Ok(response.into())
}
