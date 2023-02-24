use std::collections::HashMap;
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

    let splited: Vec<&str> = as_string.split('\\').collect();
    let mut data = HashMap::new();

    for i in 0..splited.len() / 2 {
        let position = i * 2;
        let key = splited[position];
        let value = splited.get(position + 1).unwrap_or(&"");

        data.insert(key, value);
    }

    println!("{:#?}", data);
    println!("{}", data.contains_key("final"));
    println!("{}", data["queryid"]);

    Ok(Response {

    })
}
