#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub(crate) mod common;
/// The implementations.
pub mod protocols;

pub use protocols::*;

/// Versions of the gamespy protocol
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameSpyVersion {
    One,
    Two,
    Three,
}

/// Versioned response type
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionedResponse<'a> {
    One(&'a one::Response),
    Two(&'a two::Response),
    Three(&'a three::Response),
}

/// Versioned player type
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionedPlayer<'a> {
    One(&'a one::Player),
    Two(&'a two::Player),
    Three(&'a three::Player),
}

/// Generate a module containing a query function for a gamespy game.
///
/// * `mod_name` - The name to be given to the game module (see ID naming
///   conventions in CONTRIBUTING.md).
/// * `pretty_name` - The full name of the game, will be used as the
///   documentation for the created module.
/// * `gamespy_ver`, `default_port` - Passed through to [game_query_fn].
#[cfg(feature = "games")]
macro_rules! game_query_mod {
    ($mod_name: ident, $pretty_name: expr, $gamespy_ver: ident, $default_port: literal) => {
        #[doc = $pretty_name]
        pub mod $mod_name {
            crate::protocols::gamespy::game_query_fn!($gamespy_ver, $default_port);
        }
    };
}

#[cfg(feature = "games")]
pub(crate) use game_query_mod;

// Allow generating doc comments:
// https://users.rust-lang.org/t/macros-filling-text-in-comments/20473
/// Generate a query function for a gamespy game.
///
/// * `gamespy_ver` - The name of the [module](crate::protocols::gamespy) for
///   the gamespy version the game uses.
/// * `default_port` - The default port the game uses.
///
/// ```rust,ignore
/// use crate::protocols::gamespy::game_query_fn;
/// game_query_fn!(one, 7778);
/// ```
#[cfg(feature = "games")]
macro_rules! game_query_fn {
    ($gamespy_ver: ident, $default_port: literal) => {
        crate::protocols::gamespy::game_query_fn! {@gen $gamespy_ver, $default_port, concat!(
        "Make a gamespy ", stringify!($gamespy_ver), " query with default timeout settings.\n\n",
        "If port is `None`, then the default port (", stringify!($default_port), ") will be used.")}
    };

    (@gen $gamespy_ver: ident, $default_port: literal, $doc: expr) => {
        #[doc = $doc]
        pub fn query(
            address: &std::net::IpAddr,
            port: Option<u16>,
        ) -> crate::GDResult<crate::protocols::gamespy::$gamespy_ver::Response> {
            crate::protocols::gamespy::$gamespy_ver::query(
                &std::net::SocketAddr::new(*address, port.unwrap_or($default_port)),
                None,
            )
        }
    };
}

#[cfg(feature = "games")]
pub(crate) use game_query_fn;
