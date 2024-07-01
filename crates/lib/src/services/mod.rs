//! Services that are currently implemented.

/// Reference: [Master Server Query Protocol](https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol)
pub mod valve_master_server;

/// Reference: [Node-GameDig](https://github.com/gamedig/node-gamedig/blob/master/protocols/minetest.js)
#[cfg(feature = "serde")]
pub mod minetest_master_server;
