use crate::error::Result;

impl<B: super::Bufferable> super::Buffer<B> {
    /// A helper method to read a fixed size slice of bytes from the current position,
    /// convert it into a specific type `T`, and advance the cursor by `N` bytes.
    ///
    /// This method:
    /// 1. Checks that at least `N` bytes are available from the current position.
    /// 2. Copies those `N` bytes into a fixed size array.
    /// 3. Passes the array to the provided conversion closure (`convert`), which returns the final value of type `T`.
    /// 4. Advances the cursor by `N` bytes.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type of the value to be returned after conversion.
    /// * `N`: The number of bytes to read from the buffer.
    ///
    /// # Arguments
    ///
    /// * `convert` - A closure that takes the `N` byte array and converts it into type `T`.
    ///
    /// # Errors
    ///
    /// Returns an error if there are not enough bytes remaining to read `N` bytes starting from
    /// the current cursor position.
    fn _get_inner_slice<T, const N: usize, F>(&mut self, convert: F) -> Result<T>
    where F: FnOnce([u8; N]) -> T {
        self.check_range(.. N, true)?;

        let mut x = [0u8; N];
        let pos = self.pos();
        x.copy_from_slice(&self.inner.as_ref()[pos .. pos + N]);

        self.cursor += N;

        Ok(convert(x))
    }

    /// Read unsigned 8 bit integer (`u8`) from the buffer.
    ///
    /// Advances the cursor by `1` byte.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `1` byte is available at the current cursor position.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_u8(&mut self) -> Result<u8> { self._get_inner_slice::<u8, 1, _>(|x| x[0]) }

    /// Read signed 8 bit integer (`i8`) from the buffer.
    ///
    /// Advances the cursor by `1` byte.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `1` byte is available at the current cursor position.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_i8(&mut self) -> Result<i8> {
        self._get_inner_slice::<i8, 1, _>(|x| x[0] as i8)
    }

    /// Read unsigned 16 bit integer (`u16`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `2` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `2` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_u16_be(&mut self) -> Result<u16> {
        self._get_inner_slice::<u16, 2, _>(u16::from_be_bytes)
    }

    /// Read unsigned 16 bit integer (`u16`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `2` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `2` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_u16_le(&mut self) -> Result<u16> {
        self._get_inner_slice::<u16, 2, _>(u16::from_le_bytes)
    }

    /// Read signed 16 bit integer (`i16`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `2` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `2` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_i16_be(&mut self) -> Result<i16> {
        self._get_inner_slice::<i16, 2, _>(i16::from_be_bytes)
    }

    /// Read signed 16 bit integer (`i16`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `2` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `2` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_i16_le(&mut self) -> Result<i16> {
        self._get_inner_slice::<i16, 2, _>(i16::from_le_bytes)
    }

    /// Read unsigned 32 bit integer (`u32`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_u32_be(&mut self) -> Result<u32> {
        self._get_inner_slice::<u32, 4, _>(u32::from_be_bytes)
    }

    /// Read unsigned 32 bit integer (`u32`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_u32_le(&mut self) -> Result<u32> {
        self._get_inner_slice::<u32, 4, _>(u32::from_le_bytes)
    }

    /// Read signed 32 bit integer (`i32`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_i32_be(&mut self) -> Result<i32> {
        self._get_inner_slice::<i32, 4, _>(i32::from_be_bytes)
    }

    /// Read signed 32 bit integer (`i32`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_i32_le(&mut self) -> Result<i32> {
        self._get_inner_slice::<i32, 4, _>(i32::from_le_bytes)
    }

    /// Read unsigned 64 bit integer (`u64`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_u64_be(&mut self) -> Result<u64> {
        self._get_inner_slice::<u64, 8, _>(u64::from_be_bytes)
    }

    /// Read unsigned 64 bit integer (`u64`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_u64_le(&mut self) -> Result<u64> {
        self._get_inner_slice::<u64, 8, _>(u64::from_le_bytes)
    }

    /// Read signed 64 bit integer (`i64`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_i64_be(&mut self) -> Result<i64> {
        self._get_inner_slice::<i64, 8, _>(i64::from_be_bytes)
    }

    /// Read signed 64 bit integer (`i64`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_i64_le(&mut self) -> Result<i64> {
        self._get_inner_slice::<i64, 8, _>(i64::from_le_bytes)
    }

    /// Read unsigned 128 bit integer (`u128`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `16` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `16` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_u128_be(&mut self) -> Result<u128> {
        self._get_inner_slice::<u128, 16, _>(u128::from_be_bytes)
    }

    /// Read unsigned 128 bit integer (`u128`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `16` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `16` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_u128_le(&mut self) -> Result<u128> {
        self._get_inner_slice::<u128, 16, _>(u128::from_le_bytes)
    }

    /// Read signed 128 bit integer (`i128`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `16` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `16` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_i128_be(&mut self) -> Result<i128> {
        self._get_inner_slice::<i128, 16, _>(i128::from_be_bytes)
    }

    /// Read signed 128 bit integer (`i128`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `16` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `16` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_i128_le(&mut self) -> Result<i128> {
        self._get_inner_slice::<i128, 16, _>(i128::from_le_bytes)
    }

    /// Read 32 bit floating point number (`f32`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_f32_be(&mut self) -> Result<f32> {
        self._get_inner_slice::<f32, 4, _>(f32::from_be_bytes)
    }

    /// Read 32 bit floating point number (`f32`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_f32_le(&mut self) -> Result<f32> {
        self._get_inner_slice::<f32, 4, _>(f32::from_le_bytes)
    }

    /// Read 64 bit floating point number (`f64`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_f64_be(&mut self) -> Result<f64> {
        self._get_inner_slice::<f64, 8, _>(f64::from_be_bytes)
    }

    /// Read 64 bit floating point number (`f64`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn read_f64_le(&mut self) -> Result<f64> {
        self._get_inner_slice::<f64, 8, _>(f64::from_le_bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Buffer;

    #[test]
    fn test_read_u8_ok() {
        let mut buffer = Buffer::new(vec![0x12]);

        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.read_u8().unwrap(), 0x12);
        assert_eq!(buffer.pos(), 1);
    }

    #[test]
    #[should_panic]
    fn test_read_u8_err() {
        let mut buffer = Buffer::new(vec![]);

        buffer.read_u8().unwrap();
    }

    #[test]
    fn test_read_i8_ok() {
        let mut buffer = Buffer::new(vec![0xFE]);

        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.read_i8().unwrap(), -2);
        assert_eq!(buffer.pos(), 1);
    }

    #[test]
    #[should_panic]
    fn test_read_i8_err() {
        let mut buffer = Buffer::new(vec![]);

        buffer.read_i8().unwrap();
    }

    #[test]
    fn test_read_u16_be_ok() {
        let mut buffer = Buffer::new(vec![0x12, 0x34]);

        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.read_u16_be().unwrap(), 0x1234);
        assert_eq!(buffer.pos(), 2);
    }

    #[test]
    #[should_panic]
    fn test_read_u16_be_err() {
        let mut buffer = Buffer::new(vec![0x12]);

        buffer.read_u16_be().unwrap();
    }

    #[test]
    fn test_read_u16_le_ok() {
        let mut buffer = Buffer::new(vec![0x34, 0x12]);

        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.read_u16_le().unwrap(), 0x1234);
        assert_eq!(buffer.pos(), 2);
    }

    #[test]
    #[should_panic]
    fn test_read_u16_le_err() {
        let mut buffer = Buffer::new(vec![0x34]);

        buffer.read_u16_le().unwrap();
    }

    #[test]
    fn test_read_i16_be_ok() {
        let mut buffer = Buffer::new(vec![0xFF, 0xFE]);

        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.read_i16_be().unwrap(), -2);
        assert_eq!(buffer.pos(), 2);
    }

    #[test]
    #[should_panic]
    fn test_read_i16_be_err() {
        let mut buffer = Buffer::new(vec![0xFF]);

        buffer.read_i16_be().unwrap();
    }

    #[test]
    fn test_read_i16_le_ok() {
        let mut buffer = Buffer::new(vec![0xFE, 0xFF]);

        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.read_i16_le().unwrap(), -2);
        assert_eq!(buffer.pos(), 2);
    }

    #[test]
    #[should_panic]
    fn test_read_i16_le_err() {
        let mut buffer = Buffer::new(vec![0xFE]);

        buffer.read_i16_le().unwrap();
    }

    #[test]
    fn test_read_u32_be_ok() {
        let mut buffer = Buffer::new(vec![0x12, 0x34, 0x56, 0x78]);

        assert_eq!(buffer.len(), 4);
        assert_eq!(buffer.read_u32_be().unwrap(), 0x12345678);
        assert_eq!(buffer.pos(), 4);
    }

    #[test]
    #[should_panic]
    fn test_read_u32_be_err() {
        let mut buffer = Buffer::new(vec![0x12, 0x34]);

        buffer.read_u32_be().unwrap();
    }

    #[test]
    fn test_read_u32_le_ok() {
        let mut buffer = Buffer::new(vec![0x78, 0x56, 0x34, 0x12]);

        assert_eq!(buffer.len(), 4);
        assert_eq!(buffer.read_u32_le().unwrap(), 0x12345678);
        assert_eq!(buffer.pos(), 4);
    }

    #[test]
    #[should_panic]
    fn test_read_u32_le_err() {
        let mut buffer = Buffer::new(vec![0x78, 0x56]);

        buffer.read_u32_le().unwrap();
    }

    #[test]
    fn test_read_i32_be_ok() {
        let mut buffer = Buffer::new(vec![0xFF, 0xFF, 0xFF, 0xFE]);

        assert_eq!(buffer.len(), 4);
        assert_eq!(buffer.read_i32_be().unwrap(), -2);
        assert_eq!(buffer.pos(), 4);
    }

    #[test]
    #[should_panic]
    fn test_read_i32_be_err() {
        let mut buffer = Buffer::new(vec![0xFF, 0xFF]);

        buffer.read_i32_be().unwrap();
    }

    #[test]
    fn test_read_i32_le_ok() {
        let mut buffer = Buffer::new(vec![0xFE, 0xFF, 0xFF, 0xFF]);

        assert_eq!(buffer.len(), 4);
        assert_eq!(buffer.read_i32_le().unwrap(), -2);
        assert_eq!(buffer.pos(), 4);
    }

    #[test]
    #[should_panic]
    fn test_read_i32_le_err() {
        let mut buffer = Buffer::new(vec![0xFE, 0xFF]);

        buffer.read_i32_le().unwrap();
    }

    #[test]
    fn test_read_u64_be_ok() {
        let mut buffer = Buffer::new(vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);

        assert_eq!(buffer.len(), 8);
        assert_eq!(buffer.read_u64_be().unwrap(), 0x123456789ABCDEF0);
        assert_eq!(buffer.pos(), 8);
    }

    #[test]
    #[should_panic]
    fn test_read_u64_be_err() {
        let mut buffer = Buffer::new(vec![0x12, 0x34, 0x56]);

        buffer.read_u64_be().unwrap();
    }

    #[test]
    fn test_read_u64_le_ok() {
        let mut buffer = Buffer::new(vec![0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]);

        assert_eq!(buffer.len(), 8);
        assert_eq!(buffer.read_u64_le().unwrap(), 0x123456789ABCDEF0);
        assert_eq!(buffer.pos(), 8);
    }

    #[test]
    #[should_panic]
    fn test_read_u64_le_err() {
        let mut buffer = Buffer::new(vec![0xF0, 0xDE, 0xBC]);

        buffer.read_u64_le().unwrap();
    }

    #[test]
    fn test_read_i64_be_ok() {
        let mut buffer = Buffer::new(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE]);

        assert_eq!(buffer.len(), 8);
        assert_eq!(buffer.read_i64_be().unwrap(), -2);
        assert_eq!(buffer.pos(), 8);
    }

    #[test]
    #[should_panic]
    fn test_read_i64_be_err() {
        let mut buffer = Buffer::new(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

        buffer.read_i64_be().unwrap();
    }

    #[test]
    fn test_read_i64_le_ok() {
        let mut buffer = Buffer::new(vec![0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

        assert_eq!(buffer.len(), 8);
        assert_eq!(buffer.read_i64_le().unwrap(), -2);
        assert_eq!(buffer.pos(), 8);
    }

    #[test]
    #[should_panic]
    fn test_read_i64_le_err() {
        let mut buffer = Buffer::new(vec![0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

        buffer.read_i64_le().unwrap();
    }

    #[test]
    fn test_read_u128_be_ok() {
        let mut buffer = Buffer::new(vec![
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC,
            0xDE, 0xF0,
        ]);

        assert_eq!(buffer.len(), 16);
        assert_eq!(
            buffer.read_u128_be().unwrap(),
            0x123456789ABCDEF0123456789ABCDEF0
        );
        assert_eq!(buffer.pos(), 16);
    }

    #[test]
    #[should_panic]
    fn test_read_u128_be_err() {
        let mut buffer = Buffer::new(vec![0x12, 0x34, 0x56, 0x78]);

        buffer.read_u128_be().unwrap();
    }

    #[test]
    fn test_read_u128_le_ok() {
        let mut buffer = Buffer::new(vec![
            0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12, 0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56,
            0x34, 0x12,
        ]);

        assert_eq!(buffer.len(), 16);
        assert_eq!(
            buffer.read_u128_le().unwrap(),
            0x123456789ABCDEF0123456789ABCDEF0
        );
        assert_eq!(buffer.pos(), 16);
    }

    #[test]
    #[should_panic]
    fn test_read_u128_le_err() {
        let mut buffer = Buffer::new(vec![0xF0, 0xDE, 0xBC, 0x9A]);

        buffer.read_u128_le().unwrap();
    }

    #[test]
    fn test_read_i128_be_ok() {
        let mut buffer = Buffer::new(vec![
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFE,
        ]);

        assert_eq!(buffer.len(), 16);
        assert_eq!(buffer.read_i128_be().unwrap(), -2);
        assert_eq!(buffer.pos(), 16);
    }

    #[test]
    #[should_panic]
    fn test_read_i128_be_err() {
        let mut buffer = Buffer::new(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

        buffer.read_i128_be().unwrap();
    }

    #[test]
    fn test_read_i128_le_ok() {
        let mut buffer = Buffer::new(vec![
            0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ]);

        assert_eq!(buffer.len(), 16);
        assert_eq!(buffer.read_i128_le().unwrap(), -2);
        assert_eq!(buffer.pos(), 16);
    }

    #[test]
    #[should_panic]
    fn test_read_i128_le_err() {
        let mut buffer = Buffer::new(vec![0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

        buffer.read_i128_le().unwrap();
    }

    #[test]
    fn test_read_f32_be_ok() {
        let mut buffer = Buffer::new(vec![0x40, 0x49, 0x0F, 0xDB]);

        assert_eq!(buffer.len(), 4);
        assert!((buffer.read_f32_be().unwrap() - 3.1415927).abs() < f32::EPSILON);
        assert_eq!(buffer.pos(), 4);
    }

    #[test]
    #[should_panic]
    fn test_read_f32_be_err() {
        let mut buffer = Buffer::new(vec![0x40, 0x49]);

        buffer.read_f32_be().unwrap();
    }

    #[test]
    fn test_read_f32_le_ok() {
        let mut buffer = Buffer::new(vec![0xDB, 0x0F, 0x49, 0x40]);

        assert_eq!(buffer.len(), 4);
        assert!((buffer.read_f32_le().unwrap() - 3.1415927).abs() < f32::EPSILON);
        assert_eq!(buffer.pos(), 4);
    }

    #[test]
    #[should_panic]
    fn test_read_f32_le_err() {
        let mut buffer = Buffer::new(vec![0xDB, 0x0F]);

        buffer.read_f32_le().unwrap();
    }

    #[test]
    fn test_read_f64_be_ok() {
        let mut buffer = Buffer::new(vec![0x40, 0x09, 0x21, 0xFB, 0x54, 0x44, 0x2D, 0x18]);

        assert_eq!(buffer.len(), 8);
        assert!((buffer.read_f64_be().unwrap() - 3.141592653589793).abs() < f64::EPSILON);
        assert_eq!(buffer.pos(), 8);
    }

    #[test]
    #[should_panic]
    fn test_read_f64_be_err() {
        let mut buffer = Buffer::new(vec![0x40, 0x09, 0x21]);

        buffer.read_f64_be().unwrap();
    }

    #[test]
    fn test_read_f64_le_ok() {
        let mut buffer = Buffer::new(vec![0x18, 0x2D, 0x44, 0x54, 0xFB, 0x21, 0x09, 0x40]);

        assert_eq!(buffer.len(), 8);
        assert!((buffer.read_f64_le().unwrap() - 3.141592653589793).abs() < f64::EPSILON);
        assert_eq!(buffer.pos(), 8);
    }

    #[test]
    #[should_panic]
    fn test_read_f64_le_err() {
        let mut buffer = Buffer::new(vec![0x18, 0x2D, 0x44]);

        buffer.read_f64_le().unwrap();
    }
}
