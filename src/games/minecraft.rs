use crate::GDResult;
use crate::protocols::minecraft;
use crate::protocols::minecraft::{Server, Response};

pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    minecraft::query(address, port_or_default(port), None)
}

pub fn query_specific(mc_type: Server, address: &str, port: Option<u16>) -> GDResult<Response> {
    minecraft::query_specific(mc_type, address, port_or_default(port), None)
}

fn port_or_default(port: Option<u16>) -> u16 {
    match port {
        None => 25565,
        Some(port) => port
    }
}
