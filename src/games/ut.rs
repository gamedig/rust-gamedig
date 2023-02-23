use crate::GDResult;
use crate::protocols::gamespy;
use crate::protocols::gamespy::Response;

pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    gamespy::one::query(address, match port {
        None => 7778,
        Some(port) => port
    }, None)
}
