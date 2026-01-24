use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    time::Duration,
};

/// A dynamically typed value for attaching arbitrary metadata.
///
/// This enum can represent a variety of primitive numeric types, booleans,
/// strings, durations, and socket addresses.
///
/// Larger or alignment heavy values are stored on the heap.
#[derive(Debug, Clone)]
pub enum GenericDataValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(Box<u128>),
    Usize(usize),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(Box<i128>),
    Isize(isize),

    F32(f32),
    F64(f64),

    Bool(bool),

    String(Box<String>),
    StringList(Box<Vec<String>>),

    Duration(Box<Duration>),

    IpAddr(Box<IpAddr>),
    SocketAddr(Box<SocketAddr>),
}

impl From<u8> for GenericDataValue {
    fn from(v: u8) -> Self { Self::U8(v) }
}

impl From<u16> for GenericDataValue {
    fn from(v: u16) -> Self { Self::U16(v) }
}

impl From<u32> for GenericDataValue {
    fn from(v: u32) -> Self { Self::U32(v) }
}

impl From<u64> for GenericDataValue {
    fn from(v: u64) -> Self { Self::U64(v) }
}

impl From<u128> for GenericDataValue {
    fn from(v: u128) -> Self { Self::U128(Box::new(v)) }
}

impl From<usize> for GenericDataValue {
    fn from(v: usize) -> Self { Self::Usize(v) }
}

impl From<i8> for GenericDataValue {
    fn from(v: i8) -> Self { Self::I8(v) }
}

impl From<i16> for GenericDataValue {
    fn from(v: i16) -> Self { Self::I16(v) }
}

impl From<i32> for GenericDataValue {
    fn from(v: i32) -> Self { Self::I32(v) }
}

impl From<i64> for GenericDataValue {
    fn from(v: i64) -> Self { Self::I64(v) }
}

impl From<i128> for GenericDataValue {
    fn from(v: i128) -> Self { Self::I128(Box::new(v)) }
}

impl From<isize> for GenericDataValue {
    fn from(v: isize) -> Self { Self::Isize(v) }
}

impl From<f32> for GenericDataValue {
    fn from(v: f32) -> Self { Self::F32(v) }
}

impl From<f64> for GenericDataValue {
    fn from(v: f64) -> Self { Self::F64(v) }
}

impl From<bool> for GenericDataValue {
    fn from(v: bool) -> Self { Self::Bool(v) }
}

impl From<String> for GenericDataValue {
    fn from(s: String) -> Self { Self::String(Box::new(s)) }
}

impl From<&str> for GenericDataValue {
    fn from(s: &str) -> Self { Self::String(Box::new(s.to_owned())) }
}

impl From<Vec<String>> for GenericDataValue {
    fn from(v: Vec<String>) -> Self { Self::StringList(Box::new(v)) }
}

impl From<&[String]> for GenericDataValue {
    fn from(v: &[String]) -> Self { Self::StringList(Box::new(v.to_vec())) }
}

impl From<Vec<&str>> for GenericDataValue {
    fn from(v: Vec<&str>) -> Self {
        Self::StringList(Box::new(v.into_iter().map(String::from).collect()))
    }
}

impl From<&[&str]> for GenericDataValue {
    fn from(v: &[&str]) -> Self {
        Self::StringList(Box::new(v.iter().map(|s| (*s).to_owned()).collect()))
    }
}

impl From<Duration> for GenericDataValue {
    fn from(d: Duration) -> Self { Self::Duration(Box::new(d)) }
}

impl From<IpAddr> for GenericDataValue {
    fn from(ip: IpAddr) -> Self { Self::IpAddr(Box::new(ip)) }
}

impl From<SocketAddr> for GenericDataValue {
    fn from(a: SocketAddr) -> Self { Self::SocketAddr(Box::new(a)) }
}

/// A map of arbitrary, optionally present metadata values.
pub type GenericDataMap = HashMap<String, GenericDataValue>;
