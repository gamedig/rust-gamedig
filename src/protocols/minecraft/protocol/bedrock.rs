
/*
This file has code that has been documented by the NodeJS GameDig library (MIT) from
https://github.com/gamedig/node-gamedig/blob/master/protocols/minecraftbedrock.js
*/

use crate::{GDError, GDResult};
use crate::protocols::minecraft::{BedrockResponse, GameMode, Server};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::utils::buffer::{get_string_utf8_le_unended, get_u16_be, get_u64_le, get_u8};
use crate::utils::error_by_expected_size;

pub struct Bedrock {
    socket: UdpSocket
}

impl Bedrock {
    fn new(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address, port)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self {
            socket
        })
    }

    fn send_status_request(&mut self) -> GDResult<()> {
        self.socket.send(&[
            // Message ID, ID_UNCONNECTED_PING
            0x01,
            // Nonce / timestamp
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
            // Magic
            0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
            // Client GUID
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])?;

        Ok(())
    }

    fn get_info(&mut self) -> GDResult<BedrockResponse> {
        self.send_status_request()?;

        let buf = self.socket.receive(None)?;
        let mut pos = 0;

        if get_u8(&buf, &mut pos)? != 0x1c {
            return Err(GDError::PacketBad("Invalid message id.".to_string()));
        }

        // Checking for our nonce directly from a u64 (as the nonce is 8 bytes).
        if get_u64_le(&buf, &mut pos)? != 9833440827789222417 {
            return Err(GDError::PacketBad("Invalid nonce.".to_string()));
        }

        // These 8 bytes are identical to the serverId string we receive in decimal below
        pos += 8;

        // Verifying the magic value (as we need 16 bytes, cast to two u64 values)
        if get_u64_le(&buf, &mut pos)? != 18374403896610127616 {
            return Err(GDError::PacketBad("Invalid magic (part 1).".to_string()));
        }

        if get_u64_le(&buf, &mut pos)? != 8671175388723805693 {
            return Err(GDError::PacketBad("Invalid magic (part 2).".to_string()));
        }

        let remaining_length = get_u16_be(&buf, &mut pos)? as usize;
        error_by_expected_size(remaining_length, buf.len() - pos)?;

        let binding = get_string_utf8_le_unended(&buf, &mut pos)?;
        let status: Vec<&str> = binding.split(";").collect();

        // We must have at least 6 values
        if status.len() < 6 {
            return Err(GDError::PacketBad("Not enough status parts.".to_string()));
        }

        Ok(BedrockResponse {
            edition: status[0].to_string(),
            name: status[1].to_string(),
            version_name: status[3].to_string(),
            version_protocol:  status[2].to_string(),
            max_players: status[5].parse().map_err(|_| GDError::TypeParse("couldn't parse.".to_string()))?,
            online_players: status[4].parse().map_err(|_| GDError::TypeParse("couldn't parse.".to_string()))?,
            id: status.get(6).and_then(|v| Some(v.to_string())),
            map: status.get(7).and_then(|v| Some(v.to_string())),
            game_mode: match status.get(8) {
                None => None,
                Some(v) => Some(GameMode::from_bedrock(v)?)
            },
            server_type: Server::Bedrock
        })
    }

    pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<BedrockResponse> {
        Bedrock::new(address, port, timeout_settings)?.get_info()
    }
}
