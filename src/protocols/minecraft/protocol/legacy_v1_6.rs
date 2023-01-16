use crate::GDResult;
use crate::GDError::{PacketBad, ProtocolFormat};
use crate::bufferer::{Bufferer, Endianess};
use crate::protocols::minecraft::{LegacyGroup, Response, Server};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, TcpSocket};
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

    pub fn is_protocol(buffer: &mut Bufferer) -> GDResult<bool> {
        let state = buffer.remaining_data().starts_with(&[0x00, 0xA7, 0x00, 0x31, 0x00, 0x00]);

        if state {
            buffer.move_position_ahead(6);
        }

        Ok(state)
    }

    pub fn get_response(buffer: &mut Bufferer) -> GDResult<Response> {
        let packet_string = buffer.get_string_utf16()?;

        let split: Vec<&str> = packet_string.split("\x00").collect();
        error_by_expected_size(5, split.len())?;

        let version_protocol = split[0].parse()
            .map_err(|_| PacketBad)?;
        let version_name = split[1].to_string();
        let description = split[2].to_string();
        let online_players = split[3].parse()
            .map_err(|_| PacketBad)?;
        let max_players = split[4].parse()
            .map_err(|_| PacketBad)?;

        Ok(Response {
            version_name,
            version_protocol,
            players_maximum: max_players,
            players_online: online_players,
            players_sample: None,
            description,
            favicon: None,
            previews_chat: None,
            enforces_secure_chat: None,
            server_type: Server::Legacy(LegacyGroup::V1_6)
        })
    }

    fn get_info(&mut self) -> GDResult<Response> {
        self.send_initial_request()?;

        let mut buffer = Bufferer::new_with_data(Endianess::Big, &self.socket.receive(None)?);

        if buffer.get_u8()? != 0xFF {
            return Err(ProtocolFormat);
        }

        let length = buffer.get_u16()? * 2;
        error_by_expected_size((length + 3) as usize, buffer.data_length())?;

        if !LegacyV1_6::is_protocol(&mut buffer)? {
            return Err(ProtocolFormat);
        }

        LegacyV1_6::get_response(&mut buffer)
    }

    pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
        LegacyV1_6::new(address, port, timeout_settings)?.get_info()
    }
}
