use byteorder::BigEndian;

use crate::{
    buffer::{Buffer, Utf16Decoder},
    protocols::{
        minecraft::{JavaResponse, LegacyGroup, Server},
        types::TimeoutSettings,
    },
    socket::{Socket, TcpSocket},
    utils::error_by_expected_size,
    GDError::{PacketBad, ProtocolFormat},
    GDResult,
};
use std::net::SocketAddr;

pub struct LegacyV1_6 {
    socket: TcpSocket,
}

impl LegacyV1_6 {
    fn new(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = TcpSocket::new(address)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self { socket })
    }

    fn send_initial_request(&mut self) -> GDResult<()> {
        self.socket.send(&[
            0xfe, // Packet ID (FE)
            0x01, // Ping payload (01)
            0xfa, // Packet identifier for plugin message
            0x00, 0x07, // Length of 'GameDig' string (7) as unsigned short
            0x00, 0x47, 0x00, 0x61, 0x00, 0x6D, 0x00, 0x65, 0x00, 0x44, 0x00, 0x69, 0x00,
            0x67, // 'GameDig' string as UTF-16BE
        ])?;

        Ok(())
    }

    pub fn is_protocol(buffer: &mut Buffer<BigEndian>) -> GDResult<bool> {
        let state = buffer
            .remaining_bytes()
            .starts_with(&[0x00, 0xA7, 0x00, 0x31, 0x00, 0x00]);

        if state {
            buffer.move_cursor(6);
        }

        Ok(state)
    }

    pub fn get_response(buffer: &mut Buffer<BigEndian>) -> GDResult<JavaResponse> {
        let packet_string = buffer.read_string::<Utf16Decoder<BigEndian>>(None)?;

        let split: Vec<&str> = packet_string.split('\x00').collect();
        error_by_expected_size(5, split.len())?;

        let version_protocol = split[0].parse().map_err(|_| PacketBad)?;
        let version_name = split[1].to_string();
        let description = split[2].to_string();
        let online_players = split[3].parse().map_err(|_| PacketBad)?;
        let max_players = split[4].parse().map_err(|_| PacketBad)?;

        Ok(JavaResponse {
            version_name,
            version_protocol,
            players_maximum: max_players,
            players_online: online_players,
            players_sample: None,
            description,
            favicon: None,
            previews_chat: None,
            enforces_secure_chat: None,
            server_type: Server::Legacy(LegacyGroup::V1_6),
        })
    }

    fn get_info(&mut self) -> GDResult<JavaResponse> {
        self.send_initial_request()?;

        let data = self.socket.receive(None)?;
        let mut buffer = Buffer::<BigEndian>::new(&data);

        if buffer.read::<u8>()? != 0xFF {
            return Err(ProtocolFormat);
        }

        let length = buffer.read::<u16>()? * 2;
        error_by_expected_size((length + 3) as usize, data.len())?;

        if !LegacyV1_6::is_protocol(&mut buffer)? {
            return Err(ProtocolFormat);
        }

        LegacyV1_6::get_response(&mut buffer)
    }

    pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<JavaResponse> {
        LegacyV1_6::new(address, timeout_settings)?.get_info()
    }
}
