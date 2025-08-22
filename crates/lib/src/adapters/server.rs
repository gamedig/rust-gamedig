use {
    super::player::GenericPlayer,
    std::{collections::HashMap, net::SocketAddr, time::Duration},
};

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

#[derive(Debug, Clone)]
pub struct GenericServer {
    pub addr: SocketAddr,
    pub data: Option<HashMap<String, GenericServerDataValue>>,
    pub players: Option<Vec<GenericPlayer>>,
}

pub trait IntoGenericServer: Sized {
    fn into_generic_server(self) -> GenericServer;
}
