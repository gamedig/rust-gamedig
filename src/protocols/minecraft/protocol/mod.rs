use crate::GDResult;
use crate::protocols::minecraft::{LegacyVersion, Response, Server};
use crate::protocols::minecraft::protocol::java::Java;
use crate::protocols::types::TimeoutSettings;

mod java;

pub fn query(mc_type: Server, address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    match mc_type {
        Server::Java => Java::query(address, port, timeout_settings),
        Server::Legacy(category) => match category {
            LegacyVersion::V1_6 => Java::query(address, port, timeout_settings),
            LegacyVersion::V1_4 => Java::query(address, port, timeout_settings),
            LegacyVersion::BV1_8 => Java::query(address, port, timeout_settings),
        },
        Server::Bedrock => Java::query(address, port, timeout_settings)
    }
}
