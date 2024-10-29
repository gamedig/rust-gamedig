use crate::minetest::Response;
use crate::{minetest_master_server, GDErrorKind, GDResult, TimeoutSettings};
use std::net::IpAddr;

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, &None) }

pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: &Option<TimeoutSettings>,
) -> GDResult<Response> {
    let address = address.to_string();
    let port = port.unwrap_or(30000);

    let servers = minetest_master_server::query(timeout_settings.unwrap_or_default())?;
    for server in servers.list {
        if server.ip == address && server.port == port {
            return Ok(server.into());
        }
    }

    Err(GDErrorKind::AutoQuery.context("Server not found in the master query list."))
}
