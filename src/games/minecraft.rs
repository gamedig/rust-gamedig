use crate::GDResult;
use crate::protocols::minecraft;
use crate::protocols::minecraft::Response;

pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    minecraft::query(address, match port {
        None => 25565,
        Some(port) => port
    }, None)
}
