use crate::GDError;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    /// The received packet was bigger than the buffer size.
    Overflow,
    /// The received packet was shorter than the expected one.
    Underflow,
    /// The received packet is badly formatted.
    Bad,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Parse {
    /// Couldn't decompress data.
    Decompress,
    /// Couldn't cast a value to an enum.
    UnknownEnumCast,
    /// Couldn't parse a json string.
    JsonParse,
    /// Couldn't parse a value.
    TypeParse,
    /// A protocol-defined expected format was not met.
    ProtocolFormat,
    /// The server queried is not the queried game server.
    BadGame,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Socket {
    /// Couldn't create a socket connection.
    Connect,
    /// Couldn't bind a socket.
    Bind,
    /// Couldn't send a packet.
    Send,
    /// Couldn't receive a packet.
    Receive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    /// Invalid input.
    Invalid,
    /// Couldn't automatically query.
    AutoQuery,
}

/// GameDig Error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GDErrorKind {
    Packet(Packet),
    Parse(Parse),
    Socket(Socket),
    Input(Input),
}

impl GDErrorKind {
    /// Convert error kind into a full error with a source (and implicit
    /// backtrace)
    ///
    /// ```
    /// use gamedig::{GDErrorKind, GDResult, Parse};
    /// let _: GDResult<u32> = "thing".parse().map_err(|e| GDErrorKind::Parse(Parse::TypeParse).context(e));
    /// ```
    pub fn context<E: Into<Box<dyn Error + 'static>>>(self, source: E) -> GDError { GDError::from_error(self, source) }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing cloning the GDErrorKind type
    #[test]
    fn test_cloning() {
        let error = GDErrorKind::Parse(Parse::BadGame);
        let cloned_error = error.clone();
        assert_eq!(error, cloned_error);
    }

    // test display GDError
    #[test]
    fn test_display() {
        let err = Parse::BadGame.context("Rust is not a game");
        assert_eq!(
            format!("{err}"),
            "GDError{ kind=BadGame\n  source=\"Rust is not a game\"\n  backtrace=<disabled>\n}\n"
        );
    }
}
