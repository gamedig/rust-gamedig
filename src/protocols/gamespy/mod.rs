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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionedResponse<'a> {
    One(&'a one::Response),
    Two(&'a two::Response),
    Three(&'a three::Response),
}

/// Versioned player type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionedPlayer<'a> {
    One(&'a one::Player),
    Two(&'a two::Player),
    Three(&'a three::Player),
}

/// Generate a module containing a query function for a gamespy game.
macro_rules! game_query_mod {
    ($mod_name: ident, $pretty_name: expr, $gamespy_ver: ident, $default_port: literal) => {
        #[doc = $pretty_name]
        pub mod $mod_name {
            crate::protocols::gamespy::game_query_fn!($gamespy_ver, $default_port);
        }
    };
}

pub(crate) use game_query_mod;

// Allow generating doc comments:
// https://users.rust-lang.org/t/macros-filling-text-in-comments/20473
/// Generate a query function for a gamespy game.
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

pub(crate) use game_query_fn;
