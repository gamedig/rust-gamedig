use crate::eco::{Response, Root};
use crate::http::HttpClient;
use crate::{GDResult, TimeoutSettings};
use std::net::{IpAddr, SocketAddr};

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> {
    let address = &SocketAddr::new(*address, port.unwrap_or(3001));
    let mut client = HttpClient::new(address, &Some(TimeoutSettings::default()))?;

    let response = client.request::<Root>("/frontpage")?;

    Ok(response.into())
}
