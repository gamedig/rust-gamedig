/// The implementation.
pub mod protocol;
/// All types used by the implementation.
pub mod types;

pub use protocol::*;
pub use types::*;

/// Generate a module containing a query function for an epic (EOS) game.
///
/// * `mod_name` - The name to be given to the game module (see ID naming
///   conventions in CONTRIBUTING.md).
/// * `pretty_name` - The full name of the game, will be used as the
///   documentation for the created module.
/// * `steam_app`, `default_port` - Passed through to
///   [crate::protocols::epic::game_query_fn].
#[cfg(feature = "games")]
macro_rules! game_query_mod {
    ($mod_name: ident, $pretty_name: expr, $default_port: literal, $credentials: expr) => {
        #[doc = $pretty_name]
        pub mod $mod_name {
            use crate::protocols::epic::Credentials;

            crate::protocols::epic::game_query_fn!($pretty_name, $default_port, $credentials);
        }
    };
}

#[cfg(feature = "games")]
pub(crate) use game_query_mod;

/// Generate a query function for an epic (EOS) game.
///
/// * `default_port` - The default port the game uses.
/// * `credentials` - Credentials to access EOS.
#[cfg(feature = "games")]
macro_rules! game_query_fn {
    ($pretty_name: expr, $default_port: literal, $credentials: expr) => {
        crate::protocols::epic::game_query_fn! {@gen $default_port, concat!(
        "Make a Epic query for ", $pretty_name, ".\n\n",
        "If port is `None`, then the default port (", stringify!($default_port), ") will be used."), $credentials}
    };

    (@gen $default_port: literal, $doc: expr, $credentials: expr) => {
        #[doc = $doc]
        pub fn query(
            address: &std::net::IpAddr,
            port: Option<u16>,
        ) -> crate::GDResult<crate::protocols::epic::Response> {
            crate::protocols::epic::query(
                $credentials,
                &std::net::SocketAddr::new(*address, port.unwrap_or($default_port)),
            )
        }
    };
}

#[cfg(feature = "games")]
pub(crate) use game_query_fn;
