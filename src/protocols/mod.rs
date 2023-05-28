//! Protocols that are currently implemented.
//!
//! A protocol will be here if it supports multiple entries, if not, its
//! implementation will be in that specific needed place, a protocol can be
//! independently queried.

/// Reference: [node-GameDig](https://github.com/gamedig/node-gamedig/blob/master/protocols/gamespy1.js)
#[cfg(feature="protocol_gamespy")]
pub mod gamespy;
/// Reference: [Server List Ping](https://wiki.vg/Server_List_Ping)
#[cfg(feature="protocol_minecraft")]
pub mod minecraft;
/// General types that are used by all protocols.
pub mod types;
/// Reference: [Server Query](https://developer.valvesoftware.com/wiki/Server_queries)
#[cfg(feature="protocol_valve")]
pub mod valve;
