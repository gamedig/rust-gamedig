use std::marker::PhantomData;

use byteorder::ByteOrder;

use crate::errors::GDErrorKind::{InvalidInput, OutOfMemory};
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

    /// Consume the write buffer and return the bytes represented as Vec<u8>.
    pub fn into_data(self) -> Vec<u8> { self.output }

    /// Append a value to the end of the buffer in it's byte represented form.
    pub fn write<T: Sized + BufferWrite<B>>(&mut self, data: T) -> GDResult<()> {
        // Ensure output buffer has capacity for type.
        self.output
            .try_reserve(std::mem::size_of::<T>())
            .map_err(|e| OutOfMemory.context(e))?;

        T::write_to_buffer(&mut self.output, data)
    }

    /// Append a string to the buffer.
    pub fn write_string<E: StringEncoder<B>>(&mut self, string: &str) -> GDResult<()> {
        if let Some(capacity_needed) = E::bytes_required(string) {
            self.output
                .try_reserve(capacity_needed)
                .map_err(|e| OutOfMemory.context(e))?;
        }

        E::encode_string(self, string)
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

pub trait StringEncoder<B: ByteOrder> {
    /// Return the number of bytes required to encode a string. This should not
    /// be an estimate: if unknown return None.
    fn bytes_required(_string: &str) -> Option<usize> { None }

    /// Encode a string.
    fn encode_string(buffer: &mut WriteBuffer<B>, string: &str) -> GDResult<()>;
}

pub struct UTF8NullDelimitedEncoder;
impl<B: ByteOrder> StringEncoder<B> for UTF8NullDelimitedEncoder {
    fn bytes_required(string: &str) -> Option<usize> { Some(string.as_bytes().len() + 1) }

    fn encode_string(buffer: &mut WriteBuffer<B>, string: &str) -> GDResult<()> {
        buffer.write(string.as_bytes())?;
        buffer.write(0u8)?;

        Ok(())
    }
}

pub struct UTF8LengthPrefixedEncoder<LengthWidth> {
    _marker: PhantomData<LengthWidth>,
}
impl<B: ByteOrder, LengthWidth: TryFrom<usize> + BufferWrite<B>> StringEncoder<B>
    for UTF8LengthPrefixedEncoder<LengthWidth>
{
    fn encode_string(buffer: &mut WriteBuffer<B>, string: &str) -> GDResult<()> {
        let length: LengthWidth = (string.len() + 1).try_into().map_err(|_| {
            InvalidInput.context(format!(
                "Tried to encode string that was too long: {}",
                string.len()
            ))
        })?;

        buffer.write(length)?;
        buffer.write(string.as_bytes())?;
        buffer.write(0u8)?;

        Ok(())
    }
}

pub struct UCS2Unreal2Encoder;
impl<B: ByteOrder> StringEncoder<B> for UCS2Unreal2Encoder {
    fn bytes_required(string: &str) -> Option<usize> { Some((string.len() * 2) + 1) }

    fn encode_string(buffer: &mut WriteBuffer<B>, string: &str) -> GDResult<()> {
        let length = string.len();
        if length >= 0x80 {
            return Err(InvalidInput.context(format!(
                "Cannot write strings longer than {}, tried to write {}",
                0x80,
                string.len()
            )));
        }

        let length: u8 = length
            .try_into()
            .expect("Values <= 0x80 should fit in 1 byte");
        let length = length | 0x80;

        buffer.write(length)?;

        // encoding-rs doesn't have a UTF16 encoder
        for byte in string.encode_utf16() {
            buffer.write(byte)?;
        }

        Ok(())
    }
}
