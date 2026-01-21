pub mod data;

pub mod error;
/// Adapter for converting player data.
///
/// The `player` module provides functionality for converting raw player data
/// from various game server query protocols into a standardized format. This
/// is useful for applications that need to process or display player information
/// consistently, regardless of the underlying game or protocol being used.
pub mod player;

/// Adapter for converting server data.
///
/// The `server` module is designed to convert raw server data retrieved from
/// different game server query protocols into a standardized format. This
/// ensures that server information such as status, map details, and player
/// counts can be processed and utilized uniformly.
pub mod server;
