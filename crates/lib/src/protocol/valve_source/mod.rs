mod client;
mod error;
mod model;

#[cfg(feature = "attribute_converters")]
mod ext;

// Public
pub use {
    client::ValveSourceClient,
    error::ValveSourceClientError,
    model::{
        ExtraData,
        ExtraDataFlag,
        ExtraDataFlags,
        Info,
        Player,
        Server,
        ServerEnvironment,
        ServerType,
        SourceTV,
        TheShip,
        TheShipMode,
        TheShipPlayer,
    },
};

// Private
pub(crate) use model::Fragment;
