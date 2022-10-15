use crate::errors::GDError;
use crate::protocol::Protocol;
use crate::protocols::valve::{Response, ValveProtocol};

pub struct TF2;

impl TF2 {
    pub fn query(address: &str, port: Option<u16>) -> Result<Response, GDError> {
        ValveProtocol::query(address, match port {
            None => 27015,
            Some(port) => port
        })
    }
}
