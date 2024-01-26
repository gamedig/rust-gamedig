//! Generic query functions

use std::net::{IpAddr, SocketAddr};

use crate::games::types::Game;
use crate::games::{eco, ffow, jc2m, minecraft, savage2, theship};
use crate::protocols;
use crate::protocols::gamespy::GameSpyVersion;
use crate::protocols::quake::QuakeVersion;
use crate::protocols::types::{CommonResponse, ExtraRequestSettings, ProprietaryProtocol, Protocol, TimeoutSettings};
use crate::GDResult;

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
                    .or_else(|| Option::from(game.request_settings.clone()))
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
                #[cfg(feature = "serde")]
                ProprietaryProtocol::Eco => eco::query_with_timeout(address, port, &timeout_settings).map(Box::new)?,
            }
        }
    })
}
