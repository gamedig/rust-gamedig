/// The implementation.
pub mod protocol;
/// All types used by the implementation.
pub mod types;

pub use protocol::*;
pub use types::*;

macro_rules! game_query_mod {
    ($mod_name: ident, $pretty_name: expr, $steam_app: ident, $default_port: literal) => {
        #[doc = $pretty_name]
        pub mod $mod_name {
            crate::protocols::valve::game_query_fn!($steam_app, $default_port);
        }
    };
}

pub(crate) use game_query_mod;

// Allow generating doc comments:
// https://users.rust-lang.org/t/macros-filling-text-in-comments/20473
/// Generate a query function for a valve game.
macro_rules! game_query_fn {
    ($steam_app: ident, $default_port: literal) => {
        crate::protocols::valve::game_query_fn!{@gen $steam_app, $default_port, concat!(
            "Make a valve query for ", stringify!($steam_app), " with default timeout settings and default extra request settings.\n\n",
            "If port is `None`, then the default port (", stringify!($default_port), ") will be used.")}
    };

    (@gen $steam_app: ident, $default_port: literal, $doc: expr) => {
        #[doc = $doc]
        pub fn query(address: &std::net::IpAddr, port: Option<u16>) -> crate::GDResult<crate::protocols::valve::game::Response> {
            let valve_response = crate::protocols::valve::query(
                &std::net::SocketAddr::new(*address, port.unwrap_or($default_port)),
                crate::protocols::valve::SteamApp::$steam_app.as_engine(),
                None,
                None,
            )?;

            Ok(crate::protocols::valve::game::Response::new_from_valve_response(valve_response))
        }
    };
}

pub(crate) use game_query_fn;
