use crate::bufferer::{Bufferer, Endianess};
use crate::protocols::gamespy::{Player, Response};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::{GDError, GDResult};
use std::collections::HashMap;

/// Query a server by providing the address, the port and timeout settings.
/// Providing None to the timeout settings results in using the default values.
/// (TimeoutSettings::[default](TimeoutSettings::default)).
#[allow(unused_variables)]
pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<()> {
    let mut socket = UdpSocket::new(address, port)?;
    socket.apply_timeout(timeout_settings)?;

    let challenge: Option<i32> = None;
    let payload: Option<Vec<u8>> = None;

    let challenge_length = match challenge {
        Some(_) => 4,
        None => 0,
    };
    let payload_length = match payload {
        Some(d) => d.len(),
        None => 0,
    };

    // getting the challenge
    let data = [0xFE, 0xFD, 0x09, 0, 0, 0, 9];
    // add challenge length
    // add payload length????

    socket.send(&data)?;
    let mut received = socket.receive(None)?;
    let mut buf = Bufferer::new_with_data(Endianess::Little, &received);

    let mut received_kind = buf.get_u8()?;
    if received_kind != 9 {
        return Err(GDError::PacketBad);
    }

    let mut session_id = buf.get_u32()?;
    println!("challenge: {}", session_id);

    let challenge = buf.get_u16()?;

    // asking for data with challenge
    let payload_request_form: [u8; 4] = [0xff, 0xff, 0xff, 0x01];
    let mut base_data: Vec<u8> = [0xFE, 0xFD, 0x00, 0, 0, 0, 1].to_vec();
    base_data.extend_from_slice(&challenge.to_le_bytes());
    base_data.extend_from_slice(&payload_request_form);

    socket.send(&data)?;
    received = socket.receive(None)?;
    buf = Bufferer::new_with_data(Endianess::Little, &received);

    received_kind = buf.get_u8()?;
    println!("data: {}", received_kind);
    if received_kind != 9 {
        return Err(GDError::PacketBad);
    }

    session_id = buf.get_u32()?;
    println!("data: {}", session_id);

    println!("{:02X?}", buf.remaining_data());

    Ok(())
}
