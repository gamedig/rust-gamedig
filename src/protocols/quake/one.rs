use std::collections::HashMap;
use std::net::Ipv4Addr;
use crate::bufferer::{Bufferer, Endianess};
use crate::{GDError, GDResult};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};

#[derive(Debug)]
pub struct Response {
    pub name: String
}

#[derive(Debug)]
pub struct Player {
    pub frags: u8,
    pub ping: u8,
    pub name: String
}

fn get_server_values(
    address: &Ipv4Addr,
    port: u16,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<HashMap<String, String>> {
    let mut socket = UdpSocket::new(address, port)?;
    socket.apply_timeout(timeout_settings)?;

    socket.send(&[0xFF, 0xFF, 0xFF, 0xFF, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x00])?;
    //                                         ^ header                         ^

    let data = socket.receive(None)?;
    let mut bufferer = Bufferer::new_with_data(Endianess::Little, &data);

    if bufferer.get_u32()? != 4294967295 {
        return Err(GDError::PacketBad);
    }

    bufferer.get_string_utf8_newline()?; //print

    let data = bufferer.get_string_utf8_newline()?;
    let mut data_split = data.split("\\").collect::<Vec<&str>>();
    if let Some(first) = data_split.first() {
        if first == &"" {
            data_split.remove(0);
        }
    }

    let values = data_split.chunks(2);

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

    let mut players = Vec::new();
    let mut bots = Vec::new();

    while !bufferer.is_remaining_empty() {
        let data = bufferer.get_string_utf8_newline()?;
        let data_split = data.split(" ").collect::<Vec<&str>>();
        let mut data_iter = data_split.iter();

        let player = Player {
            frags: match data_iter.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
            },
            ping: match data_iter.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.parse().map_err(|_| GDError::PacketBad)?
            },
            name: match data_iter.next() {
                None => Err(GDError::PacketBad)?,
                Some(v) => v.to_string()
            },
        };

        match player.ping == 0 {
            false => &mut players,
            true => &mut bots
        }.push(player);
    }

    println!("{:?}", players);
    println!("{:?}", bots);

    Ok(vars)
}

pub fn query(address: &Ipv4Addr, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let server_vars = get_server_values(address, port, timeout_settings)?;

    //println!("{:#?}", server_vars);
    Ok(Response {
        name: "test".to_string()
    })
}
