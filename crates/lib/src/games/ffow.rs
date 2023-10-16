use crate::buffer::{Buffer, Utf8Decoder};
use crate::protocols::types::{CommonResponse, TimeoutSettings};
use crate::protocols::valve::{Engine, Environment, Server, ValveProtocol};
use crate::protocols::GenericResponse;
use crate::GDResult;
use byteorder::LittleEndian;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

/// The query response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Response {
    /// Protocol used by the server.
    pub protocol_version: u8,
    /// Name of the server.
    pub name: String,
    /// Map name.
    pub active_mod: String,
    /// Running game mode.
    pub game_mode: String,
    /// The version that the server is running on.
    pub game_version: String,
    /// Description of the server.
    pub description: String,
    /// Current map.
    pub map: String,
    /// Number of players on the server.
    pub players_online: u8,
    /// Maximum number of players the server reports it can hold.
    pub players_maximum: u8,
    /// Dedicated, NonDedicated or SourceTV
    pub server_type: Server,
    /// The Operating System that the server is on.
    pub environment_type: Environment,
    /// Indicates whether the server requires a password.
    pub has_password: bool,
    /// Indicates whether the server uses VAC.
    pub vac_secured: bool,
    /// Current round index.
    pub round: u8,
    /// Maximum amount of rounds.
    pub rounds_maximum: u8,
    /// Time left for the current round in seconds.
    pub time_left: u16,
}

impl CommonResponse for Response {
    fn as_original(&self) -> GenericResponse { GenericResponse::FFOW(self) }

    fn name(&self) -> Option<&str> { Some(&self.name) }
    fn game_mode(&self) -> Option<&str> { Some(&self.game_mode) }
    fn description(&self) -> Option<&str> { Some(&self.description) }
    fn game_version(&self) -> Option<&str> { Some(&self.game_version) }
    fn map(&self) -> Option<&str> { Some(&self.map) }
    fn has_password(&self) -> Option<bool> { Some(self.has_password) }
    fn players_maximum(&self) -> u32 { self.players_maximum.into() }
    fn players_online(&self) -> u32 { self.players_online.into() }
}

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, None) }

pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response> {
    let mut client = ValveProtocol::new(
        &SocketAddr::new(*address, port.unwrap_or(5478)),
        timeout_settings,
    )?;
    let data = client.get_request_data(
        &Engine::GoldSrc(true),
        0,
        0x46,
        String::from("LSQ").into_bytes(),
    )?;

    let mut buffer = Buffer::<LittleEndian>::new(&data);

    let protocol_version = buffer.read::<u8>()?;
    let name = buffer.read_string::<Utf8Decoder>(None)?;
    let map = buffer.read_string::<Utf8Decoder>(None)?;
    let active_mod = buffer.read_string::<Utf8Decoder>(None)?;
    let game_mode = buffer.read_string::<Utf8Decoder>(None)?;
    let description = buffer.read_string::<Utf8Decoder>(None)?;
    let game_version = buffer.read_string::<Utf8Decoder>(None)?;
    buffer.move_cursor(2)?;
    let players_online = buffer.read::<u8>()?;
    let players_maximum = buffer.read::<u8>()?;
    let server_type = Server::from_gldsrc(buffer.read::<u8>()?)?;
    let environment_type = Environment::from_gldsrc(buffer.read::<u8>()?)?;
    let has_password = buffer.read::<u8>()? == 1;
    let vac_secured = buffer.read::<u8>()? == 1;
    buffer.move_cursor(1)?; //average fps
    let round = buffer.read::<u8>()?;
    let rounds_maximum = buffer.read::<u8>()?;
    let time_left = buffer.read::<u16>()?;

    Ok(Response {
        protocol_version,
        name,
        active_mod,
        game_mode,
        game_version,
        description,
        map,
        players_online,
        players_maximum,
        server_type,
        environment_type,
        has_password,
        vac_secured,
        round,
        rounds_maximum,
        time_left,
    })
}
