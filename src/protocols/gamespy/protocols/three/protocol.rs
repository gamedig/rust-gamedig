use crate::bufferer::{Bufferer, Endianess};
use crate::protocols::gamespy::common::has_password;
use crate::protocols::gamespy::three::{Player, Response};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::{GDError, GDResult};
use std::collections::HashMap;

const THIS_SESSION_ID: u32 = 1;

struct RequestPacket {
    header: u16,
    kind: u8,
    session_id: u32,
    challenge: Option<i32>,
    payload: Option<[u8; 4]>,
}

impl RequestPacket {
    fn to_bytes(self) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::with_capacity(7);
        packet.extend_from_slice(&self.header.to_be_bytes());
        packet.push(self.kind);
        packet.extend_from_slice(&self.session_id.to_be_bytes());

        if let Some(challenge) = self.challenge {
            packet.extend_from_slice(&challenge.to_be_bytes());
        }

        if let Some(payload) = self.payload {
            packet.extend_from_slice(&payload);
        }

        packet
    }
}

struct GameSpy3 {
    socket: UdpSocket,
}

const PACKET_SIZE: usize = 2048;

impl GameSpy3 {
    fn new(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address, port)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self { socket })
    }

    fn receive(&mut self, size: Option<usize>, kind: u8) -> GDResult<Bufferer> {
        let received = self.socket.receive(size.or(Some(PACKET_SIZE)))?;
        let mut buf = Bufferer::new_with_data(Endianess::Big, &received);

        if buf.get_u8()? != kind {
            return Err(GDError::PacketBad);
        }

        if buf.get_u32()? != THIS_SESSION_ID {
            return Err(GDError::PacketBad);
        }

        Ok(buf)
    }

    fn make_initial_handshake(&mut self) -> GDResult<Option<i32>> {
        self.socket.send(
            &RequestPacket {
                header: 65277,
                kind: 9,
                session_id: THIS_SESSION_ID,
                challenge: None,
                payload: None,
            }
            .to_bytes(),
        )?;

        let mut buf = self.receive(Some(16), 9)?;

        let challenge_as_string = buf.get_string_utf8()?;
        let challenge = challenge_as_string
            .parse()
            .map_err(|_| GDError::TypeParse)?;

        Ok(match challenge == 0 {
            true => None,
            false => Some(challenge),
        })
    }

    fn send_data_request(&mut self, challenge: Option<i32>) -> GDResult<()> {
        self.socket.send(
            &RequestPacket {
                header: 65277,
                kind: 0,
                session_id: THIS_SESSION_ID,
                challenge,
                payload: Some([0xff, 0xff, 0xff, 0x01]),
            }
            .to_bytes(),
        )
    }
}

fn get_server_packets(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Vec<Vec<u8>>> {
    let mut gs3 = GameSpy3::new(address, port, timeout_settings)?;

    let challenge = gs3.make_initial_handshake()?;
    gs3.send_data_request(challenge)?;

    let mut values: Vec<Vec<u8>> = Vec::new();

    let mut expected_number_of_packets: Option<usize> = None;

    while expected_number_of_packets.is_none() || values.len() != expected_number_of_packets.unwrap() {
        let mut buf = gs3.receive(None, 0)?;

        if buf.get_string_utf8()? != "splitnum" {
            return Err(GDError::PacketBad);
        }

        let id = buf.get_u8()?;
        let is_last = (id & 0x80) > 0;
        let packet_id = (id & 0x7f) as usize;
        buf.move_position_ahead(1); //unknown byte regarding packet no.

        if is_last {
            expected_number_of_packets = Some(packet_id + 1);
        }

        while values.len() <= packet_id {
            values.push(Vec::new());
        }

        values[packet_id] = buf.remaining_data_vec();
    }

    if values.iter().any(|v| v.is_empty()) {
        return Err(GDError::PacketBad);
    }

    Ok(values)
}

fn data_to_map(packet: &Vec<u8>) -> GDResult<HashMap<String, String>> {
    let mut vars = HashMap::new();

    let mut buf = Bufferer::new_with_data(Endianess::Big, &packet);
    while buf.remaining_length() > 0 {
        let key = buf.get_string_utf8()?;
        if key.is_empty() {
            continue;
        }

        let value = buf.get_string_utf8_optional()?;

        vars.insert(key, value);
    }

    Ok(vars)
}

/// If there are parsing problems using the `query` function, you can directly
/// get the server's values using this function.
pub fn query_vars(
    address: &str,
    port: u16,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<HashMap<String, String>> {
    let packets = get_server_packets(address, port, timeout_settings)?;

    let mut vars = HashMap::new();

    for packet in &packets {
        vars.extend(data_to_map(packet)?);
    }

    Ok(vars)
}

fn parse_parse_players(packets: Vec<Vec<u8>>) -> GDResult<Vec<Player>> {
    let mut players_data: Vec<HashMap<String, String>> = vec![HashMap::new()];

    for packet in packets {
        let mut buf = Bufferer::new_with_data(Endianess::Little, &packet);

        while buf.remaining_length() > 0 {
            if buf.get_u8()? < 3 {
                continue;
            }

            buf.move_position_backward(1);

            let field = buf.get_string_utf8()?;
            if field.is_empty() {
                continue;
            }

            let field_split: Vec<&str> = field.split('_').collect();
            let field_name = field_split.get(0).ok_or(GDError::PacketBad)?;
            if !["player", "score", "ping", "team", "deaths", "pid", "skill"].contains(field_name) {
                continue;
            }

            let field_type = match field_split.get(1) {
                None => None,
                Some(v) => {
                    match v.is_empty() {
                        true => None,
                        false => Some(v),
                    }
                }
            };

            let mut offset = buf.get_u8()?;

            println!(
                "Parsing new field, type: {:?}, name: {:?}, start offset: {:?}",
                field_type, field_name, offset
            );

            if field_type.is_some() {
                // skip parsing team data
                continue;
            }

            while buf.remaining_length() > 0 {
                let item = buf.get_string_utf8()?;
                if item.is_empty() {
                    break;
                }

                while players_data.len() <= offset as usize {
                    players_data.push(HashMap::new())
                }

                let player_data = players_data.get_mut(offset as usize).unwrap();
                // println!("{:?} {:?} {:?}", field_name, offset, item);
                player_data.insert(field_name.to_string(), item);

                offset += 1;
            }
        }
    }

    let mut players: Vec<Player> = Vec::new();
    for player_data in players_data {
        players.push(Player {
            name: player_data
                .get("player")
                .ok_or(GDError::PacketBad)?
                .to_string(),
            score: player_data
                .get("score")
                .ok_or(GDError::PacketBad)?
                .parse()
                .map_err(|_| GDError::PacketBad)?,
            ping: player_data
                .get("ping")
                .ok_or(GDError::PacketBad)?
                .parse()
                .map_err(|_| GDError::PacketBad)?,
            team: player_data
                .get("team")
                .ok_or(GDError::PacketBad)?
                .parse()
                .map_err(|_| GDError::PacketBad)?,
            deaths: player_data
                .get("deaths")
                .ok_or(GDError::PacketBad)?
                .parse()
                .map_err(|_| GDError::PacketBad)?,
            skill: player_data
                .get("skill")
                .ok_or(GDError::PacketBad)?
                .parse()
                .map_err(|_| GDError::PacketBad)?,
        })
    }

    Ok(players)
}

/// Query a server by providing the address, the port and timeout settings.
/// Providing None to the timeout settings results in using the default values.
/// (TimeoutSettings::[default](TimeoutSettings::default)).
pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let packets = get_server_packets(address, port, timeout_settings)?;

    let mut server_vars = data_to_map(packets.get(0).ok_or(GDError::PacketBad)?)?;

    let players_maximum = server_vars
        .remove("maxplayers")
        .ok_or(GDError::PacketBad)?
        .parse()
        .map_err(|_| GDError::TypeParse)?;
    let players_minimum = match server_vars.remove("minplayers") {
        None => None,
        Some(v) => Some(v.parse::<u8>().map_err(|_| GDError::TypeParse)?),
    };

    Ok(Response {
        name: server_vars.remove("hostname").ok_or(GDError::PacketBad)?,
        map: server_vars.remove("mapname").ok_or(GDError::PacketBad)?,
        map_title: server_vars.remove("maptitle"),
        admin_contact: server_vars.remove("AdminEMail"),
        admin_name: server_vars
            .remove("AdminName")
            .or_else(|| server_vars.remove("admin")),
        has_password: has_password(&mut server_vars)?,
        game_type: server_vars.remove("gametype").ok_or(GDError::PacketBad)?,
        game_version: server_vars.remove("gamever").ok_or(GDError::PacketBad)?,
        players_maximum,
        players_online: server_vars
            .remove("numplayers")
            .unwrap_or("0".to_string())
            .parse()
            .map_err(|_| GDError::TypeParse)?,
        players_minimum,
        players: parse_parse_players(packets)?,
        tournament: server_vars
            .remove("tournament")
            .unwrap_or("true".to_string())
            .to_lowercase()
            .parse()
            .map_err(|_| GDError::TypeParse)?,
        unused_entries: server_vars,
    })
}
