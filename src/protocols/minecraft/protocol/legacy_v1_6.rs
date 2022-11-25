use crate::{GDError, GDResult};
use crate::protocols::minecraft::{LegacyGroup, Response, Server};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, TcpSocket};
use crate::utils::buffer::{get_string_utf16_be, get_u16_be, get_u8};
use crate::utils::error_by_expected_size;

pub struct LegacyV1_6 {
    socket: TcpSocket
}

impl LegacyV1_6 {
    fn new(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = TcpSocket::new(address, port)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self {
            socket
        })
    }

    fn send_initial_request(&mut self) -> GDResult<()> {
        self.socket.send(&[
            // Packet ID (FE)
            0xfe,
            // Ping payload (01)
            0x01,
            // Packet identifier for plugin message
            0xfa,
            // Length of 'GameDig' string (7) as unsigned short
            0x00, 0x07,
            // 'GameDig' string as UTF-16BE
            0x00, 0x47, 0x00, 0x61, 0x00, 0x6D, 0x00, 0x65, 0x00, 0x44, 0x00, 0x69, 0x00, 0x67])?;

        Ok(())
    }

    pub fn is_protocol(buf: &[u8], pos: &mut usize) -> GDResult<bool> {
        let state = buf[*pos..].starts_with(&[0x00, 0xA7, 0x00, 0x31, 0x00, 0x00]);

        if state {
            *pos += 6;
        }

        Ok(state)
    }

    pub fn get_response(buf: &[u8], pos: &mut usize) -> GDResult<Response> {
        let packet_string = get_string_utf16_be(&buf, pos)?;

        let split: Vec<&str> = packet_string.split("\x00").collect();
        error_by_expected_size(5, split.len())?;

        let version_protocol = split[0].parse()
            .map_err(|_| GDError::PacketBad("Failed to parse to expected int.".to_string()))?;
        let version_name = split[1].to_string();
        let description = split[2].to_string();
        let max_players = split[3].parse()
            .map_err(|_| GDError::PacketBad("Failed to parse to expected int.".to_string()))?;
        let online_players = split[4].parse()
            .map_err(|_| GDError::PacketBad("Failed to parse to expected int.".to_string()))?;

        Ok(Response {
            version_name,
            version_protocol,
            max_players,
            online_players,
            sample_players: None,
            description,
            favicon: None,
            previews_chat: None,
            enforces_secure_chat: None,
            server_type: Server::Legacy(LegacyGroup::V1_6)
        })
    }

    fn get_info(&mut self) -> GDResult<Response> {
        self.send_initial_request()?;

        let buf = self.socket.receive(None)?;
        let mut pos = 0;

        if get_u8(&buf, &mut pos)? != 0xFF {
            return Err(GDError::ProtocolFormat("Expected a certain byte (0xFF) at the begin of the packet.".to_string()));
        }

        let length = get_u16_be(&buf, &mut pos)? * 2;
        error_by_expected_size((length + 3) as usize, buf.len())?;

        if !LegacyV1_6::is_protocol(&buf, &mut pos)? {
            return Err(GDError::ProtocolFormat("Expected certain bytes at the beginning of the packet.".to_string()));
        }

        LegacyV1_6::get_response(&buf, &mut pos)
    }

    pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
        LegacyV1_6::new(address, port, timeout_settings)?.get_info()
    }
}
