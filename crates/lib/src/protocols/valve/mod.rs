/// The implementation.
pub mod protocol;
/// All types used by the implementation.
pub mod types;

pub use protocol::*;
pub use types::*;

/// Generate a module containing a query function for a valve game.
///
/// * `mod_name` - The name to be given to the game module (see ID naming
///   conventions in CONTRIBUTING.md).
/// * `pretty_name` - The full name of the game, will be used as the
///   documentation for the created module.
/// * `steam_app`, `default_port` - Passed through to [game_query_fn].
#[cfg(feature = "games")]
macro_rules! game_query_mod {
    ($mod_name: ident, $pretty_name: expr, $engine: expr, $default_port: literal) => {
        crate::protocols::valve::game_query_mod!(
            $mod_name,
            $pretty_name,
            $engine,
            $default_port,
            GatheringSettings::default()
        );
    };

    ($mod_name: ident, $pretty_name: expr, $engine: expr, $default_port: literal, $gathering_settings: expr) => {
        #[doc = $pretty_name]
        pub mod $mod_name {
            #[allow(unused_imports)]
            use crate::protocols::{
                types::GatherToggle,
                valve::{Engine, GatheringSettings},
            };

            crate::protocols::valve::game_query_fn!($pretty_name, $engine, $default_port, $gathering_settings);
        }
    };
}

#[cfg(feature = "games")]
pub(crate) use game_query_mod;

// Allow generating doc comments:
// https://users.rust-lang.org/t/macros-filling-text-in-comments/20473
/// Generate a query function for a valve game.
///
/// * `engine` - The [Engine] that the game uses.
/// * `default_port` - The default port the game uses.
///
/// ```rust,ignore
/// use crate::protocols::valve::game_query_fn;
/// game_query_fn!(TEAMFORTRESS2, 27015);
/// ```
#[cfg(feature = "games")]
macro_rules! game_query_fn {
    ($pretty_name: expr, $engine: expr, $default_port: literal, $gathering_settings: expr) => {
        // TODO: By using $gathering_settings, also add to doc if a game doesnt respond to certain gathering settings
        crate::protocols::valve::game_query_fn!{@gen $engine, $default_port, concat!(
            "Make a valve query for ", $pretty_name, " with default timeout settings and default extra request settings.\n\n",
            "If port is `None`, then the default port (", stringify!($default_port), ") will be used."), $gathering_settings}
    };

    (@gen $engine: expr, $default_port: literal, $doc: expr, $gathering_settings: expr) => {
        #[doc = $doc]
        pub fn query(address: &std::net::IpAddr, port: Option<u16>) -> crate::GDResult<crate::protocols::valve::game::Response> {
            let valve_response = crate::protocols::valve::query(
                &std::net::SocketAddr::new(*address, port.unwrap_or($default_port)),
                $engine,
                Some($gathering_settings),
                None,
            )?;

            Ok(crate::protocols::valve::game::Response::new_from_valve_response(valve_response))
        }
    };
}

#[cfg(feature = "games")]
pub(crate) use game_query_fn;
