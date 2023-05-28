use std::net::IpAddr;
use crate::{
    protocols::minecraft::{self, BedrockResponse, JavaResponse, LegacyGroup},
    GDError,
    GDResult,
};

/// Query with all the protocol variants one by one (Java -> Bedrock -> Legacy
/// (1.6 -> 1.4 -> Beta 1.8)).
pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<JavaResponse> {
    if let Ok(response) = query_java(address, port) {
        return Ok(response);
    }

    if let Ok(response) = query_bedrock(address, port) {
        return Ok(JavaResponse::from_bedrock_response(response));
    }

    if let Ok(response) = query_legacy(address, port) {
        return Ok(response);
    }

    Err(GDError::AutoQuery)
}

/// Query a Java Server.
pub fn query_java(address: &IpAddr, port: Option<u16>) -> GDResult<JavaResponse> {
    minecraft::query_java(address, port_or_java_default(port), None)
}

/// Query a (Java) Legacy Server (1.6 -> 1.4 -> Beta 1.8).
pub fn query_legacy(address: &IpAddr, port: Option<u16>) -> GDResult<JavaResponse> {
    minecraft::query_legacy(address, port_or_java_default(port), None)
}

/// Query a specific (Java) Legacy Server.
pub fn query_legacy_specific(group: LegacyGroup, address: &IpAddr, port: Option<u16>) -> GDResult<JavaResponse> {
    minecraft::query_legacy_specific(group, address, port_or_java_default(port), None)
}

/// Query a Bedrock Server.
pub fn query_bedrock(address: &IpAddr, port: Option<u16>) -> GDResult<BedrockResponse> {
    minecraft::query_bedrock(address, port_or_bedrock_default(port), None)
}

fn port_or_java_default(port: Option<u16>) -> u16 { port.unwrap_or(25565) }

fn port_or_bedrock_default(port: Option<u16>) -> u16 { port.unwrap_or(19132) }
