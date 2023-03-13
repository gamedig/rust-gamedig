// This file has code that has been documented by the NodeJS GameDig library
// (MIT) from https://github.com/gamedig/node-gamedig/blob/master/protocols/minecraftbedrock.js

use crate::{
    bufferer::{Bufferer, Endianess},
    protocols::{
        minecraft::{BedrockResponse, GameMode, Server},
        types::TimeoutSettings,
    },
    socket::{Socket, UdpSocket},
    utils::error_by_expected_size,
    GDError::{PacketBad, TypeParse},
    GDResult,
};

pub struct Bedrock {
    socket: UdpSocket,
}

impl Bedrock {
    fn new(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address, port)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self { socket })
    }

    fn send_status_request(&mut self) -> GDResult<()> {
        self.socket.send(&[
            // Message ID, ID_UNCONNECTED_PING
            0x01, // Nonce / timestamp
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, // Magic
            0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34,
            0x56, 0x78, // Client GUID
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ])?;

        Ok(())
    }

    fn get_info(&mut self) -> GDResult<BedrockResponse> {
        self.send_status_request()?;

        let mut buffer = Bufferer::new_with_data(Endianess::Little, &self.socket.receive(None)?);

        if buffer.get_u8()? != 0x1c {
            return Err(PacketBad);
        }

        // Checking for our nonce directly from a u64 (as the nonce is 8 bytes).
        if buffer.get_u64()? != 9833440827789222417 {
            return Err(PacketBad);
        }

        // These 8 bytes are identical to the serverId string we receive in decimal
        // below
        buffer.move_position_ahead(8);

        // Verifying the magic value (as we need 16 bytes, cast to two u64 values)
        if buffer.get_u64()? != 18374403896610127616 {
            return Err(PacketBad);
        }

        if buffer.get_u64()? != 8671175388723805693 {
            return Err(PacketBad);
        }

        let remaining_length = buffer.as_endianess(Endianess::Big).get_u16()? as usize;
        buffer.move_position_ahead(2);
        error_by_expected_size(remaining_length, buffer.remaining_length())?;

        let binding = buffer.get_string_utf8_unended()?;
        let status: Vec<&str> = binding.split(';').collect();

        // We must have at least 6 values
        if status.len() < 6 {
            return Err(PacketBad);
        }

        Ok(BedrockResponse {
            edition: status[0].to_string(),
            name: status[1].to_string(),
            version_name: status[3].to_string(),
            version_protocol: status[2].to_string(),
            players_maximum: status[5].parse().map_err(|_| TypeParse)?,
            players_online: status[4].parse().map_err(|_| TypeParse)?,
            id: status.get(6).map(|v| v.to_string()),
            map: status.get(7).map(|v| v.to_string()),
            game_mode: match status.get(8) {
                None => None,
                Some(v) => Some(GameMode::from_bedrock(v)?),
            },
            server_type: Server::Bedrock,
        })
    }

    pub fn query(
        address: &str,
        port: u16,
        timeout_settings: Option<TimeoutSettings>,
    ) -> GDResult<BedrockResponse> {
        Bedrock::new(address, port, timeout_settings)?.get_info()
    }
}
