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
// buffer.read_string::<Utf8Decoder>(None, false);
// buffer.read_string::<Utf16Decoder<LittleEndian>>(None, false);
// }
//
// // Error because the buffer isnt big endian
// buffer.read_string::<Utf16Decoder<BigEndian>>(None, false);
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

            Some(x) if x < 0 || x > self.remaining_length() as isize => {
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
                size,
                self.remaining_length()
            )));
        }

        let bytes = &self.data[self.cursor .. self.cursor + size];

        self.cursor += size;

        T::read_from_buffer(bytes)
    }

    pub(crate) fn read_string<D: StringDecoder<ByteOrder = B>>(
        &mut self,
        until: Option<D::Delimiter>,
        optional: bool,
    ) -> Result<String, BufferError> {
        let delimiter = until.unwrap_or(D::DELIMITER);

        let position = match self.data[self.cursor ..]
            .windows(delimiter.as_ref().len())
            .position(|window| window == delimiter.as_ref())
        {
            Some(pos) => pos,
            None => {
                if optional {
                    self.data.len() - self.cursor
                } else {
                    return Err(BufferError("Delimiter not found".to_string()));
                }
            }
        };

        let mut data_slice = self.data[self.cursor .. self.cursor + position].to_vec();

        if data_slice.len() % 2 != 0 {
            // TODO: This is a bit of a hack, but it works for now.
            // We should probably find a better way to handle this.
            data_slice.push(0);
        }

        let result = D::decode_string(&data_slice)?;

        self.cursor += position + delimiter.as_ref().len();

        Ok(result)
    }
}

pub(crate) trait BufferRead<B: ByteOrder>: Sized {
    fn read_from_buffer(data: &[u8]) -> Result<Self, BufferError>;
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

impl<B: ByteOrder> BufferRead<B> for u8 {
    fn read_from_buffer(data: &[u8]) -> Result<Self, BufferError> {
        data.first()
            .copied()
            .ok_or_else(|| BufferError("Failed to read u8 from buffer".to_string()))
    }
}

impl<B: ByteOrder> BufferRead<B> for i8 {
    fn read_from_buffer(data: &[u8]) -> Result<Self, BufferError> {
        data.first()
            .map(|&b| b as i8)
            .ok_or_else(|| BufferError("Failed to read i8 from buffer".to_string()))
    }
}

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

    fn decode_string(data: &[u8]) -> Result<String, BufferError>;
}

pub(crate) struct Utf8Decoder;

impl StringDecoder for Utf8Decoder {
    type ByteOrder = LittleEndian;
    type Delimiter = [u8; 1];

    const DELIMITER: Self::Delimiter = [0x00];

    fn decode_string(data: &[u8]) -> Result<String, BufferError> {
        String::from_utf8(data.to_vec()).map_err(|_| BufferError("Failed to decode string as UTF-8".to_string()))
    }
}

pub(crate) struct Utf16Decoder<B: ByteOrder> {
    _marker: PhantomData<B>,
}

impl<B: ByteOrder> StringDecoder for Utf16Decoder<B> {
    type ByteOrder = B;
    type Delimiter = [u8; 2];

    const DELIMITER: Self::Delimiter = [0x00, 0x00];

    fn decode_string(data: &[u8]) -> Result<String, BufferError> {
        let paired_buf: Vec<u16> = data.chunks_exact(2).map(B::read_u16).collect();
        String::from_utf16(&paired_buf).map_err(|_| BufferError("Failed to decode string as UTF-16".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::LittleEndian;

    #[test]
    fn test_move_cursor_within_bounds() {
        let data = [1, 2, 3, 4, 5];
        let mut buffer = Buffer::<LittleEndian>::new(&data);

        assert_eq!(buffer.move_cursor(3), Ok(()));
        assert_eq!(buffer.cursor, 3);
    }

    #[test]
    fn test_move_cursor_out_of_bounds() {
        let data = [1, 2, 3, 4, 5];
        let mut buffer = Buffer::<LittleEndian>::new(&data);

        assert!(buffer.move_cursor(10).is_err());
    }

    #[test]
    fn test_read_u8() {
        let data = [1, 2, 3, 4, 5];
        let mut buffer = Buffer::<LittleEndian>::new(&data);

        assert_eq!(buffer.read::<u8>(), Ok(1));
    }

    #[test]
    fn test_read_u16() {
        let data = [1, 2, 3, 4, 5];
        let mut buffer = Buffer::<LittleEndian>::new(&data);

        assert_eq!(buffer.read::<u16>(), Ok(513));
    }

    #[test]
    fn test_read_string_utf8_delimiter_not_found() {
        let data = b"Hello, World!";
        let mut buffer = Buffer::<LittleEndian>::new(data);

        assert!(
            buffer
                .read_string::<Utf8Decoder>(Some([b'?']), false)
                .is_err()
        );
    }

    #[test]
    fn test_read_string_utf8() {
        let data = b"Hello, World!";
        let mut buffer = Buffer::<LittleEndian>::new(data);

        assert_eq!(
            buffer.read_string::<Utf8Decoder>(Some([b'!']), false),
            Ok("Hello, World".to_string()) // remove '!' from the expected result
        );
    }

    #[test]
    fn test_read_string_utf16() {
        // show hello in utf16 null terminated
        let data = [
            0x68, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x00, 0x00,
        ];
        let mut buffer = Buffer::<LittleEndian>::new(&data);

        assert_eq!(
            buffer.read_string::<Utf16Decoder<LittleEndian>>(None, false),
            Ok("hello".to_string())
        );
    }
}
