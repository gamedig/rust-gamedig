use crate::bufferer::{Bufferer, Endianess};
use crate::protocols::gamespy::two::Response;
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::{GDError, GDResult};
use std::collections::HashMap;
use std::net::SocketAddr;

struct GameSpy2 {
    socket: UdpSocket,
}

enum RequestType {
    INFO,
    PLAYERS,
    TEAMS,
    ALL,
}

impl RequestType {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            RequestType::INFO => vec![0xFF, 0x00, 0x00],
            RequestType::PLAYERS => vec![0x00, 0xFF, 0x00],
            RequestType::TEAMS => vec![0x00, 0x00, 0xFF],
            RequestType::ALL => vec![0xFF, 0xFF, 0xFF],
        }
    }
}

impl GameSpy2 {
    fn new(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self { socket })
    }

    fn request(&mut self, request: RequestType) -> GDResult<Bufferer> {
        self.socket.send(
            &*[
                vec![0xFE, 0xFD, 0x00],
                vec![0x00, 0x00, 0x00, 0x01],
                request.to_bytes(),
            ]
            .concat(),
        )?;

        let received = self.socket.receive(None)?;
        let mut buf = Bufferer::new_with_data(Endianess::Big, &received);

        if buf.get_u8()? != 0 {
            return Err(GDError::PacketBad);
        }

        if buf.get_u32()? != 1 {
            return Err(GDError::PacketBad);
        }

        Ok(buf)
    }

    fn get_server_info(&mut self) -> GDResult<HashMap<String, String>> {
        let mut values = HashMap::new();

        let mut data = self.request(RequestType::INFO)?;

        while data.remaining_length() > 0 {
            let key = data.get_string_utf8()?;
            let value = data.get_string_utf8_optional()?;

            if key.is_empty() {
                continue;
            }

            values.insert(key, value);
        }

        Ok(values)
    }
}

pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let mut client = GameSpy2::new(address, timeout_settings)?;
    let mut server_vars = client.get_server_info()?;

    Ok(Response {
        name: server_vars.remove("hostname").ok_or(GDError::PacketBad)?,
        map: server_vars.remove("mapname").ok_or(GDError::PacketBad)?,
        has_password: server_vars.remove("password").ok_or(GDError::PacketBad)? == "1",
        max_players: server_vars
            .remove("maxplayers")
            .ok_or(GDError::PacketBad)?
            .parse()
            .map_err(|_| GDError::PacketBad)?,
    })
}
