// Although its a lightly modified version, this file contains code
// by Jaiden Bernard (2021-2022 - MIT) from
// https://github.com/thisjaiden/golden_apple/blob/master/src/lib.rs

use crate::{
    buffer::Buffer,
    protocols::{
        types::{CommonPlayer, CommonResponse, ExtraRequestSettings, GenericPlayer},
        GenericResponse,
    },
    GDErrorKind::{InvalidInput, PacketBad, UnknownEnumCast},
    GDResult,
};

use byteorder::ByteOrder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The type of Minecraft Server you want to query.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Server {
    /// Java Edition.
    Java,
    /// Legacy Java.
    Legacy(LegacyGroup),
    /// Bedrock Edition.
    Bedrock,
}

/// Legacy Java (Versions) Groups.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LegacyGroup {
    /// 1.6
    V1_6,
    /// 1.4 - 1.5
    V1_4,
    /// Beta 1.8 - 1.3
    VB1_8,
}

/// Information about a player.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
    pub id: String,
}

impl CommonPlayer for Player {
    fn as_original(&self) -> GenericPlayer { GenericPlayer::Minecraft(self) }

    fn name(&self) -> &str { &self.name }
}

/// Versioned response type
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionedResponse<'a> {
    Bedrock(&'a BedrockResponse),
    Java(&'a JavaResponse),
}

/// A Java query response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JavaResponse {
    /// Version name, example: "1.19.2".
    pub game_version: String,
    /// Protocol version, example: 760 (for 1.19.1 or 1.19.2).
    /// Note that for versions below 1.6 this field is always -1.
    pub protocol_version: i32,
    /// Number of server capacity.
    pub players_maximum: u32,
    /// Number of online players.
    pub players_online: u32,
    /// Some online players (can be missing).
    pub players: Option<Vec<Player>>,
    /// Server's description or MOTD.
    pub description: String,
    /// The favicon (can be missing).
    pub favicon: Option<String>,
    /// Tells if the chat preview is enabled (can be missing).
    pub previews_chat: Option<bool>,
    /// Tells if secure chat is enforced (can be missing).
    pub enforces_secure_chat: Option<bool>,
    /// Tell's the server type.
    pub server_type: Server,
}

/// Java-only additional request settings.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestSettings {
    /// Some Minecraft servers do not respond as expected if this
    /// isn't a specific value, `mc.hypixel.net` is an example.
    pub hostname: String,
    /// Specifies the client [protocol version number](https://wiki.vg/Protocol_version_numbers),
    /// `-1` means anything.
    pub protocol_version: i32,
}

impl Default for RequestSettings {
    /// `hostname`: "gamedig"  
    /// `protocol_version`: -1
    fn default() -> Self {
        Self {
            hostname: "gamedig".to_string(),
            protocol_version: -1,
        }
    }
}

impl RequestSettings {
    /// Make a new *RequestSettings* with just the hostname, the protocol
    /// version defaults to -1
    pub fn new_just_hostname(hostname: String) -> Self {
        Self {
            hostname,
            protocol_version: -1,
        }
    }
}

impl From<ExtraRequestSettings> for RequestSettings {
    fn from(value: ExtraRequestSettings) -> Self {
        let default = Self::default();
        Self {
            hostname: value.hostname.unwrap_or(default.hostname),
            protocol_version: value.protocol_version.unwrap_or(default.protocol_version),
        }
    }
}

impl CommonResponse for JavaResponse {
    fn as_original(&self) -> GenericResponse { GenericResponse::Minecraft(VersionedResponse::Java(self)) }

    fn description(&self) -> Option<&str> { Some(&self.description) }
    fn players_maximum(&self) -> u32 { self.players_maximum }
    fn players_online(&self) -> u32 { self.players_online }
    fn game_version(&self) -> Option<&str> { Some(&self.game_version) }

    fn players(&self) -> Option<Vec<&dyn CommonPlayer>> {
        self.players
            .as_ref()
            .map(|players| players.iter().map(|p| p as &dyn CommonPlayer).collect())
    }
}

/// A Bedrock Edition query response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BedrockResponse {
    /// Server's edition.
    pub edition: String,
    /// Server's name.
    pub name: String,
    /// Version name, example: "1.19.40".
    pub version_name: String,
    /// Protocol version, example: 760 (for 1.19.2).
    pub protocol_version: String,
    /// Maximum number of players the server reports it can hold.
    pub players_maximum: u32,
    /// Number of players on the server.
    pub players_online: u32,
    /// Server id.
    pub id: Option<String>,
    /// Currently running map's name.
    pub map: Option<String>,
    /// Current game mode.
    pub game_mode: Option<GameMode>,
    /// Tells the server type.
    pub server_type: Server,
}

impl CommonResponse for BedrockResponse {
    fn as_original(&self) -> GenericResponse { GenericResponse::Minecraft(VersionedResponse::Bedrock(self)) }

    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn map(&self) -> Option<&str> { self.map.as_deref() }
    fn game_version(&self) -> Option<&str> { Some(&self.version_name) }
    fn players_maximum(&self) -> u32 { self.players_maximum }
    fn players_online(&self) -> u32 { self.players_online }
}

impl JavaResponse {
    pub fn from_bedrock_response(response: BedrockResponse) -> Self {
        Self {
            game_version: response.version_name,
            protocol_version: 0,
            players_maximum: response.players_maximum,
            players_online: response.players_online,
            players: None,
            description: response.name,
            favicon: None,
            previews_chat: None,
            enforces_secure_chat: None,
            server_type: Server::Bedrock,
        }
    }
}

/// A server's game mode (used only by Bedrock servers.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GameMode {
    Survival,
    Creative,
    Hardcore,
    Spectator,
    Adventure,
}

impl GameMode {
    pub fn from_bedrock(value: &&str) -> GDResult<Self> {
        match *value {
            "Survival" => Ok(Self::Survival),
            "Creative" => Ok(Self::Creative),
            "Hardcore" => Ok(Self::Hardcore),
            "Spectator" => Ok(Self::Spectator),
            "Adventure" => Ok(Self::Adventure),
            _ => Err(UnknownEnumCast.context(format!("Unknown gamemode {value:?}"))),
        }
    }
}

pub(crate) fn get_varint<B: ByteOrder>(buffer: &mut Buffer<B>) -> GDResult<i32> {
    let mut result = 0;

    let msb: u8 = 0b1000_0000;
    let mask: u8 = !msb;

    for i in 0 .. 5 {
        let current_byte = buffer.read::<u8>()?;

        result |= ((current_byte & mask) as i32) << (7 * i);

        // The 5th byte is only allowed to have the 4 smallest bits set
        if i == 4 && (current_byte & 0xf0 != 0) {
            return Err(PacketBad.context("Bad 5th byte"));
        }

        if (current_byte & msb) == 0 {
            break;
        }
    }

    Ok(result)
}

pub(crate) fn as_varint(value: i32) -> Vec<u8> {
    let mut bytes = vec![];
    let mut reading_value = value;

    let msb: u8 = 0b1000_0000;
    let mask: i32 = 0b0111_1111;

    for _ in 0 .. 5 {
        let tmp = (reading_value & mask) as u8;

        reading_value &= !mask;
        reading_value = reading_value.rotate_right(7);

        if reading_value == 0 {
            bytes.push(tmp);
            break;
        }

        bytes.push(tmp | msb);
    }

    bytes
}

pub(crate) fn get_string<B: ByteOrder>(buffer: &mut Buffer<B>) -> GDResult<String> {
    let length = get_varint(buffer)? as usize;
    let mut text = Vec::with_capacity(length);

    for _ in 0 .. length {
        text.push(buffer.read::<u8>()?)
    }

    String::from_utf8(text).map_err(|e| PacketBad.context(e))
}

pub(crate) fn as_string(value: &str) -> GDResult<Vec<u8>> {
    let length = value
        .len()
        .try_into()
        .map_err(|e| InvalidInput.context(e))?;
    let mut buf = as_varint(length);
    buf.extend(value.as_bytes());

    Ok(buf)
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test_extra_request_settings() {
        let settings = ExtraRequestSettings::default();

        let _: minecraft::RequestSettings = settings.clone().into();
    }
}
