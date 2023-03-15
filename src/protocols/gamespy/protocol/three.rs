use crate::bufferer::{Bufferer, Endianess};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::{GDError, GDResult};
// use std::collections::HashMap;

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

        if packet.len() == 7 {
            packet.extend_from_slice(&[0, 0, 0, 0]);
        }

        packet
    }
}

pub fn send_data_request(socket: &mut UdpSocket, challenge: Option<i32>) -> GDResult<()> {
    let challenge_packet = RequestPacket {
        header: 65277,
        kind: 0,
        session_id: THIS_SESSION_ID,
        challenge,
        payload: Some([0xff, 0xff, 0xff, 0x01]),
    }
    .to_bytes();
    println!("sending: {:02X?}", challenge_packet);

    socket.send(&challenge_packet)?;

    Ok(())
}

/// Query a server by providing the address, the port and timeout settings.
/// Providing None to the timeout settings results in using the default values.
/// (TimeoutSettings::[default](TimeoutSettings::default)).
#[allow(unused_variables)]
pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<()> {
    let mut socket = UdpSocket::new(address, port)?;
    socket.apply_timeout(timeout_settings)?;

    let initial_packet = RequestPacket {
        header: 65277,
        kind: 9,
        session_id: THIS_SESSION_ID,
        challenge: None,
        payload: None,
    }
    .to_bytes();
    println!("sending: {:02X?}", initial_packet);

    socket.send(&initial_packet)?;
    let mut received = socket.receive(Some(16))?;
    let mut buf = Bufferer::new_with_data(Endianess::Big, &received);
    println!("received: {:02X?}", buf.remaining_data());

    let mut received_kind = buf.get_u8()?;
    if received_kind != 9 {
        return Err(GDError::PacketBad);
    }

    let mut session_id = buf.get_u32()?;
    if session_id != THIS_SESSION_ID {
        return Err(GDError::PacketBad);
    }

    let challenge_as_string = buf.get_string_utf8().unwrap();
    let challenge = challenge_as_string.parse().unwrap();

    let challenge_as_option = match challenge == 0 {
        true => None,
        false => Some(challenge),
    };

    send_data_request(&mut socket, challenge_as_option)?;

    received = socket.receive(Some(2048))?;
    buf = Bufferer::new_with_data(Endianess::Big, &received);

    received_kind = buf.get_u8()?;
    if received_kind != 0 {
        return Err(GDError::PacketBad);
    }

    session_id = buf.get_u32()?;
    if session_id != THIS_SESSION_ID {
        return Err(GDError::PacketBad);
    }

    println!("remaining: {:02X?}", buf.remaining_data());

    Ok(())
}
