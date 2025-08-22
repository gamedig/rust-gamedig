use std::{collections::HashMap, time::Duration};

#[derive(Debug, Clone)]
pub enum GenericPlayerDataValue {
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
}

#[derive(Debug, Clone)]
pub struct GenericPlayer {
    pub name: String,
    pub data: Option<HashMap<String, GenericPlayerDataValue>>,
}

pub trait IntoGenericPlayer: Sized {
    fn into_generic_player(&self) -> GenericPlayer;
}
