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
macro_rules! game_query_mod {
    ($mod_name: ident, $pretty_name: expr, $steam_app: ident, $default_port: literal) => {
        crate::protocols::valve::game_query_mod!(
            $mod_name,
            $pretty_name,
            $steam_app,
            $default_port,
            GatheringSettings::default()
        );
    };

    ($mod_name: ident, $pretty_name: expr, $steam_app: ident, $default_port: literal, $gathering_settings: expr) => {
        #[doc = $pretty_name]
        pub mod $mod_name {
            use crate::protocols::valve::GatheringSettings;

            crate::protocols::valve::game_query_fn!($steam_app, $default_port, $gathering_settings);
        }
    };
}

pub(crate) use game_query_mod;

// Allow generating doc comments:
// https://users.rust-lang.org/t/macros-filling-text-in-comments/20473
/// Generate a query function for a valve game.
///
/// * `steam_app` - The entry in the [SteamApp] enum that the game uses.
/// * `default_port` - The default port the game uses.
///
/// ```rust,ignore
/// use crate::protocols::valve::game_query_fn;
/// game_query_fn!(TEAMFORTRESS2, 27015);
/// ```
macro_rules! game_query_fn {
    ($steam_app: ident, $default_port: literal, $gathering_settings: expr) => {
        crate::protocols::valve::game_query_fn!{@gen $steam_app, $default_port, concat!(
            "Make a valve query for ", stringify!($steam_app), " with default timeout settings and default extra request settings.\n\n",
            "If port is `None`, then the default port (", stringify!($default_port), ") will be used."), $gathering_settings}
    };

    (@gen $steam_app: ident, $default_port: literal, $doc: expr, $gathering_settings: expr) => {
        #[doc = $doc]
        pub fn query(address: &std::net::IpAddr, port: Option<u16>) -> crate::GDResult<crate::protocols::valve::game::Response> {
            let valve_response = crate::protocols::valve::query(
                &std::net::SocketAddr::new(*address, port.unwrap_or($default_port)),
                crate::protocols::valve::SteamApp::$steam_app.as_engine(),
                Some($gathering_settings),
                None,
            )?;

            Ok(crate::protocols::valve::game::Response::new_from_valve_response(valve_response))
        }
    };
}

pub(crate) use game_query_fn;
