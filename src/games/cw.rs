use crate::protocols::gamespy;
use crate::protocols::gamespy::three::Response;
use crate::GDResult;

pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    gamespy::three::query(address, port.unwrap_or(64100), None)
}
