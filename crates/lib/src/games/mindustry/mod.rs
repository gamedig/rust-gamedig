//! Mindustry game ping (v146)
//!
//! [Reference](https://github.com/Anuken/Mindustry/blob/a2e5fbdedb2fc1c8d3c157bf344d10ad6d321442/core/src/mindustry/net/ArcNetProvider.java#L225-L259)

use std::{net::IpAddr, net::SocketAddr};

use crate::{GDResult, TimeoutSettings};

use self::types::ServerData;

pub mod types;

pub mod protocol;

/// Default mindustry server port
///
/// [Reference](https://github.com/Anuken/Mindustry/blob/a2e5fbdedb2fc1c8d3c157bf344d10ad6d321442/core/src/mindustry/Vars.java#L141-L142)
pub const DEFAULT_PORT: u16 = 6567;

/// Query a mindustry server.
pub fn query(ip: &IpAddr, port: Option<u16>, timeout_settings: &Option<TimeoutSettings>) -> GDResult<ServerData> {
    let address = SocketAddr::new(*ip, port.unwrap_or(DEFAULT_PORT));

    protocol::query_with_retries(&address, timeout_settings)
}
