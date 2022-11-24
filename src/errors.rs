
//! The library's possible errors.

use core::fmt;
use std::fmt::Formatter;

/// Result of Type and GDError.
pub type GDResult<T> = Result<T, GDError>;

/// GameDigError.
#[derive(Debug, Clone)]
pub enum GDError {
    /// The received packet was bigger than the buffer size.
    PacketOverflow(&'static str),
    /// The received packet was shorter than the expected one.
    PacketUnderflow(&'static str),
    /// The received packet was badly formatted.
    PacketBad(&'static str),
    /// Couldn't send the packet.
    PacketSend(&'static str),
    /// Couldn't send the receive.
    PacketReceive(&'static str),
    /// Couldn't decompress data.
    Decompress(&'static str),
    /// Unknown cast while translating a value to an enum.
    UnknownEnumCast,
    /// The server queried is not from the queried game.
    BadGame(&'static str),
    /// Problems occurred while dns resolving.
    DnsResolve(&'static str),
    /// Couldn't bind a socket.
    SocketBind(&'static str),
    /// Invalid input.
    InvalidInput(&'static str),
    /// Couldn't create a socket connection.
    SocketConnect(&'static str),
    /// Couldn't parse a json string.
    JsonParse(&'static str),
    /// Couldn't automatically query.
    AutoQuery(&'static str),
    /// A protocol-defined expected format was not met.
    ProtocolFormat(&'static str),
}

impl fmt::Display for GDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GDError::PacketOverflow(details) => write!(f, "Packet overflow: {details}"),
            GDError::PacketUnderflow(details) => write!(f, "Packet underflow: {details}"),
            GDError::PacketBad(details) => write!(f, "Packet bad: {details}"),
            GDError::PacketSend(details) => write!(f, "Couldn't send a packet: {details}"),
            GDError::PacketReceive(details) => write!(f, "Couldn't receive a packet: {details}"),
            GDError::Decompress(details) => write!(f, "Couldn't decompress data: {details}"),
            GDError::UnknownEnumCast => write!(f, "Unknown enum cast encountered."),
            GDError::BadGame(details) => write!(f, "Queried another game that the supposed one: {details}"),
            GDError::DnsResolve(details) => write!(f, "DNS Resolve: {details}"),
            GDError::SocketBind(details) => write!(f, "Socket bind: {details}"),
            GDError::InvalidInput(details) => write!(f, "Invalid input: {details}"),
            GDError::SocketConnect(details) => write!(f, "Socket connect: {details}"),
            GDError::JsonParse(details) => write!(f, "Json parse: {details}"),
            GDError::AutoQuery(details) => write!(f, "Auto query: {details}"),
            GDError::ProtocolFormat(details) => write!(f, "Protocol rule: {details}"),
        }
    }
}
