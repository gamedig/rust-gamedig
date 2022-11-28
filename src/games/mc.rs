use crate::{GDError, GDResult};
use crate::protocols::minecraft;
use crate::protocols::minecraft::{Server, Response, LegacyGroup};

/// Query with all the protocol variants one by one (Java -> Legacy (1.6 -> 1.4 -> Beta 1.8)).
pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    minecraft::query(address, port_or_default(port), None)
}

/// Query a Java Server.
pub fn query_java(address: &str, port: Option<u16>) -> GDResult<Response> {
    minecraft::query_specific(Server::Java, address, port_or_default(port), None)
}

/// Query a (Java) Legacy Server (1.6 -> 1.4 -> Beta 1.8).
pub fn query_legacy(address: &str, port: Option<u16>) -> GDResult<Response> {
    let unwrapped_port = port_or_default(port);

    if let Ok(response) = minecraft::query_specific(Server::Legacy(LegacyGroup::V1_6), address, unwrapped_port, None) {
        return Ok(response);
    }

    if let Ok(response) = minecraft::query_specific(Server::Legacy(LegacyGroup::V1_4), address, unwrapped_port, None) {
        return Ok(response);
    }

    if let Ok(response) = minecraft::query_specific(Server::Legacy(LegacyGroup::VB1_8), address, unwrapped_port, None) {
        return Ok(response);
    }

    Err(GDError::AutoQuery)
}

fn port_or_default(port: Option<u16>) -> u16 {
    match port {
        None => 25565,
        Some(port) => port
    }
}
