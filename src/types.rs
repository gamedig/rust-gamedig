use crate::GDResult;
use std::net::IpAddr;

pub type QueryFunction = fn(&IpAddr, Option<u16>) -> GDResult<Box<dyn GenericResponse>>;

pub trait GenericResponse {
    fn server_name(&self) -> String;
    fn server_map(&self) -> String;
}

pub trait GameInfo {
    fn name(&self) -> &'static str;
    fn protocol(&self) -> &'static str;
    fn query(&self, address: &IpAddr, port: Option<u16>) -> GDResult<Box<dyn GenericResponse>>;
}

/*
#[derive(Clone)]
pub struct GameInfo {
    pub name: &'static str,
    pub protocol: &'static str,
    pub query: QueryFunction,
}

impl std::fmt::Debug for GameInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GameInfo{{ name={:?}, protocol={:?} }}",
            self.name, self.protocol,
        )
    }
}
*/
