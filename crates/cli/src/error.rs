pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Clap Error: {0}")]
    Clap(#[from] clap::Error),

    #[error("Gamedig Error: {0}")]
    Gamedig(#[from] gamedig::errors::GDError),

    #[cfg(any(feature = "json", feature = "xml"))]
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),

    #[cfg(feature = "bson")]
    #[error("Bson Error: {0}")]
    Bson(#[from] bson::ser::Error),

    #[cfg(feature = "xml")]
    #[error("Xml Error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("Unknown Game: {0}")]
    UnknownGame(String),

    #[error("Invalid hostname: {0}")]
    InvalidHostname(String),
}
