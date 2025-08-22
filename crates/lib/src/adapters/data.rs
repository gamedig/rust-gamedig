use std::{collections::HashMap, net::SocketAddr, time::Duration};

/// Represents a generic value.
///
/// This enum covers a wide range of primitive types, strings, and durations,
/// allowing flexible storage of different types of data.
#[derive(Debug, Clone)]
pub enum GenericDataValue {
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

pub type GenericDataHashMap = HashMap<String, GenericDataValue>;
