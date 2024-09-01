use crate::error::Result;

impl super::Buffer {
    pub(crate) fn read_string_utf8(&mut self, delimiter: Option<[u8; 1]>) -> Result<String> {
        let data = &self.inner[self.pos ..];
        let delimiter = delimiter.unwrap_or([0x00]);

        let end_pos = data
            .iter()
            .position(|&b| b == delimiter[0])
            .unwrap_or(data.len());

        let s = std::str::from_utf8(&data[.. end_pos]).unwrap().to_owned();

        // +1 to skip the delimiter
        self.pos += end_pos + 1;

        Ok(s)
    }

    pub(crate) fn read_string_utf8_length_prefixed(&mut self) -> Result<String> {
        let len = self.inner[self.pos] as usize;

        self.pos += 1;

        let end_pos = self.pos + len;

        let s = std::str::from_utf8(&self.inner[self.pos .. end_pos])
            .unwrap()
            .to_owned();

        self.pos = end_pos;

        Ok(s)
    }

    fn _read_string_utf16<F>(
        &mut self,
        delimiter: Option<[u8; 2]>,
        read_u16_e: F,
    ) -> Result<String>
    where
        F: Fn(&mut Self) -> Result<u16>,
    {
        let data = &self.inner[self.pos ..];
        let delimiter = delimiter.unwrap_or([0x00, 0x00]);

        let end_pos = data
            .chunks_exact(2)
            .position(|chunk| chunk == delimiter)
            .map_or(data.len(), |pos| pos * 2);

        let mut vec = Vec::with_capacity(end_pos / 2);

        for _ in 0 .. end_pos / 2 {
            vec.push(read_u16_e(self).unwrap());
        }

        let s = String::from_utf16(&vec).unwrap();

        // +2 for the delimiter
        self.pos += end_pos + 2;

        Ok(s)
    }

    pub(crate) fn read_string_utf16_be(&mut self, delimiter: Option<[u8; 2]>) -> Result<String> {
        self._read_string_utf16(delimiter, |b| b.read_u16_be())
    }

    pub(crate) fn read_string_utf16_le(&mut self, delimiter: Option<[u8; 2]>) -> Result<String> {
        self._read_string_utf16(delimiter, |b| b.read_u16_le())
    }
}
