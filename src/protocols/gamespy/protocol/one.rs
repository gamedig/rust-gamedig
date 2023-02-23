use crate::bufferer::{Bufferer, Endianess};
use crate::GDResult;
use crate::protocols::gamespy::Response;
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};

pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let mut socket = UdpSocket::new(address, port)?;
    socket.apply_timeout(timeout_settings)?;

    socket.send("\\status\\xserverquery".as_bytes())?;

    let data = socket.receive(None)?;
    let mut bufferer = Bufferer::new_with_data(Endianess::Little, &data);

    let mut as_string = bufferer.get_string_utf8_unended()?;
    as_string.remove(0);
    println!("{:02X?}", as_string);
    let splited: Vec<&str> = as_string.split('\\').collect();
    println!("{:02X?}", splited);

    Ok(Response {

    })
}
