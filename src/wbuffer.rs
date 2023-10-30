use std::marker::PhantomData;

use byteorder::ByteOrder;

use crate::errors::GDErrorKind::PacketBad;
use crate::GDResult;

/// A buffer to write packets to.
#[derive(Clone, Debug, Default)]
pub struct WriteBuffer<B: ByteOrder> {
    output: Vec<u8>,
    _marker: PhantomData<B>,
}

impl<B: ByteOrder> WriteBuffer<B> {
    /// Allocate a new write buffer with the provided capacity (to prevent
    /// re-allocations).
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            output: Vec::with_capacity(capacity),
            _marker: PhantomData,
        }
    }

    /// TODO: Doc comments
    pub fn into_data(self) -> Vec<u8> { self.output }

    /// TODO: Doc comments
    pub fn write<T: Sized + BufferWrite<B>>(&mut self, data: T) -> GDResult<()> {
        T::write_to_buffer(&mut self.output, data)
    }

    /// TODO: Doc comments
    pub fn write_string(&mut self, data: &str) -> GDResult<()> {
        // FIXME: This should use a generic string encoder similar to
        // Buffer::StringDecoder.
        let length = data.len();
        if length >= 0x80 {
            return Err(PacketBad.context(format!(
                "Cannot write strings longer than {}, tried to write {}",
                0x80,
                data.len()
            )));
        }

        let length: u8 = length
            .try_into()
            .expect("Values <= 0x80 should fit in 1 byte");
        let length = length | 0x80;

        self.write(length)?;

        // encoding-rs doesn't have a UTF16 encoder
        for byte in data.encode_utf16() {
            self.write(byte)?;
        }

        Ok(())
    }
}

/// A trait defining a protocol for writing values of a certain type to a
/// buffer.
///
/// Implementors of this trait provide a method for writing their type to a
/// byte buffer with a specific byte order.
pub trait BufferWrite<B: ByteOrder>: Sized {
    fn write_to_buffer(buffer: &mut Vec<u8>, data: Self) -> GDResult<()>;
}

macro_rules! impl_buffer_write_byte {
    ($type:ty, $map_func:expr) => {
        impl<B: ByteOrder> BufferWrite<B> for $type {
            fn write_to_buffer(buffer: &mut Vec<u8>, data: Self) -> GDResult<()> {
                let map = $map_func;
                buffer.push(map(data));

                Ok(())
            }
        }
    };
}

macro_rules! impl_buffer_write {
    ($type:ty, $write_func:ident) => {
        impl<B: ByteOrder> BufferWrite<B> for $type {
            fn write_to_buffer(buffer: &mut Vec<u8>, data: Self) -> GDResult<()> {
                let mut slice = [0u8; std::mem::size_of::<Self>()];

                B::$write_func(&mut slice, data);

                buffer.extend(slice);

                Ok(())
            }
        }
    };
}

impl_buffer_write_byte!(u8, |b| b);
impl_buffer_write_byte!(i8, |b| b as u8);

impl_buffer_write!(u16, write_u16);
impl_buffer_write!(i16, write_i16);
impl_buffer_write!(u32, write_u32);
impl_buffer_write!(i32, write_i32);
impl_buffer_write!(u64, write_u64);
impl_buffer_write!(i64, write_i64);
impl_buffer_write!(f32, write_f32);
impl_buffer_write!(f64, write_f64);

impl<B: ByteOrder> BufferWrite<B> for &[u8] {
    fn write_to_buffer(buffer: &mut Vec<u8>, data: Self) -> GDResult<()> {
        buffer.extend_from_slice(data);

        Ok(())
    }
}
