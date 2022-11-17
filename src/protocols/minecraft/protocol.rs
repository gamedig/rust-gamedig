use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json::Value;
use crate::{GDError, GDResult};
use crate::GDError::JsonParse;
use crate::protocols::minecraft::{as_string, as_varint, get_string, get_varint, Player, Response, Version};
use crate::protocols::types::TimeoutSettings;
use crate::utils::complete_address;

struct MinecraftProtocol {
    socket: TcpStream
}

impl MinecraftProtocol {
    fn new(address: &str, port: u16, timeout_settings: TimeoutSettings) -> GDResult<Self> {
        let complete_address = complete_address(address, port)?;
        let socket = TcpStream::connect(complete_address).map_err(|e| GDError::SocketConnect(e.to_string()))?;

        socket.set_read_timeout(timeout_settings.get_read()).unwrap();   //unwrapping because TimeoutSettings::new
        socket.set_write_timeout(timeout_settings.get_write()).unwrap(); //checks if these are 0 and throws an error

        Ok(Self {
            socket
        })
    }

    fn send(&mut self, data: &[u8]) -> GDResult<()> {
        self.socket.write(&data).map_err(|e| GDError::PacketSend(e.to_string()))?;
        Ok(())
    }

    fn send_packet(&mut self, data: Vec<u8>) -> GDResult<()> {
        self.send(&[as_varint(data.len() as i32), data].concat())
    }

    fn receive(&mut self) -> GDResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.socket.read_to_end(&mut buf).map_err(|e| GDError::PacketReceive(e.to_string()))?;

        Ok(buf)
    }

    fn receive_packet(&mut self) -> GDResult<Vec<u8>> {
        let buf = self.receive()?;
        let mut pos = 0;

        let _packet_length = get_varint(&buf, &mut pos)? as usize;
        //this declared 'packet length' from within the packet might be wrong, not checking with it...

        Ok(buf[pos..].to_owned())
    }

    fn send_handshake(&mut self) -> GDResult<()> {
        let mut buf = Vec::new();
        buf.extend(as_varint(0));                       //packet ID
        //packet:
        buf.extend(as_varint(-1));                       //protocol version (-1 to determine version)
        buf.extend(as_string("gamedig-rs".to_owned()));  //server address (can be anything)
        buf.extend((0 as u16).to_be_bytes());                  //server port (can be anything)
        buf.extend(as_varint(1));                        //next state (1 for status)

        self.send_packet(buf)?;

        Ok(())
    }

    fn send_status_request(&mut self) -> GDResult<()> {
        let packet_id_status = as_varint(0);

        self.send_packet(packet_id_status)?;

        Ok(())
    }

    fn send_ping_request(&mut self) -> GDResult<()> {
        let packet_id_ping = as_varint(1);

        self.send_packet(packet_id_ping)?;

        Ok(())
    }

    fn get_info(&mut self) -> GDResult<Response> {
        self.send_handshake()?;
        self.send_status_request()?;
        self.send_ping_request()?;

        let buf = self.receive_packet()?;
        let mut pos = 0;

        let packet_id = get_varint(&buf, &mut pos)?;
        if packet_id != 0 {
            return Err(GDError::PacketBad("Bad receive packet id.".to_owned()));
        }

        let json_response = get_string(&buf, &mut pos)?;
        let value_response: Value = serde_json::from_str(&json_response)
            .map_err(|_| JsonParse("Received string is unparsable.".to_owned()))?;

        let favicon = match value_response["favicon"].is_null() {
            true => None,
            false => Some(value_response["favicon"].as_str().unwrap().to_owned())
        };

        let sample_players: Vec<Player> = match value_response["players"]["sample"].is_null() {
            true => Vec::new(),
            false => {
                value_response["players"]["sample"].as_array().clone().unwrap()
                    .iter().map(|v| Player {
                    name: v["name"].to_string(),
                    id: v["id"].to_string()
                }).collect()
            }
        };

        Ok(Response {
            version: Version::V1_19,
            max_players: value_response["players"]["max"].as_u64().unwrap() as u32,
            online_players: value_response["players"]["online"].as_u64().unwrap() as u32,
            sample_players,
            description: value_response["description"].to_string(),
            favicon,
            previews_chat: value_response["previewsChat"].as_bool(),
            enforces_secure_chat: value_response["enforcesSecureChat"].as_bool()
        })
    }
}

pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let response_timeout_settings = timeout_settings.unwrap_or(TimeoutSettings::default());
    get_response(address, port, response_timeout_settings)
}

pub fn get_response(address: &str, port: u16, timeout_settings: TimeoutSettings) -> GDResult<Response> {
    let mut client = MinecraftProtocol::new(address, port, timeout_settings)?;

    Ok(client.get_info()?)
}
