use std::collections::HashMap;
use crate::bufferer::{Bufferer, Endianess};
use crate::{GDError, GDResult};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};

#[derive(Debug)]
pub struct Response {
    pub name: String
}

fn get_server_values(
    address: &str,
    port: u16,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<HashMap<String, String>> {
    let mut socket = UdpSocket::new(address, port)?;
    socket.apply_timeout(timeout_settings)?;

    socket.send(&[0xFF, 0xFF, 0xFF, 0xFF, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x00])?;

    let data = socket.receive(None)?;
    let mut bufferer = Bufferer::new_with_data(Endianess::Little, &data);

    if bufferer.get_u32()? != 4294967295 {
        return Err(GDError::PacketBad);
    }

    let data = bufferer.get_string_utf8()?;
    let after_the_first_weird_value = data.split("\\")
        .into_iter()
        .skip(1)
        .collect::<Vec<&str>>();
    let values = after_the_first_weird_value.chunks(2);

    let mut vars: HashMap<String, String> = HashMap::new();
    for data in values {
        let key = data.get(0);
        let value = data.get(1);

        if let Some(k) = key {
            if let Some(v) = value {
                vars.insert(k.to_string(), v.to_string());
            }
        }
    }

    Ok(vars)
}

pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let server_vars = get_server_values(address, port, timeout_settings)?;

    println!("{:#?}", server_vars);
    Ok(Response {
        name: "test".to_string()
    })
}
