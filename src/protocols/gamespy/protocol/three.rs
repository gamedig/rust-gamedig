use crate::bufferer::{Bufferer, Endianess};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::{GDError, GDResult};
use std::collections::HashMap;

const THIS_SESSION_ID: u32 = 1;

struct RequestPacket {
    header: u16,
    kind: u8,
    session_id: u32,
    challenge: Option<i32>,
    payload: Option<[u8; 4]>,
}

impl RequestPacket {
    fn to_bytes(self) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::with_capacity(7);
        packet.extend_from_slice(&self.header.to_be_bytes());
        packet.push(self.kind);
        packet.extend_from_slice(&self.session_id.to_be_bytes());

        if let Some(challenge) = self.challenge {
            packet.extend_from_slice(&challenge.to_be_bytes());
        }

        if let Some(payload) = self.payload {
            packet.extend_from_slice(&payload);
        }

        packet
    }
}

struct GameSpy3 {
    socket: UdpSocket,
}

static PACKET_SIZE: usize = 2048;

impl GameSpy3 {
    fn new(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address, port)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self { socket })
    }

    fn receive(&mut self, size: Option<usize>, kind: u8) -> GDResult<Bufferer> {
        let received = self.socket.receive(size.or(Some(PACKET_SIZE)))?;
        let mut buf = Bufferer::new_with_data(Endianess::Big, &received);

        if buf.get_u8()? != kind {
            return Err(GDError::PacketBad);
        }

        if buf.get_u32()? != THIS_SESSION_ID {
            return Err(GDError::PacketBad);
        }

        Ok(buf)
    }

    fn make_initial_handshake(&mut self) -> GDResult<Option<i32>> {
        self.socket.send(
            &RequestPacket {
                header: 65277,
                kind: 9,
                session_id: THIS_SESSION_ID,
                challenge: None,
                payload: None,
            }
            .to_bytes(),
        )?;

        let mut buf = self.receive(Some(16), 9)?;

        let challenge_as_string = buf.get_string_utf8()?;
        let challenge = challenge_as_string
            .parse()
            .map_err(|_| GDError::TypeParse)?;

        Ok(match challenge == 0 {
            true => None,
            false => Some(challenge),
        })
    }

    fn send_data_request(&mut self, challenge: Option<i32>) -> GDResult<()> {
        self.socket.send(
            &RequestPacket {
                header: 65277,
                kind: 0,
                session_id: THIS_SESSION_ID,
                challenge,
                payload: Some([0xff, 0xff, 0xff, 0x01]),
            }
            .to_bytes(),
        )
    }
}

/// Query a server by providing the address, the port and timeout settings.
/// Providing None to the timeout settings results in using the default values.
/// (TimeoutSettings::[default](TimeoutSettings::default)).
pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<()> {
    let mut gs3 = GameSpy3::new(address, port, timeout_settings)?;

    let challenge = gs3.make_initial_handshake()?;
    gs3.send_data_request(challenge)?;

    let mut buf = gs3.receive(None, 0)?;

    if buf.get_string_utf8()? != "splitnum" {
        return Err(GDError::PacketBad);
    }

    let id = buf.get_u8()?;
    let is_last = (id & 0x80) > 0;
    let packet_id = id & 0x7f;
    println!("id: {}, is_last: {}, packet_id: {}", id, is_last, packet_id);

    buf.move_position_ahead(1); //unused byte

    // to do: manage multiple packets

    let mut values: HashMap<String, String> = HashMap::new();

    while buf.remaining_length() > 0 {
        let key = buf.get_string_utf8()?;
        if key.is_empty() {
            continue;
        }

        let value = buf.get_string_utf8()?;

        values.insert(key, value);
    }

    println!("{:#?}", values);

    Ok(())
}
