use crate::error::{ErrorKind, IoError, Report, Result, diagnostic::FailureReason};

// TODO: There are bugs in this, it needs to be tested and fixed.

impl<B: super::Bufferable> super::Buffer<B> {
    /// Reads a `UTF 8` string from the buffer until a delimiter is encountered.
    ///
    /// # Parameters
    ///
    /// - `delimiter`: An optional byte value (`[u8; 1]`) signifying the end of the string.  
    ///   If `None` is provided, the default delimiter is `0x00`.
    ///
    /// - `strict`: Determines how invalid `UTF 8` sequences are handled.
    ///
    ///   - `true`: Uses `String::from_utf8`. If any invalid `UTF 8` sequence is found,
    ///     an error is returned.
    ///
    ///   - `false`: Uses `String::from_utf8_lossy`, replacing invalid sequences with `�`.
    ///
    /// After reading up to (but not including) the delimiter, the cursor is advanced by the number
    /// of bytes read plus the length of the delimiter (usually `1` byte).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range goes out of bounds.
    /// - `strict` is `true` and the bytes do not form valid `UTF 8`.
    #[allow(dead_code)]
    pub(crate) fn read_string_utf8(
        &mut self,
        delimiter: Option<[u8; 1]>,
        strict: bool,
    ) -> Result<String> {
        self.check_range(.., true)?;

        let pos = self.pos();
        let len = self.len();
        let delimiter = delimiter.unwrap_or([0x00]);

        let end_pos = self.inner.as_ref()[pos .. len]
            .iter()
            .position(|&b| b == delimiter[0])
            .unwrap_or(len - pos);

        let s = if strict {
            String::from_utf8(self.inner.as_ref()[pos .. pos + end_pos].to_vec()).map_err(|e| {
                Report::from(e)
                    .change_context(IoError::BufferStringConversionError {}.into())
                    .attach_printable(FailureReason::new(
                        "Invalid UTF 8 sequence found during string read.",
                    ))
            })?
        } else {
            String::from_utf8_lossy(&self.inner.as_ref()[pos .. pos + end_pos]).into_owned()
        };

        self.cursor += end_pos + delimiter.len();

        Ok(s)
    }

    /// Reads a length prefixed `UTF 8` string from the buffer.
    ///
    /// The length prefix is a single `u8` that indicates how many bytes of `UTF 8` data follow.
    /// The method first reads this `u8` length, then reads that many bytes as a `UTF 8` string.
    ///
    /// # Parameters
    ///
    /// - `strict`: Determines how invalid `UTF 8` sequences are handled.
    ///
    ///   - `true`: Uses `String::from_utf8`. Any invalid `UTF 8` sequence causes an error.
    ///
    ///   - `false`: Uses `String::from_utf8_lossy`, replacing invalid sequences with `�`.
    ///
    /// After reading the length byte and the string data, the cursor advances by `1` byte for the length
    /// plus the number of bytes read for the string.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range is out of bounds.
    /// - `strict` is `true` and the bytes do not form valid `UTF 8`.
    #[allow(dead_code)]
    pub(crate) fn read_string_utf8_len_prefixed(&mut self, strict: bool) -> Result<String> {
        self.check_range(.. 1, true)?;

        let pos = self.pos();
        let s_len = self.inner.as_ref()[pos] as usize;
        let end_pos = pos + s_len;
        self.check_range(.. end_pos, true)?;

        let s = if strict {
            String::from_utf8(self.inner.as_ref()[pos + 1 .. end_pos].to_vec()).map_err(|e| {
                Report::from(e)
                    .change_context(IoError::BufferStringConversionError {}.into())
                    .attach_printable(FailureReason::new(
                        "Invalid UTF 8 sequence found during string read.",
                    ))
            })?
        } else {
            String::from_utf8_lossy(&self.inner.as_ref()[pos + 1 .. end_pos]).into_owned()
        };

        self.cursor = end_pos;

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
        let delimiter = delimiter.unwrap_or([0x00, 0x00]);

        self.check_range(.., true)?;

        let pos = self.pos();
        let len = self.len();
        let end_pos = self.inner.as_ref()[pos .. len]
            .chunks_exact(2)
            .position(|chunk| chunk == delimiter)
            .map_or(len - pos, |p| p * 2);

        let mut vec = Vec::with_capacity(end_pos / 2);
        vec.extend(
            (0 .. end_pos / 2)
                .map(|_| read_u16_e(self))
                .collect::<Result<Vec<u16>>>()?,
        );

        let s = if strict {
            String::from_utf16(&vec).map_err(|e| {
                Report::from(e)
                    .change_context(IoError::BufferStringConversionError {}.into())
                    .attach_printable(FailureReason::new(
                        "Invalid UTF 16 sequence found during string read.",
                    ))
            })?
        } else {
            String::from_utf16_lossy(&vec)
        };

        self.cursor += end_pos + delimiter.len();

        Ok(s)
    }

    /// Reads a `UTF 16` string in big endian (`BE`) order until a `2 byte` delimiter is encountered.
    ///
    /// # Parameters
    ///
    /// - `delimiter`: An optional `2 byte` value marking the end of the string.  
    ///   Default is `[0x00, 0x00]` if `None` is provided.
    ///
    /// - `strict`: Determines how invalid `UTF 16` sequences are handled.
    ///
    ///   - `true`: Uses `String::from_utf16`. Any invalid `UTF 16` sequence causes an error.
    ///
    ///   - `false`: Uses `String::from_utf16_lossy`, replacing invalid sequences with `�`.
    ///
    /// After reading up to the delimiter, the cursor advances by the number of bytes read plus the delimiter length (2 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range goes out of bounds.
    /// - `strict` is `true` and invalid `UTF 16` data is encountered.
    #[allow(dead_code)]
    pub(crate) fn read_string_utf16_be(
        &mut self,
        delimiter: Option<[u8; 2]>,
        strict: bool,
    ) -> Result<String> {
        self._read_string_utf16(delimiter, |b| b.read_u16_be(), strict)
    }

    /// Reads a `UTF 16` string in little endian (`LE`) order until a `2 byte` delimiter is encountered.
    ///
    /// # Parameters
    ///
    /// - `delimiter`: An optional `2 byte` value marking the end of the string.  
    ///   Default is `[0x00, 0x00]` if `None` is provided.
    ///
    /// - `strict`: Determines how invalid `UTF 16` sequences are handled.
    ///
    ///   - `true`: Uses `String::from_utf16`. Any invalid `UTF 16` sequence causes an error.
    ///
    ///   - `false`: Uses `String::from_utf16_lossy`, replacing invalid sequences with `�`.
    ///
    /// After reading up to the delimiter, the cursor advances by the number of bytes read plus the delimiter length (2 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range goes out of bounds.
    /// - `strict` is `true` and invalid `UTF 16` data is encountered.
    #[allow(dead_code)]
    pub(crate) fn read_string_utf16_le(
        &mut self,
        delimiter: Option<[u8; 2]>,
        strict: bool,
    ) -> Result<String> {
        self._read_string_utf16(delimiter, |b| b.read_u16_le(), strict)
    }

    /// Reads a `UCS 2` encoded string from the buffer.
    ///
    /// `UCS 2` is essentially `UTF 16` without surrogate pairs. This method delegates to
    /// `read_string_utf16_le` as `UCS 2` data is always handled as `UTF 16 LE`.
    ///
    /// # Parameters
    ///
    /// - `delimiter`: An optional `2 byte` value marking the end of the string.  
    ///   Default is `[0x00, 0x00]` if `None` is provided.
    ///
    /// - `strict`: Determines how invalid sequences are handled.
    ///
    ///   - `true`: Uses strict `UTF 16` decoding, erroring on invalid data.
    ///
    ///   - `false`: Uses lossy decoding, replacing invalid sequences with `�`.
    ///
    /// After reading up to the delimiter, the cursor advances by the number of bytes read plus the delimiter length (2 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range goes out of bounds.
    /// - `strict` is `true` and invalid `UTF 16` data is encountered.
    #[allow(dead_code)]
    pub(crate) fn read_string_ucs2(
        &mut self,
        delimiter: Option<[u8; 2]>,
        strict: bool,
    ) -> Result<String> {
        self.read_string_utf16_le(delimiter, strict)
    }

    /// Reads a `Latin 1` (also known as `ISO 8859 1` or `Windows 1252`) encoded string from the buffer until a delimiter is reached.
    ///
    /// This method uses the `encoding_rs` crate to decode the `Latin 1` bytes into a `UTF 8` string.
    ///
    /// # Parameters
    ///
    /// - `delimiter`: An optional byte (`[u8; 1]`) that marks the end of the string.  
    ///   If `None` is provided, `0x00` is used as the delimiter.
    ///
    /// - `strict`: Determines how invalid sequences are handled.
    ///
    ///   - `true`: Returns an error if decoding encounters invalid sequences.
    ///
    ///   - `false`: Replaces invalid sequences with `�`.
    ///
    /// After reading up to the delimiter, the cursor advances by the number of bytes read plus the delimiter length (usually `1` byte).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range goes out of bounds.
    /// - `strict` is `true` and invalid `Latin 1` data is encountered.
    #[allow(dead_code)]
    #[cfg(feature = "_BUFFER_READ_LATIN_1")]
    pub(crate) fn read_string_latin1(
        &mut self,
        delimiter: Option<[u8; 1]>,
        strict: bool,
    ) -> Result<String> {
        let delimiter = delimiter.unwrap_or([0x00]);

        self.check_range(.., true)?;

        let pos = self.pos();
        let len = self.len();
        let end_pos = self.inner.as_ref()[pos .. len]
            .iter()
            .position(|&b| b == delimiter[0])
            .unwrap_or(len - pos);

        let (decoded_string, _, had_errors) =
            encoding_rs::WINDOWS_1252.decode(&self.inner.as_ref()[pos .. pos + end_pos]);

        if had_errors && strict {
            return Err(
                Report::new(ErrorKind::from(IoError::BufferStringConversionError {}))
                    .attach_printable(FailureReason::new(
                        "Invalid Latin 1 sequence found during string read.",
                    )),
            );
        }

        self.cursor += end_pos + delimiter.len();

        Ok(decoded_string.into_owned())
    }
}
