use crate::error::{bail, ErrorKind, IoError, Result};

impl super::Buffer {
    fn _get_inner_byte<T, const N: usize, F>(&mut self, convert: F) -> Result<T>
    where F: FnOnce([u8; N]) -> T {
        let available = self.inner.len().saturating_sub(self.pos);

        if available < N {
            bail!(ErrorKind::from(IoError::UnderflowError {
                #[cfg(debug_assertions)]
                _pos: self.pos,
                #[cfg(debug_assertions)]
                _raw: self.inner.clone(),
                attempted: N,
                available,
            }));
        }

        let mut x = [0u8; N];
        x.copy_from_slice(&self.inner[self.pos .. self.pos + N]);

        self.pos += N;

        Ok(convert(x))
    }

    /// Read `u8`
    #[allow(dead_code)]
    pub(crate) fn read_u8(&mut self) -> Result<u8> { self._get_inner_byte::<u8, 1, _>(|x| x[0]) }

    /// Read `i8`
    #[allow(dead_code)]
    pub(crate) fn read_i8(&mut self) -> Result<i8> {
        self._get_inner_byte::<i8, 1, _>(|x| x[0] as i8)
    }

    /// Read `u16 BE`
    #[allow(dead_code)]
    pub(crate) fn read_u16(&mut self) -> Result<u16> {
        self._get_inner_byte::<u16, 2, _>(|x| u16::from_be_bytes(x))
    }

    /// Read `u16 LE`
    #[allow(dead_code)]
    pub(crate) fn read_u16_le(&mut self) -> Result<u16> {
        self._get_inner_byte::<u16, 2, _>(|x| u16::from_le_bytes(x))
    }

    /// Read `i16 BE`
    #[allow(dead_code)]
    pub(crate) fn read_i16(&mut self) -> Result<i16> {
        self._get_inner_byte::<i16, 2, _>(|x| i16::from_be_bytes(x))
    }

    /// Read `i16 LE`
    #[allow(dead_code)]
    pub(crate) fn read_i16_le(&mut self) -> Result<i16> {
        self._get_inner_byte::<i16, 2, _>(|x| i16::from_le_bytes(x))
    }

    /// Read `u32 BE`
    #[allow(dead_code)]
    pub(crate) fn read_u32(&mut self) -> Result<u32> {
        self._get_inner_byte::<u32, 4, _>(|x| u32::from_be_bytes(x))
    }

    /// Read `u32 LE`
    #[allow(dead_code)]
    pub(crate) fn read_u32_le(&mut self) -> Result<u32> {
        self._get_inner_byte::<u32, 4, _>(|x| u32::from_le_bytes(x))
    }

    /// Read `i32 BE`
    #[allow(dead_code)]
    pub(crate) fn read_i32(&mut self) -> Result<i32> {
        self._get_inner_byte::<i32, 4, _>(|x| i32::from_be_bytes(x))
    }

    /// Read `i32 LE`
    #[allow(dead_code)]
    pub(crate) fn read_i32_le(&mut self) -> Result<i32> {
        self._get_inner_byte::<i32, 4, _>(|x| i32::from_le_bytes(x))
    }

    /// Read `u64 BE`
    #[allow(dead_code)]
    pub(crate) fn read_u64(&mut self) -> Result<u64> {
        self._get_inner_byte::<u64, 8, _>(|x| u64::from_be_bytes(x))
    }

    /// Read `u64 LE`
    #[allow(dead_code)]
    pub(crate) fn read_u64_le(&mut self) -> Result<u64> {
        self._get_inner_byte::<u64, 8, _>(|x| u64::from_le_bytes(x))
    }

    /// Read `i64 BE`
    #[allow(dead_code)]
    pub(crate) fn read_i64(&mut self) -> Result<i64> {
        self._get_inner_byte::<i64, 8, _>(|x| i64::from_be_bytes(x))
    }

    /// Read `i64 LE`
    #[allow(dead_code)]
    pub(crate) fn read_i64_le(&mut self) -> Result<i64> {
        self._get_inner_byte::<i64, 8, _>(|x| i64::from_le_bytes(x))
    }

    /// Read `u128 BE`
    #[allow(dead_code)]
    pub(crate) fn read_u128(&mut self) -> Result<u128> {
        self._get_inner_byte::<u128, 16, _>(|x| u128::from_be_bytes(x))
    }

    /// Read `u128 LE`
    #[allow(dead_code)]
    pub(crate) fn read_u128_le(&mut self) -> Result<u128> {
        self._get_inner_byte::<u128, 16, _>(|x| u128::from_le_bytes(x))
    }

    /// Read `i128 BE`
    #[allow(dead_code)]
    pub(crate) fn read_i128(&mut self) -> Result<i128> {
        self._get_inner_byte::<i128, 16, _>(|x| i128::from_be_bytes(x))
    }

    /// Read `i128 LE`
    #[allow(dead_code)]
    pub(crate) fn read_i128_le(&mut self) -> Result<i128> {
        self._get_inner_byte::<i128, 16, _>(|x| i128::from_le_bytes(x))
    }

    /// Read `f32 BE`
    #[allow(dead_code)]
    pub(crate) fn read_f32(&mut self) -> Result<f32> {
        self._get_inner_byte::<f32, 4, _>(|x| f32::from_be_bytes(x))
    }

    /// Read `f32 LE`
    #[allow(dead_code)]
    pub(crate) fn read_f32_le(&mut self) -> Result<f32> {
        self._get_inner_byte::<f32, 4, _>(|x| f32::from_le_bytes(x))
    }

    /// Read `f64 BE`
    #[allow(dead_code)]
    pub(crate) fn read_f64(&mut self) -> Result<f64> {
        self._get_inner_byte::<f64, 8, _>(|x| f64::from_be_bytes(x))
    }

    /// Read `f64 LE`
    #[allow(dead_code)]
    pub(crate) fn read_f64_le(&mut self) -> Result<f64> {
        self._get_inner_byte::<f64, 8, _>(|x| f64::from_le_bytes(x))
    }
}
