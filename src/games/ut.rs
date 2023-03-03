use crate::GDResult;
use crate::protocols::gamespy;
use crate::protocols::gamespy::Response;

pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    gamespy::one::query(address, port.unwrap_or(7778), None)
}
