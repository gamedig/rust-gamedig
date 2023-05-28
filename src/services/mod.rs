//! Services that are currently implemented.

/// Reference: [Master Server Query Protocol](https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol)
#[cfg(feature="service_valve")]
pub mod valve_master_server;
