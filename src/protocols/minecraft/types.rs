
/*

This file contains lightly modified versions of the original code. (using only the varint parts)
Code reference: https://github.com/thisjaiden/golden_apple/blob/master/src/lib.rs

MIT License

Copyright (c) 2021-2022 Jaiden Bernard

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/

use crate::{GDError, GDResult};
use crate::utils::buffer::get_u8;

/// The type of Minecraft Server you want to query
pub enum Server {
    /// Java Edition
    Java,
    /// Legacy Java Versions
    Legacy(LegacyVersion),
    /// Bedrock Edition
    Bedrock
}

/// Legacy Java Versions
pub enum LegacyVersion {
    /// 1.6
    V1_6,
    /// 1.4 - 1.5
    V1_4,
    /// Beta 1.8 - 1.3
    BV1_8
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub id: String
}

#[derive(Debug)]
pub struct Response {
    pub version_name: String,
    pub version_protocol: i32,
    pub max_players: u32,
    pub online_players: u32,
    pub sample_players: Vec<Player>,
    pub description: String,
    pub favicon: Option<String>,
    pub previews_chat: Option<bool>,
    pub enforces_secure_chat: Option<bool>
}

pub fn get_varint(buf: &[u8], pos: &mut usize) -> GDResult<i32> {
    let mut result = 0;

    let msb: u8 = 0b10000000;
    let mask: u8 = !msb;

    for i in 0..5 {
        let current_byte = get_u8(buf, pos)?;

        result |= ((current_byte & mask) as i32) << (7 * i);

        // The 5th byte is only allowed to have the 4 smallest bits set
        if i == 4 && (current_byte & 0xf0 != 0) {
            return Err(GDError::PacketBad("VarInt Overflow".to_string()))
        }

        if (current_byte & msb) == 0 {
            break;
        }
    }

    Ok(result)
}

pub fn as_varint(value: i32) -> Vec<u8> {
    let mut bytes = vec![];
    let mut reading_value = value;

    let msb: u8 = 0b10000000;
    let mask: i32 = 0b01111111;

    for _ in 0..5 {
        let tmp = (reading_value & mask) as u8;

        reading_value &= !mask;
        reading_value = reading_value.rotate_right(7);

        if reading_value != 0 {
            bytes.push(tmp | msb);
        } else {
            bytes.push(tmp);
            break;
        }
    }

    bytes
}

pub fn get_string(buf: &[u8], pos: &mut usize) -> GDResult<String> {
    let length = get_varint(buf, pos)? as usize;
    let mut text = vec![0; length];

    for i in 0..length {
        text[i] = get_u8(buf, pos)?;
    }

    Ok(String::from_utf8(text)
        .map_err(|_| GDError::PacketBad("Minecraft bad String".to_string()))?)
}

pub fn as_string(value: String) -> Vec<u8> {
    let mut buf = as_varint(value.len() as i32);
    buf.extend(value.as_bytes().to_vec());

    buf
}
