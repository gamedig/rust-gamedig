use crate::{
    protocols::gamespy::{self, types::one::Response},
    GDResult,
};

pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    gamespy::one::query(address, port.unwrap_or(7778), None)
}
