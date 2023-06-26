// TODO: Comments and docs
//
// Example usage:
//
// mod example {
// use super::*;
// use byteorder::{BigEndian, LittleEndian};
//
// fn example(data: &[u8]) {
// let mut buffer = Buffer::<LittleEndian>::new(data);
//
// buffer.read::<u8>();
// buffer.read::<i8>();
// buffer.read::<u16>();
// buffer.read::<i16>();
// buffer.read::<u32>();
// buffer.read::<i32>();
// buffer.read::<u64>();
// buffer.read::<i64>();
// buffer.read::<f32>();
// buffer.read::<f64>();
// buffer.read_string::<Utf8Decoder>();
// buffer.read_string::<Utf16Decoder<LittleEndian>>();
// }
//
// // Error because the buffer isnt big endian
// buffer.read_string::<Utf16Decoder<BigEndian>>();
// }

use byteorder::{ByteOrder, LittleEndian};
use std::{convert::TryInto, marker::PhantomData};

#[derive(Debug, PartialEq)]
struct BufferError(String);

pub(crate) struct Buffer<'a, B: ByteOrder> {
    data: &'a [u8],
    cursor: usize,
    _marker: PhantomData<B>,
}

impl<'a, B: ByteOrder> Buffer<'a, B> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            cursor: 0,
            _marker: PhantomData,
        }
    }

    pub(crate) fn remaining_length(&self) -> usize { self.data.len() - self.cursor }

    pub(crate) fn move_cursor(&mut self, offset: isize) -> Result<(), BufferError> {
        let new_cursor = (self.cursor as isize).checked_add(offset);

        match new_cursor {
            None => {
                Err(BufferError(
                    "Checked add failed, cursor out of bounds".to_string(),
                ))
            }

            Some(x) if x < 0 || x as usize > self.remaining_length() => {
                Err(BufferError(format!(
                    "Cursor out of bounds, tried to move cursor to {}",
                    x
                )))
            }

            Some(x) => {
                self.cursor = x as usize;
                Ok(())
            }
        }
    }

    pub(crate) fn read<T: Sized + BufferRead<B>>(&mut self) -> Result<T, BufferError> {
        let size = std::mem::size_of::<T>();
        let remaining = self.remaining_length();

        if size > remaining {
            return Err(BufferError(format!(
                "Packet underflow, expected {} bytes, got {}",
                size, remaining
            )));
        }

        let bytes = &self.data[self.cursor .. self.cursor + size];

        self.cursor += size;

        T::read_from_buffer(bytes)
    }

    pub(crate) fn read_string<D: StringDecoder<ByteOrder = B>>(
        &mut self,
        until: Option<D::Delimiter>,
    ) -> Result<String, BufferError> {
        let data_slice = &self.data[self.cursor ..];
        let delimiter = until.unwrap_or(D::DELIMITER);

        let result = D::decode_string(data_slice, &mut self.cursor, delimiter)?;

        Ok(result)
    }
}

pub(crate) trait BufferRead<B: ByteOrder>: Sized {
    fn read_from_buffer(data: &[u8]) -> Result<Self, BufferError>;
}

macro_rules! impl_buffer_read_byte {
    ($type:ty, $map_func:expr) => {
        impl<B: ByteOrder> BufferRead<B> for $type {
            fn read_from_buffer(data: &[u8]) -> Result<Self, BufferError> {
                data.first()
                    .map($map_func)
                    .ok_or_else(|| BufferError(format!("Failed to read {} from buffer", stringify!($type))))
            }
        }
    };
}

macro_rules! impl_buffer_read {
    ($type:ty, $read_func:ident) => {
        impl<B: ByteOrder> BufferRead<B> for $type {
            fn read_from_buffer(data: &[u8]) -> Result<Self, BufferError> {
                let array = data.try_into().map_err(|_| {
                    BufferError(format!(
                        "Failed to convert {} bytes into {}",
                        data.len(),
                        stringify!($type)
                    ))
                })?;

                Ok(B::$read_func(array))
            }
        }
    };
}

impl_buffer_read_byte!(u8, |&b| b);
impl_buffer_read_byte!(i8, |&b| b as i8);

impl_buffer_read!(u16, read_u16);
impl_buffer_read!(i16, read_i16);
impl_buffer_read!(u32, read_u32);
impl_buffer_read!(i32, read_i32);
impl_buffer_read!(u64, read_u64);
impl_buffer_read!(i64, read_i64);
impl_buffer_read!(f32, read_f32);
impl_buffer_read!(f64, read_f64);

pub(crate) trait StringDecoder {
    type ByteOrder: ByteOrder;
    type Delimiter: AsRef<[u8]>;

    const DELIMITER: Self::Delimiter;

    fn decode_string(data: &[u8], cursor: &mut usize, delimiter: Self::Delimiter) -> Result<String, BufferError>;
}

pub(crate) struct Utf8Decoder;

impl StringDecoder for Utf8Decoder {
    type ByteOrder = LittleEndian;
    type Delimiter = [u8; 1];

    const DELIMITER: Self::Delimiter = [0x00];

    fn decode_string(data: &[u8], cursor: &mut usize, delimiter: Self::Delimiter) -> Result<String, BufferError> {
        let position = data
            .iter()
            .position(|&b| b == delimiter.as_ref()[0])
            .unwrap_or(data.len());

        let result = std::str::from_utf8(&data[.. position])
            .map_err(|_| BufferError("Failed to decode string as UTF-8".to_string()))?
            .to_owned();

        *cursor += position + 1;

        Ok(result)
    }
}

pub(crate) struct Utf16Decoder<B: ByteOrder> {
    _marker: PhantomData<B>,
}

impl<B: ByteOrder> StringDecoder for Utf16Decoder<B> {
    type ByteOrder = B;
    type Delimiter = [u8; 2];

    const DELIMITER: Self::Delimiter = [0x00, 0x00];

    fn decode_string(data: &[u8], cursor: &mut usize, delimiter: Self::Delimiter) -> Result<String, BufferError> {
        let position = data
            .chunks_exact(2)
            .position(|chunk| chunk == delimiter.as_ref())
            .map_or(data.len(), |pos| pos * 2);

        let mut paired_buf: Vec<u16> = vec![0; position / 2];

        B::read_u16_into(&data[.. position], &mut paired_buf);

        let result = String::from_utf16(&paired_buf)
            .map_err(|_| BufferError("Failed to decode string as UTF-16".to_string()))?;

        *cursor += position + 2;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::BigEndian;

    #[test]
    fn test_new_buffer() {
        let data: &[u8] = &[1, 2, 3, 4];
        let buffer = Buffer::<LittleEndian>::new(data);

        assert_eq!(buffer.data, data);
        assert_eq!(buffer.cursor, 0);
    }

    #[test]
    fn test_remaining_length() {
        let data: &[u8] = &[1, 2, 3, 4];
        let mut buffer = Buffer::<LittleEndian>::new(data);

        assert_eq!(buffer.remaining_length(), 4);

        buffer.cursor = 2;
        assert_eq!(buffer.remaining_length(), 2);
    }

    #[test]
    fn test_move_cursor() {
        let data: &[u8] = &[1, 2, 3, 4];
        let mut buffer = Buffer::<LittleEndian>::new(data);

        // Test moving forward
        assert!(buffer.move_cursor(2).is_ok());
        assert_eq!(buffer.cursor, 2);

        // Test moving backward
        assert!(buffer.move_cursor(-1).is_ok());
        assert_eq!(buffer.cursor, 1);

        // Test moving beyond data limits
        assert!(buffer.move_cursor(5).is_err());
        assert!(buffer.move_cursor(-2).is_err());
    }

    #[test]
    fn test_buffer_read_u8() {
        let data: &[u8] = &[1, 2, 3, 4];
        let mut buffer = Buffer::<LittleEndian>::new(data);

        let result: Result<u8, _> = buffer.read();
        assert_eq!(result.unwrap(), 1);
        assert_eq!(buffer.cursor, 1);
    }

    #[test]
    fn test_buffer_read_u16() {
        let data: &[u8] = &[1, 2, 3, 4];
        let mut buffer = Buffer::<LittleEndian>::new(data);

        let result: Result<u16, _> = buffer.read();
        assert_eq!(result.unwrap(), 0x0201);
        assert_eq!(buffer.cursor, 2);
    }

    #[test]
    fn test_buffer_read_u16_big_endian() {
        let data: &[u8] = &[1, 2, 3, 4];
        let mut buffer = Buffer::<BigEndian>::new(data);

        let result: Result<u16, _> = buffer.read();
        assert_eq!(result.unwrap(), 0x0102);
        assert_eq!(buffer.cursor, 2);
    }

    #[test]
    fn test_decode_string_utf8() {
        let data: &[u8] = b"Hello\0World\0";
        let mut cursor = 0;
        let delimiter = [0x00];

        let result = Utf8Decoder::decode_string(data, &mut cursor, delimiter);
        assert_eq!(result.unwrap(), "Hello");
        assert_eq!(cursor, 6);
    }

    #[test]
    fn test_decode_string_utf16_le() {
        let data: &[u8] = &[0x48, 0x00, 0x65, 0x00, 0x00, 0x00];
        let mut cursor = 0;
        let delimiter = [0x00, 0x00];

        let result = Utf16Decoder::<LittleEndian>::decode_string(data, &mut cursor, delimiter);
        assert_eq!(result.unwrap(), "He");
        assert_eq!(cursor, 6);
    }

    #[test]
    fn test_decode_string_utf16_be() {
        let data: &[u8] = &[0x00, 0x48, 0x00, 0x65, 0x00, 0x00];
        let mut cursor = 0;
        let delimiter = [0x00, 0x00];

        let result = Utf16Decoder::<BigEndian>::decode_string(data, &mut cursor, delimiter);
        assert_eq!(result.unwrap(), "He");
        assert_eq!(cursor, 6);
    }

    #[test]
    fn test_buffer_underflow_error() {
        let data: &[u8] = &[1, 2];
        let mut buffer = Buffer::<LittleEndian>::new(data);

        let result: Result<u32, _> = buffer.read();
        assert_eq!(
            result.unwrap_err(),
            BufferError("Packet underflow, expected 4 bytes, got 2".to_string())
        );
    }
}
