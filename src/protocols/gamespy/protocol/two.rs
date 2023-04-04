use std::{
    collections::{HashMap, HashSet},
    net::Ipv4Addr,
};

use crate::{
    bufferer::{Bufferer, Endianess},
    protocols::gamespy::types::two::{Response, REQUEST_PACKET_BYTES},
    socket::{Socket, UdpSocket},
    GDResult,
};

pub struct GameSpy2 {
    address: String,
    port: u16,
}

impl GameSpy2 {
    pub fn new(address: Ipv4Addr, port: u16) -> Self {
        Self {
            address: address.to_string(),
            port,
        }
    }

    pub fn request(&self) -> GDResult<RawResponse> {
        let mut socket = UdpSocket::new(&self.address, self.port)?;

        socket.send(&REQUEST_PACKET_BYTES)?;

        let data = socket.receive(None)?;

        Ok(RawResponse { data })
    }
}

pub struct RawResponse {
    pub data: Vec<u8>,
}

impl RawResponse {
    pub fn parse(&self) -> GDResult<Response> {
        let mut bufferer = Bufferer::new_with_data(Endianess::Little, &self.data);

        // Skip the header
        bufferer.move_position_ahead(5);

        // Parse the key/value strings pairs, ending with an empty key and value
        let mut server_data = HashMap::new();
        while let Some(key) = bufferer.get_string_utf8().ok().filter(|s| !s.is_empty()) {
            if let Some(value) = bufferer.get_string_utf8().ok() {
                server_data.insert(key, value);
            }
        }

        // Skip empty key/value
        bufferer.move_position_ahead(2);

        // Parse the player count and score names
        let player_count = bufferer.get_u8()?;
        let mut score_set = HashSet::new();
        while let Some(score_name) = bufferer.get_string_utf8().ok().filter(|s| !s.is_empty()) {
            score_set.insert(score_name);
        }

        // Skip empty byte
        bufferer.move_position_ahead(1);

        // Parse the players
        let mut players = Vec::new();
        for _ in 0 .. player_count {
            let mut player = HashMap::new();
            for score_name in &score_set {
                if let Some(score_value) = bufferer.get_string_utf8().ok() {
                    player.insert(score_name.clone(), score_value);
                }
            }
            players.push(player);
        }

        // Skip empty byte
        bufferer.move_position_ahead(1);

        // ! Team structure is unknown
        let teams = bufferer.get_string_utf8_unended()?;

        // TODO: Parse the data into a Response struct

        todo!()
    }
}
