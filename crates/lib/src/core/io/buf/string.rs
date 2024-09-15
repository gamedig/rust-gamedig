use crate::error::{
    diagnostic::{FailureReason, HexDump, OpenGitHubIssue},
    ErrorKind,
    IoError,
    Report,
    Result,
};

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
    #[allow(dead_code)]
    pub(crate) fn read_string_utf8(
        &mut self,
        delimiter: Option<[u8; 1]>,
        strict: bool,
    ) -> Result<String> {
        self.check_range(..)?;

        let delimiter = delimiter.unwrap_or([0x00]);

        // Using self.len to represent the length of valid data in the buffer
        let end_pos = self.inner[self.pos .. self.len]
            .iter()
            .position(|&b| b == delimiter[0])
            .unwrap_or(self.len - self.pos);

        let s = match strict {
            false => {
                String::from_utf8_lossy(&self.inner[self.pos .. self.pos + end_pos]).into_owned()
            }
            true => {
                String::from_utf8(self.inner[self.pos .. self.pos + end_pos].to_vec()).map_err(
                    |e| {
                        Report::from(e)
                            .change_context(IoError::StringConversionError {}.into())
                            .attach_printable(FailureReason::new(
                                "Invalid UTF-8 sequence found during string read.",
                            ))
                            .attach_printable(HexDump::new(
                                format!("Current buffer state (pos: {})", self.pos),
                                self.inner.clone(),
                            ))
                            .attach_printable(OpenGitHubIssue())
                    },
                )?
            }
        };

        self.pos += end_pos + delimiter.len();

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
    #[allow(dead_code)]
    pub(crate) fn read_string_utf8_len_prefixed(&mut self, strict: bool) -> Result<String> {
        // Cant use check range here but needs to be checked
        if self.pos > self.len {
            return Err(Report::new(ErrorKind::from(IoError::UnderflowError {
                attempted: 1,
                available: self.len - self.pos,
            }))
            .attach_printable(FailureReason::new(
                "Attempted to read more bytes than available in the buffer.",
            ))
            .attach_printable(HexDump::new(
                format!("Current buffer state (pos: {})", self.pos),
                self.inner.clone(),
            ))
            .attach_printable(OpenGitHubIssue()));
        }

        let len = self.inner[self.pos] as usize;
        self.pos += 1;

        let end_pos = self.pos + len;

        self.check_range(.. end_pos)?;

        let s = match strict {
            false => String::from_utf8_lossy(&self.inner[self.pos .. end_pos]).into_owned(),
            true => {
                String::from_utf8(self.inner[self.pos .. end_pos].to_vec()).map_err(|e| {
                    Report::from(e)
                        .change_context(IoError::StringConversionError {}.into())
                        .attach_printable(FailureReason::new(
                            "Invalid UTF-8 sequence found during string read.",
                        ))
                        .attach_printable(HexDump::new(
                            format!("Current buffer state (pos: {})", self.pos),
                            self.inner.clone(),
                        ))
                        .attach_printable(OpenGitHubIssue())
                })?
            }
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
        let delimiter = delimiter.unwrap_or([0x00, 0x00]);

        self.check_range(..)?;

        let end_pos = self.inner[self.pos .. self.len]
            .chunks_exact(2)
            .position(|chunk| chunk == delimiter)
            .map_or(self.len - self.pos, |pos| pos * 2);

        let mut vec = Vec::with_capacity(end_pos / 2);

        vec.extend(
            (0 .. end_pos / 2)
                .map(|_| read_u16_e(self))
                .collect::<Result<Vec<u16>>>()?,
        );

        let s = match strict {
            false => String::from_utf16_lossy(&vec),
            true => {
                String::from_utf16(&vec).map_err(|e| {
                    Report::from(e)
                        .change_context(IoError::StringConversionError {}.into())
                        .attach_printable(FailureReason::new(
                            "Invalid UTF-16 sequence found during string read.",
                        ))
                        .attach_printable(HexDump::new(
                            format!("Current buffer state (pos: {})", self.pos),
                            self.inner.clone(),
                        ))
                        .attach_printable(OpenGitHubIssue())
                })?
            }
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    #[cfg(feature = "_BUFFER_READ_LATIN_1")]
    pub(crate) fn read_string_latin1(
        &mut self,
        delimiter: Option<[u8; 1]>,
        strict: bool,
    ) -> Result<String> {
        let delimiter = delimiter.unwrap_or([0x00]);

        self.check_range(..)?;

        let end_pos = self.inner[self.pos .. self.len]
            .iter()
            .position(|&b| b == delimiter[0])
            .unwrap_or(self.len - self.pos);

        let (decoded_string, _, had_errors) =
            encoding_rs::WINDOWS_1252.decode(&self.inner[self.pos .. self.pos + end_pos]);

        if had_errors && strict {
            return Err(
                Report::new(ErrorKind::from(IoError::StringConversionError {}))
                    .attach_printable(FailureReason::new(
                        "Invalid Latin-1 sequence found during string read.",
                    ))
                    .attach_printable(HexDump::new(
                        format!("Current buffer state (pos: {})", self.pos),
                        self.inner.clone(),
                    ))
                    .attach_printable(OpenGitHubIssue()),
            );
        }

        self.pos += end_pos + delimiter.len();

        Ok(decoded_string.into_owned())
    }
}
