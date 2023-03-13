use crate::{
    protocols::gamespy::{self, Response},
    GDResult,
};

pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    gamespy::one::query(address, port.unwrap_or(25601), None)
}
