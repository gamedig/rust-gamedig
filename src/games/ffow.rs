use std::net::{IpAddr, SocketAddr};
use crate::protocols::types::TimeoutSettings;
use crate::protocols::valve::{Engine, Environment, Server, ValveProtocol};
use crate::GDResult;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The query response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Response {
    /// Protocol used by the server.
    pub protocol: u8,
    /// Name of the server.
    pub name: String,
    /// Map name.
    pub active_mod: String,
    /// Running game mode.
    pub game_mode: String,
    /// Description of the server.
    pub description: String,
    /// The version that the server is running on.
    pub version: String,
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

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> {
    query_with_timeout(address, port, TimeoutSettings::default())
}

pub fn query_with_timeout(address: &IpAddr, port: Option<u16>, timeout_settings: TimeoutSettings) -> GDResult<Response> {
    let mut client = ValveProtocol::new(&SocketAddr::new(*address, port.unwrap_or(5478)), Some(timeout_settings))?;
    let mut buffer = client.get_request_data(
        &Engine::GoldSrc(true),
        0,
        0x46,
        String::from("LSQ").into_bytes(),
    )?;

    let protocol = buffer.get_u8()?;
    let name = buffer.get_string_utf8()?;
    let map = buffer.get_string_utf8()?;
    let active_mod = buffer.get_string_utf8()?;
    let game_mode = buffer.get_string_utf8()?;
    let description = buffer.get_string_utf8()?;
    let version = buffer.get_string_utf8()?;
    buffer.move_position_ahead(2);
    let players_online = buffer.get_u8()?;
    let players_maximum = buffer.get_u8()?;
    let server_type = Server::from_gldsrc(buffer.get_u8()?)?;
    let environment_type = Environment::from_gldsrc(buffer.get_u8()?)?;
    let has_password = buffer.get_u8()? == 1;
    let vac_secured = buffer.get_u8()? == 1;
    buffer.move_position_ahead(1); //average fps
    let round = buffer.get_u8()?;
    let rounds_maximum = buffer.get_u8()?;
    let time_left = buffer.get_u16()?;

    Ok(Response {
        protocol,
        name,
        active_mod,
        game_mode,
        description,
        version,
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
