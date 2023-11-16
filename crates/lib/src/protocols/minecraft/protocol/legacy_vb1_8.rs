use crate::{
    buffer::{Buffer, Utf16Decoder},
    protocols::{
        minecraft::{JavaResponse, LegacyGroup, Server},
        types::TimeoutSettings,
    },
    socket::{Socket, TcpSocket},
    utils::{error_by_expected_size, retry_on_timeout},
    GDErrorKind::{PacketBad, ProtocolFormat},
    GDResult,
};

use std::net::SocketAddr;

use byteorder::BigEndian;

pub struct LegacyVB1_8 {
    socket: TcpSocket,
    retry_count: usize,
}

impl LegacyVB1_8 {
    fn new(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = TcpSocket::new(address)?;
        socket.apply_timeout(&timeout_settings)?;

        let retry_count = TimeoutSettings::get_retries_or_default(&timeout_settings);
        Ok(Self {
            socket,
            retry_count,
        })
    }

    fn send_initial_request(&mut self) -> GDResult<()> { self.socket.send(&[0xFE]) }

    /// Send request for info and parse response.
    /// This function will retry fetch on timeouts.
    fn get_info(&mut self) -> GDResult<JavaResponse> {
        retry_on_timeout(self.retry_count, move || self.get_info_impl())
    }

    /// Send request for info and parse response (without retry logic).
    fn get_info_impl(&mut self) -> GDResult<JavaResponse> {
        self.send_initial_request()?;

        let data = self.socket.receive(None)?;
        let mut buffer = Buffer::<BigEndian>::new(&data);

        if buffer.read::<u8>()? != 0xFF {
            return Err(ProtocolFormat.context("Expected 0xFF"));
        }

        let length = buffer.read::<u16>()? * 2;
        error_by_expected_size((length + 3) as usize, data.len())?;

        let packet_string = buffer.read_string::<Utf16Decoder<BigEndian>>(None)?;

        let split: Vec<&str> = packet_string.split('§').collect();
        error_by_expected_size(3, split.len())?;

        let description = split[0].to_string();
        let online_players = split[1].parse().map_err(|e| PacketBad.context(e))?;
        let max_players = split[2].parse().map_err(|e| PacketBad.context(e))?;

        Ok(JavaResponse {
            game_version: "Beta 1.8+".to_string(),
            protocol_version: -1,
            players_maximum: max_players,
            players_online: online_players,
            players: None,
            description,
            favicon: None,
            previews_chat: None,
            enforces_secure_chat: None,
            server_type: Server::Legacy(LegacyGroup::VB1_8),
        })
    }

    pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<JavaResponse> {
        Self::new(address, timeout_settings)?.get_info()
    }
}
