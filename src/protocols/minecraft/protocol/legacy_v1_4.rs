
use crate::{GDError, GDResult};
use crate::protocols::minecraft::{Player, Response};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, TcpSocket};
use crate::utils::buffer::{get_string_utf16_be, get_u16_be, get_u8};

pub struct LegacyV1_4 {
    socket: TcpSocket
}

impl LegacyV1_4 {
    fn new(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = TcpSocket::new(address, port)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self {
            socket
        })
    }

    fn send_initial_request(&mut self) -> GDResult<()> {
        self.socket.send(&[0xFE, 0x01])?;

        Ok(())
    }

    fn get_info(&mut self) -> GDResult<Response> {
        self.send_initial_request()?;

        let buf = self.socket.receive(None)?;
        let mut pos = 0;

        if get_u8(&buf, &mut pos)? != 0xFF {
            return Err(GDError::PacketBad("Expected 0xFF".to_string()));
        }

        let length = get_u16_be(&buf, &mut pos)? * 2;
        if buf.len() != (length + 3) as usize { //+ 3 because of the first byte and the u16
            return Err(GDError::PacketBad("Not right size".to_string()));
        }

        /*if buf[pos..].starts_with(&[0x00, 0xA7, 0x00, 0x31, 0x00, 0x00]) {
            return Err(GDError::PacketBad("1.6 protocol".to_string()));
        }*/

        //let sss = get_string_utf16_be(&buf, &mut pos)?;
        //println!("{sss}");

        println!("{:02X?}", &buf);

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

    pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
        LegacyV1_4::new(address, port, timeout_settings)?.get_info()
    }
}
