
use crate::{GDError, GDResult};
use crate::bufferer::{Bufferer, Endianess};
use crate::protocols::minecraft::{LegacyGroup, Response, Server};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, TcpSocket};
use crate::utils::error_by_expected_size;

pub struct LegacyBV1_8 {
    socket: TcpSocket
}

impl LegacyBV1_8 {
    fn new(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = TcpSocket::new(address, port)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self {
            socket
        })
    }

    fn send_initial_request(&mut self) -> GDResult<()> {
        self.socket.send(&[0xFE])
    }

    fn get_info(&mut self) -> GDResult<Response> {
        self.send_initial_request()?;

        let mut buffer = Bufferer::new_with_data(Endianess::Big, &self.socket.receive(None)?);

        if buffer.get_u8()? != 0xFF {
            return Err(GDError::ProtocolFormat("Expected 0xFF at the begin of the packet.".to_string()));
        }

        let length = buffer.get_u16()? * 2;
        error_by_expected_size((length + 3) as usize, buffer.data_length())?;

        let packet_string = buffer.get_string_utf16()?;

        let split: Vec<&str> = packet_string.split("§").collect();
        error_by_expected_size(3, split.len())?;

        let description = split[0].to_string();
        let online_players = split[1].parse()
            .map_err(|_| GDError::PacketBad("Failed to parse to expected int.".to_string()))?;
        let max_players = split[2].parse()
            .map_err(|_| GDError::PacketBad("Failed to parse to expected int.".to_string()))?;

        Ok(Response {
            version_name: "Beta 1.8+".to_string(),
            version_protocol: -1,
            max_players,
            online_players,
            sample_players: None,
            description,
            favicon: None,
            previews_chat: None,
            enforces_secure_chat: None,
            server_type: Server::Legacy(LegacyGroup::VB1_8)
        })
    }

    pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
        LegacyBV1_8::new(address, port, timeout_settings)?.get_info()
    }
}
