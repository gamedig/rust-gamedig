
use crate::{GDError, GDResult};
use crate::protocols::minecraft::{LegacyGroup, Response, Server};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, TcpSocket};
use crate::utils::buffer::{get_string_utf16_be, get_u16_be, get_u8};
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

        let buf = self.socket.receive(None)?;
        let mut pos = 0;

        if get_u8(&buf, &mut pos)? != 0xFF {
            return Err(GDError::ProtocolRule("Expected 0xFF at the begin of the packet."));
        }

        let length = get_u16_be(&buf, &mut pos)? * 2;
        error_by_expected_size((length + 3) as usize, buf.len())?;

        let packet_string = get_string_utf16_be(&buf, &mut pos)?;

        let split: Vec<&str> = packet_string.split("§").collect();
        error_by_expected_size(3, split.len())?;

        let description = split[0].to_string();
        let online_players = split[1].parse()
            .map_err(|_| GDError::PacketBad("Failed to parse to expected int."))?;
        let max_players = split[2].parse()
            .map_err(|_| GDError::PacketBad("Failed to parse to expected int."))?;

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
