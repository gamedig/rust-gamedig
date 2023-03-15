#[allow(unused_imports)]
use crate::{
    protocols::types::TimeoutSettings,
    socket::{Socket, UdpSocket},
    GDResult,
    GDError::PacketBad, bufferer::{Bufferer, Endianess}
};

#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(dead_code)]
struct RequestPacket {
    /// The header is a 64-bit signed integer, but we only need the first 16
    /// bits as the header should always be `0xFEFD`.
    header: u16,
    /// The delimiter, which is always `0x00`.
    delimiter: u8,
    /// The ping value, This can be anything you want, use it to make sure the
    /// response is valid.
    ping_value: [u8; 4],
    /// Whether to return the server info and rules.
    /// `0x00` = No
    /// `0xFF` = Yes
    server_info_and_rules: u8,
    /// Whether to return the player info.
    /// `0x00` = No
    /// `0xFF` = Yes
    player_info: u8,
    /// Whether to return the team info.
    /// `0x00` = No
    /// `0xFF` = Yes
    team_info: u8,
}

impl RequestPacket {
    /// Converts the request packet to a byte array of 10 bytes.
    /// This is the format that the server expects.
    #[allow(dead_code)]
    const fn to_u8_array(&self) -> [u8; 10] {
        let header_byte: [u8; 2] = self.header.to_le_bytes();

        [
            header_byte[0],
            header_byte[1],
            self.delimiter,
            self.ping_value[0],
            self.ping_value[1],
            self.ping_value[2],
            self.ping_value[3],
            self.server_info_and_rules,
            self.player_info,
            self.team_info,
        ]
    }
}

/// The bytes of the request packet to be sent to the server
#[allow(dead_code)]
const REQUEST_PACKET_BYTES: [u8; 10] = RequestPacket {
    header: 0xFEFD,
    delimiter: 0x00,
    ping_value: [b'p', b'i', b'n', b'g'],
    server_info_and_rules: 0xFF,
    player_info: 0xFF,
    team_info: 0xFF,
}
.to_u8_array();