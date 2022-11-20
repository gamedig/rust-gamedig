use serde_json::Value;
use crate::{GDError, GDResult};
use crate::protocols::minecraft::{as_string, as_varint, get_string, get_varint, Player, Response};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, TcpSocket};

pub struct Java {
    socket: TcpSocket
}

impl Java {
    fn new(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = TcpSocket::new(address, port)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self {
            socket
        })
    }

    fn send(&mut self, data: Vec<u8>) -> GDResult<()> {
        self.socket.send(&[as_varint(data.len() as i32), data].concat())
    }

    fn receive(&mut self) -> GDResult<Vec<u8>> {
        let buf = self.socket.receive(None)?;
        let mut pos = 0;

        let _packet_length = get_varint(&buf, &mut pos)? as usize;
        //this declared 'packet length' from within the packet might be wrong (?), not checking with it...

        Ok(buf[pos..].to_vec())
    }

    fn send_handshake(&mut self) -> GDResult<()> {
        let mut buf = Vec::new();
        buf.extend(as_varint(0));                       //packet ID
        //packet:
        buf.extend(as_varint(-1));                       //protocol version (-1 to determine version)
        buf.extend(as_string("gamedig-rs".to_string()));  //server address (can be anything)
        buf.extend((0 as u16).to_be_bytes());                  //server port (can be anything)
        buf.extend(as_varint(1));                        //next state (1 for status)

        self.send(buf)?;

        Ok(())
    }

    fn send_status_request(&mut self) -> GDResult<()> {
        let packet_id_status = as_varint(0);

        self.send(packet_id_status)?;

        Ok(())
    }

    fn send_ping_request(&mut self) -> GDResult<()> {
        let packet_id_ping = as_varint(1);

        self.send(packet_id_ping)?;

        Ok(())
    }

    fn get_info(&mut self) -> GDResult<Response> {
        self.send_handshake()?;
        self.send_status_request()?;
        self.send_ping_request()?;

        let buf = self.receive()?;
        let mut pos = 0;

        let packet_id = get_varint(&buf, &mut pos)?;
        if packet_id != 0 {
            return Err(GDError::PacketBad("Bad receive packet id.".to_string()));
        }

        let json_response = get_string(&buf, &mut pos)?;
        let value_response: Value = serde_json::from_str(&json_response)
            .map_err(|e| GDError::JsonParse(e.to_string()))?;

        let version_name = value_response["version"]["name"].as_str()
            .ok_or(GDError::PacketBad("Couldn't get expected string.".to_string()))?.to_string();
        let version_protocol = value_response["version"]["protocol"].as_i64()
            .ok_or(GDError::PacketBad("Couldn't get expected number.".to_string()))? as i32;

        let max_players = value_response["players"]["max"].as_u64()
            .ok_or(GDError::PacketBad("Couldn't get expected number.".to_string()))? as u32;
        let online_players = value_response["players"]["online"].as_u64()
            .ok_or(GDError::PacketBad("Couldn't get expected number.".to_string()))? as u32;
        let sample_players: Vec<Player> = match value_response["players"]["sample"].is_null() {
            true => Vec::new(),
            false => value_response["players"]["sample"].as_array()
                .ok_or(GDError::PacketBad("Couldn't get expected array.".to_string()))?
                .iter().map(|v| Player {
                name: v["name"].as_str().unwrap().to_string(),
                id: v["id"].as_str().unwrap().to_string()
            }).collect()
        };

        Ok(Response {
            version_name,
            version_protocol,
            max_players,
            online_players,
            sample_players,
            description: value_response["description"].to_string(),
            favicon: value_response["favicon"].as_str().map(str::to_string),
            previews_chat: value_response["previewsChat"].as_bool(),
            enforces_secure_chat: value_response["enforcesSecureChat"].as_bool()
        })
    }

    pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
        Java::new(address, port, timeout_settings)?.get_info()
    }
}
