use crate::errors::GDError;

pub trait Protocol {
    type Response;

    fn query(address: &str, port: u16) -> Result<Self::Response, GDError>;
}
