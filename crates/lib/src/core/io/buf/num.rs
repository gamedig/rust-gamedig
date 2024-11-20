use crate::error::Result;

impl super::Buffer {
    fn _get_inner_slice<T, const N: usize, F>(&mut self, convert: F) -> Result<T>
    where F: FnOnce([u8; N]) -> T {
        self.check_range(.. N)?;

        let mut x = [0u8; N];
        x.copy_from_slice(&self.inner[self.pos .. self.pos + N]);

        self.pos += N;

        Ok(convert(x))
    }

    /// Read `u8`
    #[allow(dead_code)]
    pub(crate) fn read_u8(&mut self) -> Result<u8> { self._get_inner_slice::<u8, 1, _>(|x| x[0]) }

    /// Read `i8`
    #[allow(dead_code)]
    pub(crate) fn read_i8(&mut self) -> Result<i8> {
        self._get_inner_slice::<i8, 1, _>(|x| x[0] as i8)
    }

    /// Read `u16 BE`
    #[allow(dead_code)]
    pub(crate) fn read_u16_be(&mut self) -> Result<u16> {
        self._get_inner_slice::<u16, 2, _>(u16::from_be_bytes)
    }

    /// Read `u16 LE`
    #[allow(dead_code)]
    pub(crate) fn read_u16_le(&mut self) -> Result<u16> {
        self._get_inner_slice::<u16, 2, _>(u16::from_le_bytes)
    }

    /// Read `i16 BE`
    #[allow(dead_code)]
    pub(crate) fn read_i16_be(&mut self) -> Result<i16> {
        self._get_inner_slice::<i16, 2, _>(i16::from_be_bytes)
    }

    /// Read `i16 LE`
    #[allow(dead_code)]
    pub(crate) fn read_i16_le(&mut self) -> Result<i16> {
        self._get_inner_slice::<i16, 2, _>(i16::from_le_bytes)
    }

    /// Read `u32 BE`
    #[allow(dead_code)]
    pub(crate) fn read_u32_be(&mut self) -> Result<u32> {
        self._get_inner_slice::<u32, 4, _>(u32::from_be_bytes)
    }

    /// Read `u32 LE`
    #[allow(dead_code)]
    pub(crate) fn read_u32_le(&mut self) -> Result<u32> {
        self._get_inner_slice::<u32, 4, _>(u32::from_le_bytes)
    }

    /// Read `i32 BE`
    #[allow(dead_code)]
    pub(crate) fn read_i32_be(&mut self) -> Result<i32> {
        self._get_inner_slice::<i32, 4, _>(i32::from_be_bytes)
    }

    /// Read `i32 LE`
    #[allow(dead_code)]
    pub(crate) fn read_i32_le(&mut self) -> Result<i32> {
        self._get_inner_slice::<i32, 4, _>(i32::from_le_bytes)
    }

    /// Read `u64 BE`
    #[allow(dead_code)]
    pub(crate) fn read_u64_be(&mut self) -> Result<u64> {
        self._get_inner_slice::<u64, 8, _>(u64::from_be_bytes)
    }

    /// Read `u64 LE`
    #[allow(dead_code)]
    pub(crate) fn read_u64_le(&mut self) -> Result<u64> {
        self._get_inner_slice::<u64, 8, _>(u64::from_le_bytes)
    }

    /// Read `i64 BE`
    #[allow(dead_code)]
    pub(crate) fn read_i64_be(&mut self) -> Result<i64> {
        self._get_inner_slice::<i64, 8, _>(i64::from_be_bytes)
    }

    /// Read `i64 LE`
    #[allow(dead_code)]
    pub(crate) fn read_i64_le(&mut self) -> Result<i64> {
        self._get_inner_slice::<i64, 8, _>(i64::from_le_bytes)
    }

    /// Read `u128 BE`
    #[allow(dead_code)]
    pub(crate) fn read_u128_be(&mut self) -> Result<u128> {
        self._get_inner_slice::<u128, 16, _>(u128::from_be_bytes)
    }

    /// Read `u128 LE`
    #[allow(dead_code)]
    pub(crate) fn read_u128_le(&mut self) -> Result<u128> {
        self._get_inner_slice::<u128, 16, _>(u128::from_le_bytes)
    }

    /// Read `i128 BE`
    #[allow(dead_code)]
    pub(crate) fn read_i128_be(&mut self) -> Result<i128> {
        self._get_inner_slice::<i128, 16, _>(i128::from_be_bytes)
    }

    /// Read `i128 LE`
    #[allow(dead_code)]
    pub(crate) fn read_i128_le(&mut self) -> Result<i128> {
        self._get_inner_slice::<i128, 16, _>(i128::from_le_bytes)
    }

    /// Read `f32 BE`
    #[allow(dead_code)]
    pub(crate) fn read_f32_be(&mut self) -> Result<f32> {
        self._get_inner_slice::<f32, 4, _>(f32::from_be_bytes)
    }

    /// Read `f32 LE`
    #[allow(dead_code)]
    pub(crate) fn read_f32_le(&mut self) -> Result<f32> {
        self._get_inner_slice::<f32, 4, _>(f32::from_le_bytes)
    }

    /// Read `f64 BE`
    #[allow(dead_code)]
    pub(crate) fn read_f64_be(&mut self) -> Result<f64> {
        self._get_inner_slice::<f64, 8, _>(f64::from_be_bytes)
    }

    /// Read `f64 LE`
    #[allow(dead_code)]
    pub(crate) fn read_f64_le(&mut self) -> Result<f64> {
        self._get_inner_slice::<f64, 8, _>(f64::from_le_bytes)
    }
}
