//! Currently supported games.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod gamespy;
pub mod quake;
pub mod unreal2;
pub mod valve;

pub use gamespy::*;
pub use quake::*;
pub use unreal2::*;
pub use valve::*;

/// Battalion 1944
pub mod battalion1944;
/// Frontlines: Fuel of War
pub mod ffow;
/// Just Cause 2: Multiplayer
pub mod jc2m;
/// Minecraft
pub mod minecraft;
/// The Ship
pub mod theship;
pub mod savage2;

use crate::protocols::gamespy::GameSpyVersion;
use crate::protocols::quake::QuakeVersion;
use crate::protocols::types::{CommonResponse, ExtraRequestSettings, ProprietaryProtocol, TimeoutSettings};
use crate::protocols::{self, Protocol};
use crate::GDResult;
use std::net::{IpAddr, SocketAddr};

/// Definition of a game
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    /// Full name of the game
    pub name: &'static str,
    /// Default port used by game
    pub default_port: u16,
    /// The protocol the game's query uses
    pub protocol: Protocol,
    /// Request settings.
    pub request_settings: ExtraRequestSettings,
}

#[cfg(feature = "game_defs")]
mod definitions;

#[cfg(feature = "game_defs")]
pub use definitions::GAMES;

/// Make a query given a game definition
#[inline]
pub fn query(game: &Game, address: &IpAddr, port: Option<u16>) -> GDResult<Box<dyn CommonResponse>> {
    query_with_timeout_and_extra_settings(game, address, port, None, None)
}

/// Make a query given a game definition and timeout settings
#[inline]
pub fn query_with_timeout(
    game: &Game,
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Box<dyn CommonResponse>> {
    query_with_timeout_and_extra_settings(game, address, port, timeout_settings, None)
}

/// Make a query given a game definition, timeout settings, and extra settings
pub fn query_with_timeout_and_extra_settings(
    game: &Game,
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
    extra_settings: Option<ExtraRequestSettings>,
) -> GDResult<Box<dyn CommonResponse>> {
    let socket_addr = SocketAddr::new(*address, port.unwrap_or(game.default_port));
    Ok(match &game.protocol {
        Protocol::Valve(engine) => {
            protocols::valve::query(
                &socket_addr,
                *engine,
                extra_settings
                    .or(Option::from(game.request_settings.clone()))
                    .map(ExtraRequestSettings::into),
                timeout_settings,
            )
            .map(Box::new)?
        }
        Protocol::Gamespy(version) => {
            match version {
                GameSpyVersion::One => protocols::gamespy::one::query(&socket_addr, timeout_settings).map(Box::new)?,
                GameSpyVersion::Two => protocols::gamespy::two::query(&socket_addr, timeout_settings).map(Box::new)?,
                GameSpyVersion::Three => {
                    protocols::gamespy::three::query(&socket_addr, timeout_settings).map(Box::new)?
                }
            }
        }
        Protocol::Quake(version) => {
            match version {
                QuakeVersion::One => protocols::quake::one::query(&socket_addr, timeout_settings).map(Box::new)?,
                QuakeVersion::Two => protocols::quake::two::query(&socket_addr, timeout_settings).map(Box::new)?,
                QuakeVersion::Three => protocols::quake::three::query(&socket_addr, timeout_settings).map(Box::new)?,
            }
        }
        Protocol::Unreal2 => {
            protocols::unreal2::query(
                &socket_addr,
                &extra_settings
                    .map(ExtraRequestSettings::into)
                    .unwrap_or_default(),
                timeout_settings,
            )
            .map(Box::new)?
        }
        Protocol::PROPRIETARY(protocol) => {
            match protocol {
                ProprietaryProtocol::Savage2 => {
                    savage2::query_with_timeout(address, port, timeout_settings).map(Box::new)?
                }
                ProprietaryProtocol::TheShip => {
                    theship::query_with_timeout(address, port, timeout_settings).map(Box::new)?
                }
                ProprietaryProtocol::FFOW => ffow::query_with_timeout(address, port, timeout_settings).map(Box::new)?,
                ProprietaryProtocol::JC2M => jc2m::query_with_timeout(address, port, timeout_settings).map(Box::new)?,
                ProprietaryProtocol::Minecraft(version) => {
                    match version {
                        Some(minecraft::Server::Java) => {
                            minecraft::protocol::query_java(
                                &socket_addr,
                                timeout_settings,
                                extra_settings.map(ExtraRequestSettings::into),
                            )
                            .map(Box::new)?
                        }
                        Some(minecraft::Server::Bedrock) => {
                            minecraft::protocol::query_bedrock(&socket_addr, timeout_settings).map(Box::new)?
                        }
                        Some(minecraft::Server::Legacy(group)) => {
                            minecraft::protocol::query_legacy_specific(*group, &socket_addr, timeout_settings)
                                .map(Box::new)?
                        }
                        None => {
                            minecraft::protocol::query(
                                &socket_addr,
                                timeout_settings,
                                extra_settings.map(ExtraRequestSettings::into),
                            )
                            .map(Box::new)?
                        }
                    }
                }
            }
        }
    })
}
