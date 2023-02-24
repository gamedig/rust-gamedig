use std::collections::HashMap;
use crate::bufferer::{Bufferer, Endianess};
use crate::{GDError, GDResult};
use crate::protocols::gamespy::Response;
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};

pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let mut socket = UdpSocket::new(address, port)?;
    socket.apply_timeout(timeout_settings)?;

    socket.send("\\status\\xserverquery".as_bytes())?;

    let mut receivedQueryId: Option<usize> = None;
    let mut parts: Vec<usize> = Vec::new();
    let mut maxPartNum: Option<usize> = None;

    let mut serverValues = HashMap::new();

    while maxPartNum.is_none() {
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

            serverValues.insert(key, value);
        }

        let isFinal = serverValues.contains_key("final");
        let queryData = serverValues.get("queryid");
        let mut part = None;
        let mut queryId = None;
        if let Some(qid) = queryData {
            let split: Vec<&str> = qid.split('.').collect();
            if split.len() > 1 {
                part = Some(split[1].parse::<usize>().unwrap());
            }
            queryId = Some(split[0].parse::<usize>().unwrap());
        }

        serverValues.remove("final");
        serverValues.remove("queryid");

        println!("{:?} {:?} {:?}", part, queryId, isFinal);

        if receivedQueryId.is_some() && receivedQueryId != queryId {
            println!("Rejected packet, wrong query ID");
            return Err(GDError::PacketBad);
        }
        else {
            receivedQueryId = queryId;
        }

        if part.is_none() {
            part = Some(parts.len());
            println!("No part number received, assigned: {:?}", part.unwrap());
        }
        else {
            let part_n = part.unwrap();
            if parts.contains(&part_n) {
                println!("Rejected packet (duplicate)");
                return Err(GDError::PacketBad);
            }
            else {
                parts.push(part_n);
            }
        }

        if isFinal {
            maxPartNum = part;
        }
    }

    println!("{:#?}", serverValues);

    Ok(Response {

    })
}
