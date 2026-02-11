mod data;
mod error;
mod player;
mod query;
mod server;
mod timeout;

// Public
pub use {
    data::{GenericDataMap, GenericDataValue},
    error::{ErrorCategory, ErrorCategoryExt},
    player::{GenericPlayer, GenericPlayerExt},
    query::GenericQueryExt,
    server::{GenericServer, GenericServerExt},
    timeout::{GenericTimeoutExt, HttpTimeout, TcpTimeout, TimeoutConfig, UdpTimeout},
};

// Private
#[allow(unused)]
pub(crate) use timeout::marker::{DictMarker, HttpMarker, TcpMarker, TimeoutShape, UdpMarker};
