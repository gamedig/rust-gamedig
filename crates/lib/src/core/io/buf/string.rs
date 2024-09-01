use crate::error::Result;

// TODO: handle errors
impl super::Buffer {
    /// Reads a UTF-8 string from the buffer until a delimiter is encountered.
    ///
    /// # Parameters
    /// - `delimiter`: An optional byte value that signifies the end of the
    ///   string. If `None` is provided, the default delimiter is `0x00`.
    ///
    /// - `strict`: A flag for if the conversion should be strict or lossy.
    ///
    ///   - `true`: Converts the string using `String::from_utf8`, returning an
    ///     error if the sequence is not valid UTF-8.
    ///
    ///   - `false`: Converts the string using `String::from_utf8_lossy`,
    ///     replacing invalid UTF-8 sequences with `�`.
    pub(crate) fn read_string_utf8(
        &mut self,
        delimiter: Option<u8>,
        strict: bool,
    ) -> Result<String> {
        let slice = &self.inner[self.pos ..];
        let delimiter = delimiter.unwrap_or(0x00);

        let end_pos = slice
            .iter()
            .position(|&b| b == delimiter)
            .unwrap_or(slice.len());

        let s = match strict {
            true => String::from_utf8(slice[.. end_pos].to_vec()).unwrap(),
            false => String::from_utf8_lossy(&slice[.. end_pos]).into_owned(),
        };

        self.pos += end_pos + (if end_pos < slice.len() { 1 } else { 0 });

        Ok(s)
    }

    /// Reads a length-prefixed UTF-8 string from the buffer.
    ///
    /// # Parameters
    /// - `strict`: A flag for if the conversion should be strict or lossy.
    ///
    ///   - `true`: Converts the string using `String::from_utf8`, returning an
    ///     error if the sequence is not valid UTF-8.
    ///
    ///   - `false`: Converts the string using `String::from_utf8_lossy`,
    ///     replacing invalid UTF-8 sequences with `�`.
    pub(crate) fn read_string_utf8_len_prefixed(&mut self, strict: bool) -> Result<String> {
        let len = self.inner[self.pos] as usize;
        self.pos += 1;

        let end_pos = self.pos + len;

        let s = match strict {
            true => String::from_utf8(self.inner[self.pos .. end_pos].to_vec()).unwrap(),
            false => String::from_utf8_lossy(&self.inner[self.pos .. end_pos]).into_owned(),
        };

        self.pos = end_pos;

        Ok(s)
    }

    fn _read_string_utf16<F>(
        &mut self,
        delimiter: Option<[u8; 2]>,
        read_u16_e: F,
        strict: bool,
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
        vec.extend(
            (0 .. end_pos / 2)
                .map(|_| read_u16_e(self))
                .collect::<Result<Vec<u16>>>()?,
        );

        let s = match strict {
            true => String::from_utf16(&vec).unwrap(),
            false => String::from_utf16_lossy(&vec),
        };

        self.pos += end_pos + delimiter.len();

        Ok(s)
    }

    /// Reads a BE UTF-16 string from the buffer.
    ///
    /// # Parameters
    /// - `delimiter`: An optional 2-byte value that signifies the end of the
    ///   string. If `None` is provided, the default delimiter is `[0x00,
    ///   0x00]`.
    ///
    /// - `strict`: A flag for if the conversion should be strict or lossy.
    ///
    ///   - `true`: Converts the string using `String::from_utf16`, returning an
    ///     error if the sequence is not valid UTF-16.
    ///
    ///   - `false`: Converts the string using `String::from_utf16_lossy`,
    ///     replacing invalid UTF-16 sequences with `�`.
    pub(crate) fn read_string_utf16_be(
        &mut self,
        delimiter: Option<[u8; 2]>,
        strict: bool,
    ) -> Result<String> {
        self._read_string_utf16(delimiter, |b| b.read_u16_be(), strict)
    }

    /// Reads a LE UTF-16 string from the buffer.
    ///
    /// # Parameters
    /// - `delimiter`: An optional 2-byte value that signifies the end of the
    ///   string. If `None` is provided, the default delimiter is `[0x00,
    ///   0x00]`.
    ///
    /// - `strict`: A flag for if the conversion should be strict or lossy.
    ///
    ///   - `true`: Converts the string using `String::from_utf16`, returning an
    ///     error if the sequence is not valid UTF-16.
    ///
    ///   - `false`: Converts the string using `String::from_utf16_lossy`,
    ///     replacing invalid UTF-16 sequences with `�`.
    pub(crate) fn read_string_utf16_le(
        &mut self,
        delimiter: Option<[u8; 2]>,
        strict: bool,
    ) -> Result<String> {
        self._read_string_utf16(delimiter, |b| b.read_u16_le(), strict)
    }

    /// Reads a UCS-2 encoded string from the buffer.
    ///
    /// This function is essentially a wrapper around the `read_string_utf16_le`
    /// function, as UCS-2 is a subset of UTF-16 that doesn't include
    /// surrogate pairs.
    ///
    /// # Parameters
    /// - `delimiter`: An optional 2-byte value that signifies the end of the
    ///   string. If `None` is provided, the default delimiter is `[0x00,
    ///   0x00]`.
    ///
    /// - `strict`: A flag for if the conversion should be strict or lossy.
    ///
    ///   - `true`: Converts the string using strict UTF-16 decoding, returning
    ///     an error if the sequence is not valid UTF-16.
    ///
    ///   - `false`: Converts the string using lossy UTF-16 decoding, replacing
    ///     invalid sequences with `�`.
    pub(crate) fn read_string_ucs2(
        &mut self,
        delimiter: Option<[u8; 2]>,
        strict: bool,
    ) -> Result<String> {
        self.read_string_utf16_le(delimiter, strict)
    }

    /// Reads a Latin-1 encoded string from the buffer.
    ///
    /// This function uses the `encoding_rs` crate to decode the Latin-1
    /// (or ISO-8859-1 / Windows-1252) encoded byte slice into a UTF-8 string.
    ///
    /// # Parameters
    /// - `delimiter`: An optional byte value that signifies the end of the
    ///   string. If `None` is provided, the default delimiter is `0x00`.
    #[cfg(feature = "_BUFFER_READ_LATIN_1")]
    pub(crate) fn read_string_latin1(&mut self, delimiter: Option<u8>) -> Result<String> {
        let slice = &self.inner[self.pos ..];
        let delimiter = delimiter.unwrap_or(0x00);

        let end_pos = slice
            .iter()
            .position(|&b| b == delimiter)
            .unwrap_or(slice.len());

        let (decoded_string, _, had_errors) = encoding_rs::WINDOWS_1252.decode(&slice[.. end_pos]);

        if had_errors {
            unimplemented!("Error handling");
        }

        self.pos += end_pos + (if end_pos < slice.len() { 1 } else { 0 });

        Ok(decoded_string.into_owned())
    }
}
