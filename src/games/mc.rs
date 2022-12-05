use crate::GDResult;
use crate::protocols::minecraft;
use crate::protocols::minecraft::{Response, LegacyGroup, BedrockResponse};

/// Query with all the protocol variants one by one (Java -> Bedrock -> Legacy (1.6 -> 1.4 -> Beta 1.8)).
pub fn query(address: &str, port: Option<u16>) -> GDResult<Response> {
    minecraft::query(address, port, None)
}

/// Query a Java Server.
pub fn query_java(address: &str, port: Option<u16>) -> GDResult<Response> {
    minecraft::query_java(address, port, None)
}

/// Query a (Java) Legacy Server (1.6 -> 1.4 -> Beta 1.8).
pub fn query_legacy(address: &str, port: Option<u16>) -> GDResult<Response> {
    minecraft::query_legacy(address, port, None)
}

/// Query a specific (Java) Legacy Server.
pub fn query_legacy_specific(group: LegacyGroup, address: &str, port: Option<u16>) -> GDResult<Response> {
    minecraft::query_legacy_specific(group, address, port, None)
}

/// Query a Bedrock Server.
pub fn query_bedrock(address: &str, port: Option<u16>) -> GDResult<BedrockResponse> {
    minecraft::query_bedrock(address, port, None)
}
