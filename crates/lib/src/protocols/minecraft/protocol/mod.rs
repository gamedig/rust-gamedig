use crate::protocols::minecraft::types::RequestSettings;
use crate::{
    protocols::minecraft::{
        protocol::{
            bedrock::Bedrock,
            java::Java,
            legacy_v1_3::LegacyV1_3,
            legacy_v1_5::LegacyV1_5,
            legacy_v1_6::LegacyV1_6,
        },
        BedrockResponse,
        JavaResponse,
        LegacyGroup,
    },
    protocols::types::TimeoutSettings,
    GDErrorKind::AutoQuery,
    GDResult,
};
use std::net::SocketAddr;

mod bedrock;
mod java;
mod legacy_v1_3;
mod legacy_v1_5;
mod legacy_v1_6;

/// Queries a Minecraft server with all the protocol variants one by one (Java
/// -> Bedrock -> Legacy (1.6 -> 1.4 -> Beta 1.8)).
pub fn query(
    address: &SocketAddr,
    timeout_settings: Option<TimeoutSettings>,
    request_settings: Option<RequestSettings>,
) -> GDResult<JavaResponse> {
    if let Ok(response) = query_java(address, timeout_settings, request_settings) {
        return Ok(response);
    }

    if let Ok(response) = query_bedrock(address, timeout_settings) {
        return Ok(JavaResponse::from_bedrock_response(response));
    }

    if let Ok(response) = query_legacy(address, timeout_settings) {
        return Ok(response);
    }

    Err(AutoQuery.into())
}

/// Query a Java Server.
pub fn query_java(
    address: &SocketAddr,
    timeout_settings: Option<TimeoutSettings>,
    request_settings: Option<RequestSettings>,
) -> GDResult<JavaResponse> {
    Java::query(address, timeout_settings, request_settings)
}

/// Query a (Java) Legacy Server (1.6 -> 1.4 -> Beta 1.8).
pub fn query_legacy(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<JavaResponse> {
    if let Ok(response) = query_legacy_specific(LegacyGroup::V1_6, address, timeout_settings) {
        return Ok(response);
    }

    if let Ok(response) = query_legacy_specific(LegacyGroup::V1_5, address, timeout_settings) {
        return Ok(response);
    }

    if let Ok(response) = query_legacy_specific(LegacyGroup::V1_3, address, timeout_settings) {
        return Ok(response);
    }

    Err(AutoQuery.into())
}

/// Query a specific (Java) Legacy Server.
pub fn query_legacy_specific(
    group: LegacyGroup,
    address: &SocketAddr,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<JavaResponse> {
    match group {
        LegacyGroup::V1_6 => LegacyV1_6::query(address, timeout_settings),
        LegacyGroup::V1_5 => LegacyV1_5::query(address, timeout_settings),
        LegacyGroup::V1_3 => LegacyV1_3::query(address, timeout_settings),
    }
}

/// Query a Bedrock Server.
pub fn query_bedrock(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<BedrockResponse> {
    Bedrock::query(address, timeout_settings)
}
