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
        self.socket.send(&[// Packet ID (FE)
            0xfe,

            // Ping payload (01)
            0x01,

            // Packet identifier for plugin message (FA)
            0xfa,

            // Length of 'MC|PingHost' string (11) as unsigned short
            0x00, 0x0b,

            // 'MC|PingHost' string as UTF-16BE
            0x00, 0x4d, 0x00, 0x43, 0x00, 0x7c, 0x00, 0x50, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x67, 0x00, 0x48, 0x00, 0x6f, 0x00, 0x73, 0x00, 0x74])?;

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

        println!("{:02X?}", buf);
        let split: Vec<&str> = packet_string.split("\x00").collect();
        println!("{}", split.len());
        if split.len() != 5 {
            return Err(GDError::PacketBad("Not right split size".to_string()));
        }

        let version_protocol = split[0].parse()
            .map_err(|_| GDError::PacketBad("Expected int".to_string()))?;
        let version_name = split[1].to_string();
        let description = split[2].to_string();
        let max_players = split[3].parse()
            .map_err(|_| GDError::PacketBad("Expected int".to_string()))?;
        let online_players = split[4].parse()
            .map_err(|_| GDError::PacketBad("Expected int".to_string()))?;

        Ok(Response {
            version_name,
            version_protocol,
            max_players,
            online_players,
            sample_players: None,
            description,
            favicon: None,
            previews_chat: None,
            enforces_secure_chat: None
        })
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

        if LegacyV1_6::is_protocol(&buf, &mut pos)? {
            return Err(GDError::PacketBad("Not good".to_string()));
        }

        LegacyV1_6::get_response(&buf, &mut pos)
    }

    pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
        LegacyV1_6::new(address, port, timeout_settings)?.get_info()
    }
}
