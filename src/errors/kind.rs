use crate::error::ErrorSource;
use crate::GDError;

/// All GameDig Error kinds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GDErrorKind {
    /// The received packet was bigger than the buffer size.
    PacketOverflow,
    /// The received packet was shorter than the expected one.
    PacketUnderflow,
    /// The received packet is badly formatted.
    PacketBad,
    /// Couldn't send the packet.
    PacketSend,
    /// Couldn't send the receive.
    PacketReceive,
    /// Couldn't decompress data.
    Decompress,
    /// Couldn't create a socket connection.
    SocketConnect,
    /// Couldn't bind a socket.
    SocketBind,
    /// Invalid input.
    InvalidInput,
    /// The server queried is not the queried game server.
    BadGame,
    /// Couldn't automatically query.
    AutoQuery,
    /// A protocol-defined expected format was not met.
    ProtocolFormat,
    /// Couldn't cast a value to an enum.
    UnknownEnumCast,
    /// Couldn't parse a json string.
    JsonParse,
    /// Couldn't parse a value.
    TypeParse,
    /// Out of memory.
    OutOfMemory,
}

impl GDErrorKind {
    /// Convert error kind into a full error with a source (and implicit
    /// backtrace)
    ///
    /// ```
    /// use gamedig::{GDErrorKind, GDResult};
    /// let _: GDResult<u32> = "thing".parse().map_err(|e| GDErrorKind::TypeParse.context(e));
    /// ```
    pub fn context<E: Into<ErrorSource>>(self, source: E) -> GDError { GDError::from_error(self, source) }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing cloning the GDErrorKind type
    #[test]
    fn test_cloning() {
        let error = GDErrorKind::BadGame;
        let cloned_error = error.clone();
        assert_eq!(error, cloned_error);
    }

    // test display GDError
    #[test]
    fn test_display() {
        let err = GDErrorKind::BadGame.context("Rust is not a game");
        assert_eq!(
            format!("{err}"),
            "GDError{ kind=BadGame\n  source=\"Rust is not a game\"\n  backtrace=<disabled>\n}\n"
        );
    }
}
