use {
    super::player::GenericPlayer,
    std::{collections::HashMap, net::SocketAddr, time::Duration},
};

/// Represents a generic value that can be associated with a server.
///
/// This enum covers a wide range of primitive types, strings, and durations,
/// allowing flexible storage of different types of server related data.
#[derive(Debug, Clone)]
pub enum GenericServerDataValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),

    F32(f32),
    F64(f64),

    Bool(bool),

    String(String),

    Duration(Duration),

    SocketAddr(SocketAddr),
}

/// Represents a generic game server with associated metadata and connected players.
#[derive(Debug, Clone)]
pub struct GenericServer {
    pub addr: SocketAddr,
    pub data: Option<HashMap<String, GenericServerDataValue>>,
    pub players: Option<Vec<GenericPlayer>>,
}

/// A trait for converting server structs into a [`GenericServer`].
pub trait IntoGenericServer: Sized {
    fn into_generic_server(&self) -> GenericServer;
}
