use crate::GDResult;
use crate::protocols::minecraft;
use crate::protocols::minecraft::{Server, Response};

pub fn query(mc_type: Server, address: &str, port: Option<u16>) -> GDResult<Response> {
    minecraft::query(mc_type, address, match port {
        None => 25565,
        Some(port) => port
    }, None)
}
