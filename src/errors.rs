use core::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum GDError {
    PacketOverflow(String),
    PacketUnderflow(String),
    PacketBad(String),
    PacketSend(String),
    PacketReceive(String),
    UnknownEnumCast,
    BadGame(String)
}

impl fmt::Display for GDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GDError::PacketOverflow(details) => write!(f, "Packet overflow: {details}"),
            GDError::PacketUnderflow(details) => write!(f, "Packet underflow: {details}"),
            GDError::PacketBad(details) => write!(f, "Packet bad: {details}"),
            GDError::PacketSend(details) => write!(f, "Couldn't send a packet: {details}"),
            GDError::PacketReceive(details) => write!(f, "Couldn't receive a packet: {details}"),
            GDError::UnknownEnumCast => write!(f, "Unknown enum cast encountered."),
            GDError::BadGame(details) => write!(f, "Queried another game that the supposed one: {details}"),
        }
    }
}
