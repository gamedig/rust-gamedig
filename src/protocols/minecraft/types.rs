
/*
Although its a lightly modified version, this file contains code
by Jaiden Bernard (2021-2022 - MIT) from
https://github.com/thisjaiden/golden_apple/blob/master/src/lib.rs
*/

use crate::{GDError, GDResult};
use crate::utils::buffer::get_u8;

/// The type of Minecraft Server you want to query.
#[derive(Debug)]
pub enum Server {
    /// Java Edition.
    Java,
    /// Legacy Java.
    Legacy(LegacyGroup),
    /// Bedrock Edition.
    Bedrock
}

/// Legacy Java (Versions) Groups.
#[derive(Debug)]
pub enum LegacyGroup {
    /// 1.6
    V1_6,
    /// 1.4 - 1.5
    V1_4,
    /// Beta 1.8 - 1.3
    VB1_8
}

/// Information about a player.
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub id: String
}

/// A query response.
#[derive(Debug)]
pub struct Response {
    /// Version name, example: "1.19.2".
    pub version_name: String,
    /// Version protocol, example: 760 (for 1.19.2).
    pub version_protocol: i32,
    /// Number of server capacity.
    pub max_players: u32,
    /// Number of online players.
    pub online_players: u32,
    /// Some online players (can be missing).
    pub sample_players: Option<Vec<Player>>,
    /// Server's description or MOTD.
    pub description: String,
    /// The favicon (can be missing).
    pub favicon: Option<String>,
    /// Tells if the chat preview is enabled (can be missing).
    pub previews_chat: Option<bool>,
    /// Tells if secure chat is enforced (can be missing).
    pub enforces_secure_chat: Option<bool>,
    /// Tell's the server type.
    pub server_type: Server
}

/// A Bedrock Edition query response.
#[derive(Debug)]
pub struct BedrockResponse {
    /// Server edition.
    pub edition: String,
    /// Server name.
    pub name: String,
    /// Version name, example: "1.19.40".
    pub version_name: String,
    /// Version protocol, example: 760 (for 1.19.2).
    pub version_protocol: String,
    /// Number of server capacity.
    pub max_players: u32,
    /// Number of online players.
    pub online_players: u32,
    /// Server id.
    pub id: Option<String>,
    /// The map.
    pub map: Option<String>,
    /// Game mode.
    pub game_mode: Option<GameMode>,
    /// Tell's the server type.
    pub server_type: Server
}

impl Response {
    pub fn from_bedrock_response(response: BedrockResponse) -> Self {
        Self {
            version_name: response.version_name,
            version_protocol: 0,
            max_players: response.max_players,
            online_players: response.online_players,
            sample_players: None,
            description: response.name,
            favicon: None,
            previews_chat: None,
            enforces_secure_chat: None,
            server_type: Server::Bedrock
        }
    }
}

pub fn port_or_java_default(port: Option<u16>) -> u16 {
    match port {
        None => 25565,
        Some(port) => port
    }
}

pub fn port_or_bedrock_default(port: Option<u16>) -> u16 {
    match port {
        None => 19132,
        Some(port) => port
    }
}

/// A server's game mode (used only by Bedrock servers).
#[derive(Debug)]
pub enum GameMode {
    Survival, Creative, Hardcore, Spectator, Adventure
}

impl GameMode {
    pub fn from_bedrock(value: &&str) -> GDResult<Self> {
        match *value {
            "Survival" => Ok(GameMode::Survival),
            "Creative" => Ok(GameMode::Creative),
            "Hardcore" => Ok(GameMode::Hardcore),
            "Spectator" => Ok(GameMode::Spectator),
            "Adventure" => Ok(GameMode::Adventure),
            _ => Err(GDError::UnknownEnumCast)
        }
    }
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
            return Err(GDError::PacketBad("Couldn't parse to VarInt: Overflow.".to_string()))
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
        .map_err(|_| GDError::PacketBad("Couldn't parse to a Minecraft String.".to_string()))?)
}

pub fn as_string(value: String) -> Vec<u8> {
    let mut buf = as_varint(value.len() as i32);
    buf.extend(value.as_bytes().to_vec());

    buf
}
