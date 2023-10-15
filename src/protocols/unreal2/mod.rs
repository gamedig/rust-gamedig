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
/// * `default_port` - Passed through to [game_query_fn].
macro_rules! game_query_mod {
    ($mod_name: ident, $pretty_name: expr, $default_port: literal) => {
        #[doc = $pretty_name]
        pub mod $mod_name {
            crate::protocols::unreal2::game_query_fn!($default_port);
        }
    };
}

pub(crate) use game_query_mod;

// Allow generating doc comments:
// https://users.rust-lang.org/t/macros-filling-text-in-comments/20473
/// Generate a query function for a valve game.
///
/// * `default_port` - The default port the game uses.
macro_rules! game_query_fn {
    ($default_port: literal) => {
        crate::protocols::unreal2::game_query_fn! {@gen $default_port, concat!(
        "Make a Unreal2 query for with default timeout settings and default extra request settings.\n\n",
        "If port is `None`, then the default port (", stringify!($default_port), ") will be used.")}
    };

    (@gen $default_port: literal, $doc: expr) => {
        #[doc = $doc]
        pub fn query(
            address: &std::net::IpAddr,
            port: Option<u16>,
        ) -> crate::GDResult<crate::protocols::unreal2::Response> {
            crate::protocols::unreal2::query(
                &std::net::SocketAddr::new(*address, port.unwrap_or($default_port)),
                None,
            )
        }
    };
}

pub(crate) use game_query_fn;
