pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Clap Error: {0}")]
    Clap(#[from] clap::Error),

    #[error("Gamedig Error: {0}")]
    Gamedig(#[from] gamedig::errors::GDError),

    #[error("Strum Error: {0}")]
    Strum(#[from] strum::ParseError),
}