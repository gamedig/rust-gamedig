use std::{
    error::Error,
    fmt::{self, Formatter},
};

/// Result of Type and GDError.
pub type GDResult<T> = Result<T, GDError>;

/// GameDig Error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GDError {
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
    BadGame(String),
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
}

impl Error for GDError {}

impl fmt::Display for GDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing Ok variant of the GDResult type
    #[test]
    fn test_gdresult_ok() {
        let result: GDResult<u32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    // Testing Err variant of the GDResult type
    #[test]
    fn test_gdresult_err() {
        let result: GDResult<u32> = Err(GDError::InvalidInput);
        assert_eq!(result.is_err(), true);
    }

    // Testing the Display trait for the GDError type
    #[test]
    fn test_display() {
        let error = GDError::PacketOverflow;
        assert_eq!(format!("{}", error), "PacketOverflow");
    }

    // Testing the Error trait for the GDError type
    #[test]
    fn test_error_trait() {
        let error = GDError::PacketBad;
        assert!(error.source().is_none());
    }

    // Testing cloning the GDError type
    #[test]
    fn test_cloning() {
        let error = GDError::BadGame(String::from("MyGame"));
        let cloned_error = error.clone();
        assert_eq!(error, cloned_error);
    }
}
