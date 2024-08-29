/// GameSpy 1 query protocol.
#[cfg(feature = "gamespy_1")]
pub mod gamespy_1;

/// GameSpy 2 query protocol.
#[cfg(feature = "gamespy_2")]
pub mod gamespy_2;

/// GameSpy 3 query protocol.
#[cfg(feature = "gamespy_3")]
pub mod gamespy_3;

/// Mumble query protocol.
#[cfg(feature = "mumble")]
pub mod mumble;

/// Quake 1 query protocol.
#[cfg(feature = "quake_1")]
pub mod quake_1;

/// Quake 2 query protocol.
#[cfg(feature = "quake_2")]
pub mod quake_2;

/// Quake 3 query protocol.
#[cfg(feature = "quake_3")]
pub mod quake_3;

/// TeamSpeak 2 query protocol.
#[cfg(feature = "teamspeak_2")]
pub mod teamspeak_2;

/// TeamSpeak 3 query protocol.
#[cfg(feature = "teamspeak_3")]
pub mod teamspeak_3;

/// Unreal Engine 2 query protocol.
#[cfg(feature = "unreal_2")]
pub mod unreal_2;

/// Valve Source Engine query protocol.
#[cfg(feature = "valve")]
pub mod valve;

/// Valve GoldSrc Engine query protocol.
#[cfg(feature = "valve_gold_src")]
pub mod valve_gold_src;
