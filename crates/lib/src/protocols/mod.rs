//! Protocols that are currently implemented.
//!
//! A protocol will be here if it supports multiple entries, if not, its
//! implementation will be in that specific needed place, a protocol can be
//! independently queried.

/// Reference: [node-GameDig](https://github.com/gamedig/node-gamedig/blob/master/protocols/gamespy1.js)
pub mod gamespy;
/// Reference: [node-GameDig](https://github.com/gamedig/node-gamedig/blob/master/protocols/quake1.js)
pub mod quake;
/// General types that are used by all protocols.
pub mod types;
/// Reference: [node-GameDig](https://github.com/gamedig/node-gamedig/blob/master/protocols/unreal2.js)
pub mod unreal2;
/// Reference: [Server Query](https://developer.valvesoftware.com/wiki/Server_queries)
pub mod valve;

pub use types::{GenericResponse, Protocol};
