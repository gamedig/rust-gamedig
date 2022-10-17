use crate::errors::GDError;
use crate::valve::{Response, ValveProtocol, App};

pub struct TF2;

impl TF2 {
    pub fn query(address: &str, port: Option<u16>) -> Result<Response, GDError> {
        ValveProtocol::query(App::TF2, address, match port {
            None => 27015,
            Some(port) => port
        })
    }
}
