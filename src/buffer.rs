use crate::GDError::{PacketBad, PacketUnderflow};
use crate::GDResult;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::{convert::TryInto, marker::PhantomData};

/// A struct representing a buffer with a specific byte order.
///
/// It's comprised of a byte slice that it reads from, a cursor to keep track of
/// the current position within the byte slice, and a `PhantomData` marker to
/// bind it to a specific byte order (BigEndian or LittleEndian).
///
/// The byte order is defined by the `B: ByteOrder` generic parameter.
pub(crate) struct Buffer<'a, B: ByteOrder> {
    /// The byte slice that the buffer reads from.
    data: &'a [u8],
    /// The cursor marking our current position in the buffer.
    cursor: usize,
    /// A phantom field used to bind the `Buffer` to a specific `ByteOrder`.
    _marker: PhantomData<B>,
}

impl<'a, B: ByteOrder> Buffer<'a, B> {
    /// Creates and returns a new `Buffer` with the given data.
    ///
    /// The cursor is set to the start of the buffer (position 0) upon
    /// initialization.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice that the buffer will read from.
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            cursor: 0,
            _marker: PhantomData,
        }
    }

    pub(crate) fn current_position(&self) -> usize { self.cursor }

    /// Returns the length of the remaining bytes from the current cursor
    /// position.
    pub(crate) fn remaining_length(&self) -> usize { self.data.len() - self.cursor }

    /// Returns the length of the buffer data.
    pub(crate) fn data_length(&self) -> usize { self.data.len() }

    // Added for legacy support just for the refactoring
    // Not Tested
    pub(crate) fn remaining_bytes(&self) -> &[u8] { &self.data[self.cursor ..] }

    /// Moves the cursor forward or backward by a specified offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The amount to move the cursor. Use a negative value to move
    ///   backwards.
    ///
    /// # Errors
    ///
    /// Returns a `BufferError` if the attempted move would position the cursor
    /// out of bounds.
    pub(crate) fn move_cursor(&mut self, offset: isize) -> GDResult<()> {
        // Compute the new cursor position by adding the offset to the current cursor
        // position. The checked_add method is used for safe addition,
        // preventing overflow and underflow.
        let new_cursor = (self.cursor as isize).checked_add(offset);

        match new_cursor {
            // If the addition was not successful (i.e., it resulted in an overflow or underflow),
            // return an error indicating that the cursor is out of bounds.
            None => Err(PacketBad),

            // If the new cursor position is either less than zero (i.e., before the start of the buffer)
            // or greater than the remaining length of the buffer (i.e., past the end of the buffer),
            // return an error indicating that the cursor is out of bounds.
            Some(x) if x < 0 || x as usize > self.data_length() => Err(PacketBad),

            // If the new cursor position is within the bounds of the buffer, update the cursor
            // position and return Ok.
            Some(x) => {
                self.cursor = x as usize;
                Ok(())
            }
        }
    }

    /// Reads a value of type `T` from the buffer, and advances the cursor by
    /// the size of `T`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of value to be read from the buffer. This type must
    ///   implement the `BufferRead` trait with the same byte order as the
    ///   buffer.
    ///
    /// # Errors
    ///
    /// Returns a `BufferError` if there is not enough data remaining in the
    /// buffer to read a value of type `T`.
    pub(crate) fn read<T: Sized + BufferRead<B>>(&mut self) -> GDResult<T> {
        // Get the size of `T` in bytes.
        let size = std::mem::size_of::<T>();
        // Calculate remaining length of the buffer.
        let remaining = self.remaining_length();

        // If the size of `T` is larger than the remaining length, return an error
        // because we don't have enough data left to read.
        if size > remaining {
            return Err(PacketUnderflow);
        }

        // Slice the data array from the current cursor position for `size` amount of
        // bytes.
        let bytes = &self.data[self.cursor .. self.cursor + size];

        // Move the cursor forward by `size`.
        self.cursor += size;

        // Use the `read_from_buffer` function of the `BufferRead` implementation for
        // `T` to convert the bytes into an instance of `T`.
        T::read_from_buffer(bytes)
    }

    /// Reads a string from the buffer using a specified `StringDecoder`, until
    /// an optional delimiter.
    ///
    /// # Type Parameters
    ///
    /// * `D` - The type of string decoder to use. This type must implement the
    /// `StringDecoder` trait with the same byte order as the buffer.
    ///
    /// # Arguments
    ///
    /// * `until` - An optional delimiter. If provided, the method will read
    ///   until this
    /// delimiter is encountered. If not provided, the method will read until
    /// the default delimiter of the decoder.
    ///
    /// # Errors
    ///
    /// Returns a `BufferError` if there is an error decoding the string.
    pub(crate) fn read_string<D: StringDecoder>(&mut self, until: Option<D::Delimiter>) -> GDResult<String> {
        // Slice the data array from the current cursor position to the end.
        let data_slice = &self.data[self.cursor ..];

        // Use the provided delimiter if one was given, or default to the
        // delimiter specified by the StringDecoder.
        let delimiter = until.unwrap_or(D::DELIMITER);

        // Invoke the decode_string function of the provided StringDecoder,
        // passing in the remaining data slice, the mutable reference to the
        // cursor, and the delimiter.
        let result = D::decode_string(data_slice, &mut self.cursor, delimiter)?;

        // If decoding was successful, return the decoded string. The cursor
        // position has been updated within the decode_string call to reflect
        // the new position after reading.
        Ok(result)
    }
}

/// A trait that provides an interface to switch endianness.
///
/// The trait `SwitchEndian` is used for types that have a specific
/// byte order (endianness) and can switch to another byte order.
/// The type of the switched endianness is determined by the associated
/// type `Output`.
///
/// The associated type `Output` must implement the `ByteOrder` trait.
pub(crate) trait SwitchEndian {
    type Output: ByteOrder;
}

/// An implementation of `SwitchEndian` for `LittleEndian`.
///
/// The switched endianness type is `BigEndian`.
impl SwitchEndian for LittleEndian {
    type Output = BigEndian;
}

/// An implementation of `SwitchEndian` for `BigEndian`.
///
/// The switched endianness type is `LittleEndian`.
impl SwitchEndian for BigEndian {
    type Output = LittleEndian;
}

impl<'a, B: SwitchEndian + ByteOrder> Buffer<'a, B> {
    /// Switches the byte order of a chunk in the buffer.
    ///
    /// This method consumes the buffer and returns a new buffer
    /// with a chunk of the original buffer's data, starting from the
    /// original cursor position and of the given size, where the byte
    /// order is switched according to the implementation
    /// of `SwitchEndian` for `B`.
    ///
    /// Note: The method also advances the cursor of the original buffer
    /// by `size`.
    ///
    /// # Parameters
    ///
    /// * `size`: The size of the chunk to be taken from the original buffer.
    pub(crate) fn switch_endian_chunk(&mut self, size: usize) -> GDResult<Buffer<'a, B::Output>> {
        let old_cursor = self.cursor;
        self.move_cursor(size as isize)?;

        Ok(Buffer {
            data: &self.data[old_cursor .. old_cursor + size],
            cursor: 0,
            _marker: PhantomData,
        })
    }
}

/// A trait defining a protocol for reading values of a certain type from a
/// buffer.
///
/// Implementors of this trait provide a method for reading their type from a
/// byte buffer with a specific byte order.
pub(crate) trait BufferRead<B: ByteOrder>: Sized {
    fn read_from_buffer(data: &[u8]) -> GDResult<Self>;
}

/// Macro to implement the `BufferRead` trait for byte types.
///
/// This macro generates an implementation of the `BufferRead` trait for a
/// specified byte type. The implementation will read a single byte from the
/// buffer and convert it to the target type using the provided map function.
///
/// # Arguments
///
/// * `$type` - The target type to implement `BufferRead` for.
/// * `$map_func` - The function to map a byte to the target type.
macro_rules! impl_buffer_read_byte {
    ($type:ty, $map_func:expr) => {
        impl<B: ByteOrder> BufferRead<B> for $type {
            fn read_from_buffer(data: &[u8]) -> GDResult<Self> {
                // Use the `first` method to get the first byte from the data array.
                data.first()
                    // Apply the $map_func function to convert the raw byte to the $type.
                    .map($map_func)
                    // If the data array is empty (and thus `first` returns None),
                    // `ok_or_else` will return a BufferError.
                    .ok_or_else(|| PacketBad)
            }
        }
    };
}

/// Macro to implement the `BufferRead` trait for multi-byte types.
///
/// This macro generates an implementation of the `BufferRead` trait for a
/// specified multi-byte type. The implementation will read the appropriate
/// number of bytes from the buffer and convert them to the target type using
/// the provided read function.
///
/// # Arguments
///
/// * `$type` - The target type to implement `BufferRead` for.
/// * `$read_func` - The function to read the bytes into the target type.
macro_rules! impl_buffer_read {
    ($type:ty, $read_func:ident) => {
        impl<B: ByteOrder> BufferRead<B> for $type {
            fn read_from_buffer(data: &[u8]) -> GDResult<Self> {
                // Convert the byte slice into an array of the appropriate type.
                let array = data.try_into().map_err(|_| {
                    // If conversion fails, return an error indicating the required and provided
                    // lengths.
                    PacketBad
                })?;

                // Use the provided function to read the data from the array into the given
                // type.
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

/// A trait defining a protocol for decoding strings from a buffer.
///
/// This trait should be implemented by types that can decode strings from a
/// byte buffer with a specific byte order and delimiter.
pub(crate) trait StringDecoder {
    /// The type of the delimiter used by the decoder.
    type Delimiter: AsRef<[u8]>;

    /// The default delimiter used by the decoder.
    const DELIMITER: Self::Delimiter;

    /// Decodes a string from the provided byte slice, and updates the cursor
    /// position accordingly.
    ///
    /// # Arguments
    ///
    /// * `data` - The byte slice to decode the string from.
    /// * `cursor` - The current position in the byte slice.
    /// * `delimiter` - The delimiter to use for decoding the string.
    ///
    /// # Errors
    ///
    /// Returns a `BufferError` if there is an error decoding the string.
    fn decode_string(data: &[u8], cursor: &mut usize, delimiter: Self::Delimiter) -> GDResult<String>;
}

/// A decoder for UTF-8 encoded strings.
///
/// This decoder uses a single null byte (`0x00`) as the default delimiter.
pub(crate) struct Utf8Decoder;

impl StringDecoder for Utf8Decoder {
    type Delimiter = [u8; 1];

    const DELIMITER: Self::Delimiter = [0x00];

    /// Decodes a UTF-8 string from the given data, updating the cursor position
    /// accordingly.
    fn decode_string(data: &[u8], cursor: &mut usize, delimiter: Self::Delimiter) -> GDResult<String> {
        // Find the position of the delimiter in the data. If the delimiter is not
        // found, the length of the data is returned.
        let position = data
        // Create an iterator over the data.
            .iter()
            // Find the position of the delimiter
            .position(|&b| b == delimiter.as_ref()[0])
            // If the delimiter is not found, use the whole data slice.
            .unwrap_or(data.len());

        // Convert the data until the found position into a UTF-8 string.
        let result = std::str::from_utf8(
            // Take a slice of data until the position.
            &data[.. position]
        )
        // If the data cannot be converted into a UTF-8 string, return an error
            .map_err(|_| PacketBad)?
            // Convert the resulting &str into a String
            .to_owned();

        // Update the cursor position
        // The +1 is to skip the delimiter
        *cursor += position + 1;

        Ok(result)
    }
}

/// A decoder for UTF-16 encoded strings.
///
/// This decoder uses a pair of null bytes (`0x00, 0x00`) as the default
/// delimiter.
///
/// # Type Parameters
///
/// * `B` - The byte order to use when decoding the string.
pub(crate) struct Utf16Decoder<B: ByteOrder> {
    _marker: PhantomData<B>,
}

impl<B: ByteOrder> StringDecoder for Utf16Decoder<B> {
    type Delimiter = [u8; 2];

    const DELIMITER: Self::Delimiter = [0x00, 0x00];

    /// Decodes a UTF-16 string from the given data, updating the cursor
    /// position accordingly.
    fn decode_string(data: &[u8], cursor: &mut usize, delimiter: Self::Delimiter) -> GDResult<String> {
        // Try to find the position of the delimiter in the data
        let position = data
        // Split the data into 2-byte chunks (as UTF-16 uses 2 bytes per character)
            .chunks_exact(2)
            // Find the position of the delimiter
            .position(|chunk| chunk == delimiter.as_ref())
            // If the delimiter is not found, use the whole data, otherwise use the position of the delimiter
            .map_or(data.len(), |pos| pos * 2);

        // Create a buffer of u16 values to hold the decoded characters
        let mut paired_buf: Vec<u16> = vec![0; position / 2];

        // Decode the data into the buffer
        B::read_u16_into(&data[.. position], &mut paired_buf);

        // Convert the buffer of u16 values into a String
        let result = String::from_utf16(&paired_buf).map_err(|_| PacketBad)?;

        // Update the cursor position
        // The +2 accounts for the delimiter
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
    fn test_switch_endian_chunk_le_be() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut buffer = Buffer::<LittleEndian>::new(&data[..]);

        let switched_buffer = buffer.switch_endian_chunk(2).unwrap();

        assert_eq!(switched_buffer.data, [0x01, 0x02]);
        assert_eq!(switched_buffer.cursor, 0);

        assert_eq!(buffer.remaining_bytes(), [0x03, 0x04]);
        assert_eq!(buffer.cursor, 2);
    }

    #[test]
    fn test_switch_endian_chunk_be_le() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut buffer = Buffer::<BigEndian>::new(&data[..]);

        let switched_buffer = buffer.switch_endian_chunk(2).unwrap();

        assert_eq!(switched_buffer.data, [0x01, 0x02]);
        assert_eq!(switched_buffer.cursor, 0);

        assert_eq!(buffer.remaining_bytes(), [0x03, 0x04]);
        assert_eq!(buffer.cursor, 2);
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
        assert_eq!(result.unwrap_err(), PacketUnderflow);
    }
}
