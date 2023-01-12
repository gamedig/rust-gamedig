
//! The library's possible errors.

/// Result of Type and GDError.
pub type GDResult<T> = Result<T, GDError>;

/// GameDigError.
#[derive(Debug, Clone)]
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
    /// Unknown cast while translating a value to an enum.
    UnknownEnumCast,
    /// Invalid input.
    InvalidInput,
    /// Couldn't create a socket connection.
    SocketConnect,
    /// Couldn't bind a socket.
    SocketBind,
    /// Couldn't parse a json string.
    JsonParse,
    /// The server queried is not from the queried game.
    BadGame,
    /// Couldn't automatically query.
    AutoQuery,
    /// A protocol-defined expected format was not met.
    ProtocolFormat,
    /// Couldn't parse a value.
    TypeParse,
}
