use crate::{GDError, GDResult};
use crate::protocols::minecraft::{Player, Response};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, TcpSocket};
use crate::utils::buffer::{get_string_utf16_be, get_u16_be, get_u8};

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
        //self.socket.send(&[0xFE, 0x01])?;

        Ok(())
    }

    pub fn is_protocol(buf: &[u8], pos: &mut usize) -> GDResult<bool> {
        if get_u8(&buf, pos)? != 0xFF {
            return Err(GDError::PacketBad("Expected 0xFF".to_string()));
        }

        let length = get_u16_be(&buf, pos)? * 2;
        if buf.len() != (length + 3) as usize { //+ 3 because of the first byte and the u16
            return Err(GDError::PacketBad("Not right size".to_string()));
        }

        Ok(buf[*pos..].starts_with(&[0x00, 0xA7, 0x00, 0x31, 0x00, 0x00]))
    }

    pub fn get_response(buf: &[u8], pos: &mut usize) -> GDResult<Response> {

        Ok(Response {
            version_name: "".to_string(),
            version_protocol: 0,
            max_players: 0,
            online_players: 0,
            sample_players: vec![],
            description: "".to_string(),
            favicon: None,
            previews_chat: None,
            enforces_secure_chat: None
        })
    }

    fn get_info(&mut self) -> GDResult<Response> {
        self.send_initial_request()?;

        let buf = self.socket.receive(None)?;
        let mut pos = 0;

        if LegacyV1_6::is_protocol(&buf, &mut pos)? {
            return Err(GDError::PacketBad("Not good".to_string()));
        }

        LegacyV1_6::get_response(&buf, &mut pos)
    }

    pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
        LegacyV1_6::new(address, port, timeout_settings)?.get_info()
    }
}
