use byteorder::BigEndian;

use crate::{
    buffer::{Buffer, Utf16Decoder},
    protocols::{
        minecraft::{JavaResponse, LegacyGroup, Server},
        types::TimeoutSettings,
    },
    socket::{Socket, TcpSocket},
    utils::error_by_expected_size,
    GDErrorKind::{PacketBad, ProtocolFormat},
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

    pub(crate) fn is_protocol(buffer: &mut Buffer<BigEndian>) -> GDResult<bool> {
        let state = buffer
            .remaining_bytes()
            .starts_with(&[0x00, 0xA7, 0x00, 0x31, 0x00, 0x00]);

        if state {
            buffer.move_cursor(6)?;
        }

        Ok(state)
    }

    pub(crate) fn get_response(buffer: &mut Buffer<BigEndian>) -> GDResult<JavaResponse> {
        // This is a specific order!
        let protocol_version = buffer
            .read_string::<Utf16Decoder<BigEndian>>(None)?
            .parse()
            .map_err(|e| PacketBad.context(e))?;
        let game_version = buffer.read_string::<Utf16Decoder<BigEndian>>(None)?;
        let description = buffer.read_string::<Utf16Decoder<BigEndian>>(None)?;
        let online_players = buffer
            .read_string::<Utf16Decoder<BigEndian>>(None)?
            .parse()
            .map_err(|e| PacketBad.context(e))?;
        let max_players = buffer
            .read_string::<Utf16Decoder<BigEndian>>(None)?
            .parse()
            .map_err(|e| PacketBad.context(e))?;

        Ok(JavaResponse {
            game_version,
            protocol_version,
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
            return Err(ProtocolFormat.context("Expected 0xFF"));
        }

        let length = buffer.read::<u16>()? * 2;
        error_by_expected_size((length + 3) as usize, data.len())?;

        if !LegacyV1_6::is_protocol(&mut buffer)? {
            return Err(ProtocolFormat.context("Not legacy 1.6 protocol"));
        }

        LegacyV1_6::get_response(&mut buffer)
    }

    pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<JavaResponse> {
        LegacyV1_6::new(address, timeout_settings)?.get_info()
    }
}
