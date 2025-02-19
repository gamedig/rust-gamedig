//! # Valve Query Structures
//!
//! This module implements the client side structures and constants needed to
//! query game servers running on the Source (and GoldSrc) engine. The query protocol
//! is documented in detail on the [Valve Developer Wiki: Server queries].
//!
//! [Valve Developer Wiki: Server queries]: https://developer.valvesoftware.com/wiki/Server_queries

/// Module for Server Information (`A2S_INFO`).
pub mod info;
/// Module for Player Information (`A2S_PLAYER`).
pub mod player;
/// Module for Server Rules (`A2S_RULES`).
pub mod rules;
