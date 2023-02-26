use std::collections::HashMap;
use crate::bufferer::{Bufferer, Endianess};
use crate::{GDError, GDResult};
use crate::protocols::gamespy::Response;
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};

fn get_server_values(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<HashMap<String, String>> {
    let mut socket = UdpSocket::new(address, port)?;
    socket.apply_timeout(timeout_settings)?;

    socket.send("\\status\\xserverquery".as_bytes())?;

    let mut received_query_id: Option<usize> = None;
    let mut parts: Vec<usize> = Vec::new();
    let mut is_finished = false;

    let mut server_values = HashMap::new();

    while !is_finished {
        let data = socket.receive(None)?;
        let mut bufferer = Bufferer::new_with_data(Endianess::Little, &data);

        let mut as_string = bufferer.get_string_utf8_unended()?;
        as_string.remove(0);

        let splited: Vec<String> = as_string.split('\\').map(str::to_string).collect();

        for i in 0..splited.len() / 2 {
            let position = i * 2;
            let key = splited[position].clone();
            let value = match splited.get(position + 1) {
                None => "".to_string(),
                Some(v) => v.clone()
            };

            server_values.insert(key, value);
        }

        is_finished = server_values.contains_key("final");
        server_values.remove("final");

        let query_data = server_values.get("queryid");

        let mut part = parts.len(); //by default, if not part number is provided, its the parts length
        let mut query_id = None;
        if let Some(qid) = query_data {
            let split: Vec<&str> = qid.split('.').collect();

            query_id = Some(split[0].parse().unwrap());
            match split.len() {
                1 => (),
                2 => part = split[1].parse().unwrap(),
                _ => Err(GDError::PacketBad)? //the queryid can't be splitted in more than 2 elements
            };
        }

        server_values.remove("queryid");

        println!("{:?} {:?} {:?}", part, query_id, is_finished);

        if received_query_id.is_some() && received_query_id != query_id {
            return Err(GDError::PacketBad); //wrong query id!
        }
        else {
            received_query_id = query_id;
        }

        match parts.contains(&part) {
            true => Err(GDError::PacketBad)?,
            false => parts.push(part)
        }
    }

    Ok(server_values)
}

pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let server_vars = get_server_values(address, port, timeout_settings)?;

    println!("{:#?}", server_vars);

    Ok(Response {

    })
}
