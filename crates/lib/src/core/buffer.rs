use {
    super::error::{
        Report,
        ResultExt,
        diagnostic::{
            CRATE_INFO,
            ContextComponent,
            FailureReason,
            HexDump,
            OpenGitHubIssue,
            SYSTEM_INFO,
        },
    },

    std::{
        fmt::Debug,
        ops::{Bound, RangeBounds},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum BufferError {
    #[error(
        "[GameDig]::[BUF::OUT_OF_RANGE]: buffer cursor movement is outside the valid buffer range"
    )]
    OutOfRange,

    #[error(
        "[GameDig]::[BUF::NOT_REPRESENTABLE]: buffer cursor movement cannot be represented \
         numerically"
    )]
    NotRepresentable,

    #[error(
        "[GameDig]::[BUF::RANGE_BOUNDS_OVERFLOW]: buffer range validation failed due to non \
         representable bounds"
    )]
    RangeBoundsOverflow,

    #[error(
        "[GameDig]::[BUF::RANGE_BOUNDS_INVALID]: buffer range validation failed due to invalid \
         bound ordering"
    )]
    RangeBoundsInvalid,

    #[error(
        "[GameDig]::[BUF::RANGE_BOUNDS_OUT_OF_BOUNDS]: buffer range validation failed because the \
         range exceeds buffer length"
    )]
    RangeBoundsOutOfBounds,

    #[error("[GameDig]::[BUF::RANGE_CHECK_FAILED]: buffer range check operation failed")]
    RangeCheckFailed,

    #[error(
        "[GameDig]::[BUF::DELIMITER_NOT_FOUND]: expected string delimiter was not found before \
         end of buffer"
    )]
    DelimiterNotFound,

    #[error("[GameDig]::[BUF::INVALID_UTF8_STRING]: buffer contains invalid UTF 8 string data")]
    InvalidUTF8String,

    #[error("[GameDig]::[BUF::INVALID_UTF16_STRING]: buffer contains invalid UTF 16 string data")]
    InvalidUTF16String,

    #[error("[GameDig]::[BUF::INVALID_LATIN1_STRING]: buffer contains invalid Latin 1 string data")]
    InvalidLatin1String,
}

/// The `Bufferable` trait abstracts types that represent byte storage and provides
/// a method to retrieve the length of the underlying storage.
pub(crate) trait Bufferable: Clone + Debug + Send + Sync + 'static + AsRef<[u8]> {
    /// If the underlying storage has a known size at compile time.
    #[cfg(feature = "_DEV_LOG")]
    const IS_FIXED_SIZE: bool;

    /// Returns the number of elements in the underlying byte storage.
    fn len(&self) -> usize;
}

impl Bufferable for Vec<u8> {
    #[cfg(feature = "_DEV_LOG")]
    const IS_FIXED_SIZE: bool = false;

    #[inline]
    fn len(&self) -> usize { self.len() }
}

impl<const N: usize> Bufferable for [u8; N] {
    #[cfg(feature = "_DEV_LOG")]
    const IS_FIXED_SIZE: bool = true;

    #[inline]
    fn len(&self) -> usize { N }
}

/// The `Buffer` struct provides a lightweight, runtime agnostic abstraction for byte storage,
/// whether allocated on the stack or the heap. It ensures safe indexing and supports cursor based
/// read operations, enabling efficient in-memory data access without depending on a specific runtime I/O.
pub(crate) struct Buffer<B: Bufferable> {
    /// The underlying byte storage.
    inner: B,
    /// The current position in the buffer.
    cursor: usize,
}

impl<B: Bufferable> Buffer<B> {
    /// Creates a new `Buffer` from a provided byte storage.
    ///
    /// # Arguments
    ///
    /// * `inner` - The underlying byte storage.
    #[inline]
    pub(crate) fn new(inner: B) -> Self {
        dev_trace_fmt!("GAMEDIG::CORE::BUFFER::<NEW>: {:?}", |f| {
            f.debug_struct("Args")
                .field(
                    "inner",
                    format_args!("alloc({})", if B::IS_FIXED_SIZE { "stack" } else { "heap" }),
                )
                .finish()
        });

        Self { inner, cursor: 0 }
    }

    /// Returns the current position in the buffer
    ///
    /// The position is zero based and increments as you read or move through the buffer.
    #[inline]
    pub(crate) fn pos(&self) -> usize {
        dev_trace!("GAMEDIG::CORE::BUFFER::<POS>");

        self.cursor
    }

    /// Returns the number of elements in the underlying byte storage.
    #[inline]
    pub(crate) fn len(&self) -> usize {
        dev_trace!("GAMEDIG::CORE::BUFFER::<LEN>");

        self.inner.len()
    }

    /// Returns the number of elements remaining from the current position to the end of the byte storage.
    ///
    /// This gives you how many more bytes can be read without going out of bounds.
    #[inline]
    pub(crate) fn remaining(&self) -> usize {
        dev_trace!("GAMEDIG::CORE::BUFFER::<REMAINING>");

        self.len().saturating_sub(self.pos())
    }

    /// Checks if there are no remaining bytes to read from the current position.
    pub(crate) fn is_empty(&self) -> bool {
        dev_trace!("GAMEDIG::CORE::BUFFER::<IS_EMPTY>");

        self.remaining() == 0
    }

    /// Returns a slice of the remaining unread bytes.
    pub(crate) fn remaining_slice(&self) -> &[u8] {
        dev_trace!("GAMEDIG::CORE::BUFFER::<REMAINING_SLICE>");

        &self.inner.as_ref()[self.pos().min(self.len()) ..]
    }

    /// Consumes the `Buffer` and returns the underlying byte storage.
    ///
    /// This conversion moves the underlying byte storage out of the `Buffer`,
    /// effectively discarding the `Buffer` wrapper.
    #[inline]
    pub(crate) fn unpack(self) -> B {
        dev_trace!("GAMEDIG::CORE::BUFFER::<UNPACK>");

        self.inner
    }

    /// Moves the buffer’s current position by the specified amount.
    ///
    /// Negative values move the position backward, and positive values move it forward.
    /// The new position must remain within the buffer’s bounds.
    ///
    /// # Arguments
    ///
    /// * `off` - The signed offset to move the position by.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if:
    /// * The resulting position would be out of bounds.
    /// * Addition overflows or underflows `isize` (Note: We should not encounter isize::MAX under normal circumstances).
    pub(crate) fn move_pos(&mut self, off: isize) -> Result<(), Report<BufferError>> {
        dev_trace_fmt!("GAMEDIG::CORE::BUFFER::<MOVE_POS>: {:?}", |f| {
            f.debug_struct("Args").field("off", &off).finish()
        });

        // just in case someone tries to move 0
        if off == 0 {
            dev_warn!("GAMEDIG::CORE::BUFFER::<MOVE_POS>: No movement (off = 0)");
            return Ok(());
        }

        match (self.pos() as isize).checked_add(off) {
            None => {
                Err(Report::new(BufferError::NotRepresentable)
                    .attach(FailureReason::new(
                        "The buffer cursor could not be repositioned because adding the requested \
                         offset to the current position produced a value that is not \
                         representable. The resulting cursor position exceeds the valid numeric \
                         range.",
                    ))
                    .attach(OpenGitHubIssue())
                    .attach(ContextComponent::new("Offset", off))
                    .attach(HexDump::new(
                        "Buffer (Cursor Not Representable Error)",
                        self.inner.clone(),
                        Some(self.pos()),
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO))
            }

            Some(n) if n < 0 || n as usize > self.len() => {
                Err(Report::new(BufferError::OutOfRange)
                    .attach(FailureReason::new(
                        "The buffer cursor could not be repositioned because the computed target \
                         index falls outside the valid range of the underlying data. The \
                         requested movement would place the cursor before the start of the buffer \
                         or beyond its end.",
                    ))
                    .attach(OpenGitHubIssue())
                    .attach(ContextComponent::new("Offset", off))
                    .attach(ContextComponent::new("Attempted Position", n))
                    .attach(HexDump::new(
                        "Buffer (Out of Range Cursor Error)",
                        self.inner.clone(),
                        Some(self.pos()),
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO))
            }

            Some(n) => {
                self.cursor = n as usize;

                Ok(())
            }
        }
    }

    /// Checks if a given range is valid within the buffer, relative to the current position.
    ///
    /// This internal helper function ensures that range bounds are correctly within the buffer’s size,
    /// and that no overflow occurs when calculating start/end positions. It is used to prevent out-of-bounds
    /// reads and to provide descriptive error messages if the requested range is invalid.
    ///
    /// # Arguments
    ///
    /// * `range` - The `RangeBounds` object specifying the range to check. Supports `..`, `..end`,
    ///   `start..`, and `start..end` forms, with `Included` and `Excluded` variants.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if:
    /// * The range results in arithmetic overflow or underflow.
    /// * The range is invalid (e.g., start > end).
    /// * The range extends beyond the length of the buffer.
    fn check_range(
        &self,
        range: impl RangeBounds<usize> + Debug,
    ) -> Result<(), Report<BufferError>> {
        dev_trace_fmt!("GAMEDIG::CORE::BUFFER::<CHECK_RANGE>: {:?}", |f| {
            f.debug_struct("Args")
                .field("range", &format_args!("{range:?}"))
                .finish()
        });

        let len = self.len();
        let pos = self.pos();

        let start = match range.start_bound() {
            Bound::Included(&n) => {
                match pos.checked_add(n) {
                    Some(v) => v,
                    None => {
                        return Err(Report::new(BufferError::RangeBoundsOverflow)
                            .attach(FailureReason::new(
                                "The buffer range could not be validated because the computed \
                                 start index is not representable in usize. Adding the start \
                                 bound to the base position overflowed.",
                            ))
                            .attach(OpenGitHubIssue())
                            .attach(ContextComponent::new("Position", pos))
                            .attach(ContextComponent::new("Start Bound", n))
                            .attach(ContextComponent::new("Start Bound Kind", "Included"))
                            .attach(ContextComponent::new("Range (Debug)", format!("{range:?}")))
                            .attach(SYSTEM_INFO)
                            .attach(CRATE_INFO));
                    }
                }
            }

            Bound::Excluded(&n) => {
                match n.checked_add(1).and_then(|x| pos.checked_add(x)) {
                    Some(v) => v,
                    None => {
                        return Err(Report::new(BufferError::RangeBoundsOverflow)
                            .attach(FailureReason::new(
                                "The buffer range could not be validated because the computed \
                                 start index is not representable in usize. Converting an \
                                 excluded start bound requires adding 1, and the resulting \
                                 arithmetic overflowed.",
                            ))
                            .attach(OpenGitHubIssue())
                            .attach(ContextComponent::new("Position", pos))
                            .attach(ContextComponent::new("Start Bound", n))
                            .attach(ContextComponent::new("Start Bound Kind", "Excluded"))
                            .attach(ContextComponent::new("Range (Debug)", format!("{range:?}")))
                            .attach(SYSTEM_INFO)
                            .attach(CRATE_INFO));
                    }
                }
            }

            Bound::Unbounded => pos,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => {
                match n.checked_add(1).and_then(|x| pos.checked_add(x)) {
                    Some(v) => v,
                    None => {
                        return Err(Report::new(BufferError::RangeBoundsOverflow)
                            .attach(FailureReason::new(
                                "The buffer range could not be validated because the computed end \
                                 index is not representable in usize. Converting an included end \
                                 bound requires adding 1, and the resulting arithmetic overflowed.",
                            ))
                            .attach(OpenGitHubIssue())
                            .attach(ContextComponent::new("Position", pos))
                            .attach(ContextComponent::new("End Bound", n))
                            .attach(ContextComponent::new("End Bound Kind", "Included"))
                            .attach(ContextComponent::new("Range (Debug)", format!("{range:?}")))
                            .attach(SYSTEM_INFO)
                            .attach(CRATE_INFO));
                    }
                }
            }

            Bound::Excluded(&n) => {
                match pos.checked_add(n) {
                    Some(v) => v,
                    None => {
                        return Err(Report::new(BufferError::RangeBoundsOverflow)
                            .attach(FailureReason::new(
                                "The buffer range could not be validated because the computed end \
                                 index is not representable in usize. Adding the end bound to the \
                                 base position overflowed.",
                            ))
                            .attach(OpenGitHubIssue())
                            .attach(ContextComponent::new("Position", pos))
                            .attach(ContextComponent::new("End Bound", n))
                            .attach(ContextComponent::new("End Bound Kind", "Excluded"))
                            .attach(ContextComponent::new("Range (Debug)", format!("{range:?}")))
                            .attach(SYSTEM_INFO)
                            .attach(CRATE_INFO));
                    }
                }
            }

            Bound::Unbounded => len,
        };

        if start > end {
            return Err(Report::new(BufferError::RangeBoundsInvalid)
                .attach(FailureReason::new(
                    "The provided range is invalid because the computed start index is greater \
                     than the computed end index.",
                ))
                .attach(OpenGitHubIssue())
                .attach(ContextComponent::new("Position", pos))
                .attach(ContextComponent::new("Computed Start", start))
                .attach(ContextComponent::new("Computed End", end))
                .attach(ContextComponent::new("Range (Debug)", format!("{range:?}")))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO));
        }

        if start > len || end > len {
            return Err(Report::new(BufferError::RangeBoundsOutOfBounds)
                .attach(FailureReason::new(
                    "The provided range is out of bounds for the underlying buffer. The computed \
                     start or end index exceeds the buffer length.",
                ))
                .attach(OpenGitHubIssue())
                .attach(ContextComponent::new("Position", pos))
                .attach(ContextComponent::new("Buffer Length", len))
                .attach(ContextComponent::new("Computed Start", start))
                .attach(ContextComponent::new("Computed End", end))
                .attach(ContextComponent::new("Range (Debug)", format!("{range:?}")))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO));
        }

        Ok(())
    }

    /// Peeks at the given number of bytes from the current position without advancing the cursor.
    ///
    /// This method checks that `cnt` bytes are available from the current position before returning
    /// the requested slice.
    ///
    /// # Arguments
    ///
    /// * `cnt` - The number of bytes to view from the current position.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the requested number of bytes (`cnt`) is not available from the current position.
    pub(crate) fn peek(&self, cnt: usize) -> Result<&[u8], Report<BufferError>> {
        dev_trace_fmt!("GAMEDIG::CORE::BUFFER::<PEEK>: {:?}", |f| {
            f.debug_struct("Args").field("cnt", &cnt).finish()
        });

        let pos = self.pos();

        self.check_range(.. cnt)
            .change_context(BufferError::RangeCheckFailed)
            .attach(FailureReason::new(
                "The peek operation could not be completed because the range check failed.",
            ))
            .attach(ContextComponent::new("Count", cnt))
            .attach(ContextComponent::new("Position", pos))?;

        let end = pos.checked_add(cnt).ok_or_else(|| {
            Report::new(BufferError::RangeBoundsOverflow)
                .attach(FailureReason::new(
                    "The peek operation could not compute a valid end index because adding the \
                     requested count to the current buffer position is not representable in usize.",
                ))
                .attach(OpenGitHubIssue())
                .attach(ContextComponent::new("Count", cnt))
                .attach(ContextComponent::new("Position", pos))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO)
        })?;

        Ok(&self.inner.as_ref()[pos .. end])
    }

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
    fn get_inner_slice<T, const N: usize, F>(
        &mut self,
        convert: F,
    ) -> Result<T, Report<BufferError>>
    where
        F: FnOnce([u8; N]) -> T,
    {
        dev_trace_fmt!("GAMEDIG::CORE::BUFFER::<GET_INNER_SLICE>: {:?}", |f| {
            f.debug_struct("Args").field("N", &N).finish()
        });

        self.check_range(.. N)
            .change_context(BufferError::RangeCheckFailed)
            .attach(FailureReason::new(
                "The fixed size slice read operation could not be completed because the range \
                 check failed.",
            ))
            .attach(ContextComponent::new("Bytes Requested", N))
            .attach(ContextComponent::new("Position", self.pos()))
            .attach(ContextComponent::new(
                "Target Type",
                std::any::type_name::<T>(),
            ))?;

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
    #[inline]
    pub(crate) fn read_u8(&mut self) -> Result<u8, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::IO::BUF::NUM::<READ_U8>: []");

        self.get_inner_slice::<u8, 1, _>(|x| x[0])
    }

    /// Read signed 8 bit integer (`i8`) from the buffer.
    ///
    /// Advances the cursor by `1` byte.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `1` byte is available at the current cursor position.
    #[inline]
    pub(crate) fn read_i8(&mut self) -> Result<i8, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_I8>");

        self.get_inner_slice::<i8, 1, _>(|x| x[0] as i8)
    }

    /// Read unsigned 16 bit integer (`u16`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `2` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `2` bytes are available.
    #[inline]
    pub(crate) fn read_u16_be(&mut self) -> Result<u16, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_U16_BE>");

        self.get_inner_slice::<u16, 2, _>(u16::from_be_bytes)
    }

    /// Read unsigned 16 bit integer (`u16`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `2` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `2` bytes are available.
    #[inline]
    pub(crate) fn read_u16_le(&mut self) -> Result<u16, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_U16_LE>");

        self.get_inner_slice::<u16, 2, _>(u16::from_le_bytes)
    }

    /// Read signed 16 bit integer (`i16`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `2` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `2` bytes are available.
    #[inline]
    pub(crate) fn read_i16_be(&mut self) -> Result<i16, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_I16_BE>");

        self.get_inner_slice::<i16, 2, _>(i16::from_be_bytes)
    }

    /// Read signed 16 bit integer (`i16`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `2` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `2` bytes are available.
    #[inline]
    pub(crate) fn read_i16_le(&mut self) -> Result<i16, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_I16_LE>");

        self.get_inner_slice::<i16, 2, _>(i16::from_le_bytes)
    }

    /// Read unsigned 32 bit integer (`u32`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[inline]
    pub(crate) fn read_u32_be(&mut self) -> Result<u32, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_U32_BE>");

        self.get_inner_slice::<u32, 4, _>(u32::from_be_bytes)
    }

    /// Read unsigned 32 bit integer (`u32`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[inline]
    pub(crate) fn read_u32_le(&mut self) -> Result<u32, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_U32_LE>");

        self.get_inner_slice::<u32, 4, _>(u32::from_le_bytes)
    }

    /// Read signed 32 bit integer (`i32`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[inline]
    pub(crate) fn read_i32_be(&mut self) -> Result<i32, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_I32_BE>");

        self.get_inner_slice::<i32, 4, _>(i32::from_be_bytes)
    }

    /// Read signed 32 bit integer (`i32`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[inline]
    pub(crate) fn read_i32_le(&mut self) -> Result<i32, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_I32_LE>");

        self.get_inner_slice::<i32, 4, _>(i32::from_le_bytes)
    }

    /// Read unsigned 64 bit integer (`u64`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[inline]
    pub(crate) fn read_u64_be(&mut self) -> Result<u64, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_U64_BE>");

        self.get_inner_slice::<u64, 8, _>(u64::from_be_bytes)
    }

    /// Read unsigned 64 bit integer (`u64`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[inline]
    pub(crate) fn read_u64_le(&mut self) -> Result<u64, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_U64_LE>");

        self.get_inner_slice::<u64, 8, _>(u64::from_le_bytes)
    }

    /// Read signed 64 bit integer (`i64`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[inline]
    pub(crate) fn read_i64_be(&mut self) -> Result<i64, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_I64_BE>");

        self.get_inner_slice::<i64, 8, _>(i64::from_be_bytes)
    }

    /// Read signed 64 bit integer (`i64`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[inline]
    pub(crate) fn read_i64_le(&mut self) -> Result<i64, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_I64_LE>");

        self.get_inner_slice::<i64, 8, _>(i64::from_le_bytes)
    }

    /// Read unsigned 128 bit integer (`u128`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `16` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `16` bytes are available.
    #[inline]
    pub(crate) fn read_u128_be(&mut self) -> Result<u128, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_U128_BE>");

        self.get_inner_slice::<u128, 16, _>(u128::from_be_bytes)
    }

    /// Read unsigned 128 bit integer (`u128`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `16` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `16` bytes are available.
    #[inline]
    pub(crate) fn read_u128_le(&mut self) -> Result<u128, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_U128_LE>");

        self.get_inner_slice::<u128, 16, _>(u128::from_le_bytes)
    }

    /// Read signed 128 bit integer (`i128`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `16` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `16` bytes are available.
    #[inline]
    pub(crate) fn read_i128_be(&mut self) -> Result<i128, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_I128_BE>");

        self.get_inner_slice::<i128, 16, _>(i128::from_be_bytes)
    }

    /// Read signed 128 bit integer (`i128`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `16` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `16` bytes are available.
    #[inline]
    pub(crate) fn read_i128_le(&mut self) -> Result<i128, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_I128_LE>");

        self.get_inner_slice::<i128, 16, _>(i128::from_le_bytes)
    }

    /// Read 32 bit floating point number (`f32`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[inline]
    pub(crate) fn read_f32_be(&mut self) -> Result<f32, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_F32_BE>");

        self.get_inner_slice::<f32, 4, _>(f32::from_be_bytes)
    }

    /// Read 32 bit floating point number (`f32`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `4` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `4` bytes are available.
    #[inline]
    pub(crate) fn read_f32_le(&mut self) -> Result<f32, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_F32_LE>");

        self.get_inner_slice::<f32, 4, _>(f32::from_le_bytes)
    }

    /// Read 64 bit floating point number (`f64`) in big endian (`BE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[inline]
    pub(crate) fn read_f64_be(&mut self) -> Result<f64, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_F64_BE>");

        self.get_inner_slice::<f64, 8, _>(f64::from_be_bytes)
    }

    /// Read 64 bit floating point number (`f64`) in little endian (`LE`) order.
    ///
    /// Advances the cursor by `8` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if fewer than `8` bytes are available.
    #[inline]
    pub(crate) fn read_f64_le(&mut self) -> Result<f64, Report<BufferError>> {
        dev_trace!("GAMEDIG::CORE::BUFFER::<READ_F64_LE>");

        self.get_inner_slice::<f64, 8, _>(f64::from_le_bytes)
    }

    /// Reads a `UTF 8` string from the buffer until a delimiter is encountered.
    ///
    /// # Parameters
    ///
    /// - `DELIMITER`: A byte value signifying the end of the string.  
    ///
    /// - `STRICT`: Determines how invalid `UTF 8` sequences are handled.
    ///
    ///   - `true`: Uses `from_utf8`. If any invalid `UTF 8` sequence is found,
    ///     an error is returned.
    ///
    ///   - `false`: Uses `from_utf8_lossy`, replacing invalid sequences with `�`.
    ///
    /// After reading up to (but not including) the delimiter, the cursor is advanced by the number
    /// of bytes read plus the length of the delimiter.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range goes out of bounds.
    /// - The delimiter is not found before the end of the buffer.
    /// - `STRICT` is `true` and the bytes do not form valid `UTF 8`.
    pub(crate) fn read_string_utf8<const DELIMITER: u8, const STRICT: bool>(
        &mut self,
    ) -> Result<String, Report<BufferError>> {
        dev_trace_fmt!("GAMEDIG::CORE::BUFFER::<READ_STRING_UTF8>: {:?}", |f| {
            f.debug_struct("Args")
                .field("delimiter", &DELIMITER)
                .field("strict", &STRICT)
                .finish()
        });

        self.check_range(..)
            .change_context(BufferError::RangeCheckFailed)
            .attach(FailureReason::new(
                "The requested string could not be read because the range check failed.",
            ))?;

        let start = self.pos();
        let buf = self.remaining_slice();

        let end_pos = memchr::memchr(DELIMITER, buf).ok_or_else(|| {
            Report::new(BufferError::DelimiterNotFound)
                .attach(FailureReason::new(
                    "The requested string could not be read because the delimiter was not found \
                     before the end of the buffer.",
                ))
                .attach(ContextComponent::new("Delimiter", DELIMITER))
                .attach(HexDump::new(
                    "Buffer (Delimiter Not Found)",
                    self.inner.clone(),
                    Some(start),
                ))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO)
        })?;

        let slice = &buf[.. end_pos];

        let s = if STRICT {
            str::from_utf8(slice)
                .map(str::to_owned)
                .change_context(BufferError::InvalidUTF8String)
                .attach(FailureReason::new(
                    "UTF-8 decoding failed while reading a delimited string. The byte sequence \
                     before the delimiter is not valid UTF-8.",
                ))
                .attach(ContextComponent::new("Delimiter", DELIMITER))
                .attach(ContextComponent::new("Bytes Read", end_pos))
                .attach(HexDump::new(
                    "Buffer (Invalid UTF-8)",
                    self.inner.clone(),
                    Some(start),
                ))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO)?
        } else {
            String::from_utf8_lossy(slice).into_owned()
        };

        self.cursor += end_pos + 1;

        Ok(s)
    }

    /// Reads a length prefixed `UTF 8` string from the buffer.
    ///
    /// The length prefix is a single `u8` that indicates how many bytes of `UTF 8` data follow.
    /// The method first reads this `u8` length, then reads that many bytes as a `UTF 8` string.
    ///
    /// # Parameters
    ///
    /// - `STRICT`: Determines how invalid `UTF 8` sequences are handled.
    ///
    ///   - `true`: Uses `from_utf8`. Any invalid `UTF 8` sequence causes an error.
    ///
    ///   - `false`: Uses `from_utf8_lossy`, replacing invalid sequences with `�`.
    ///
    /// After reading the length byte and the string data, the cursor advances by `1` byte for the length
    /// plus the number of bytes read for the string.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range is out of bounds.
    /// - `STRICT` is `true` and the bytes do not form valid `UTF 8`.
    pub(crate) fn read_string_utf8_len_prefixed<const STRICT: bool>(
        &mut self,
    ) -> Result<String, Report<BufferError>> {
        dev_trace_fmt!(
            "GAMEDIG::CORE::BUFFER::<READ_STRING_UTF8_LEN_PREFIXED>: {:?}",
            |f| { f.debug_struct("Args").field("strict", &STRICT).finish() }
        );

        let len_byte = self
            .peek(1)
            .change_context(BufferError::RangeCheckFailed)
            .attach(FailureReason::new(
                "The requested length prefixed string could not be read because the range check \
                 for the length byte failed.",
            ))?[0];

        let s_len = len_byte as usize;

        let total = 1usize.checked_add(s_len).ok_or_else(|| {
            Report::new(BufferError::RangeBoundsOverflow)
                .attach(FailureReason::new(
                    "The requested length prefixed string could not be read because the computed \
                     total length is not representable in usize.",
                ))
                .attach(ContextComponent::new("String Length", s_len))
                .attach(ContextComponent::new("Position", self.pos()))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO)
        })?;

        let block = self
            .peek(total)
            .change_context(BufferError::RangeCheckFailed)
            .attach(FailureReason::new(
                "The requested length prefixed string could not be read because the range check \
                 for the string data failed.",
            ))
            .attach(ContextComponent::new("String Length", s_len))
            .attach(ContextComponent::new("Position", self.pos()))?;

        let bytes = &block[1 ..];

        let s = if STRICT {
            str::from_utf8(bytes)
                .map(str::to_owned)
                .change_context(BufferError::InvalidUTF8String)
                .attach(FailureReason::new(
                    "Invalid UTF-8 sequence found during string read.",
                ))
                .attach(ContextComponent::new("String Length", s_len))
                .attach(HexDump::new(
                    "Buffer (Invalid UTF-8 in Length Prefixed String)",
                    self.inner.clone(),
                    Some(self.pos()),
                ))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO)?
        } else {
            String::from_utf8_lossy(bytes).into_owned()
        };

        self.cursor += total;

        Ok(s)
    }

    fn read_string_utf16<const DELIMITER: u16, const STRICT: bool, const LE: bool>(
        &mut self,
    ) -> Result<String, Report<BufferError>> {
        dev_trace_fmt!("GAMEDIG::CORE::BUFFER::<READ_STRING_UTF16>: {:?}", |f| {
            f.debug_struct("Args")
                .field("delimiter", &DELIMITER)
                .field("strict", &STRICT)
                .field("le", &LE)
                .finish()
        });

        self.check_range(..)
            .change_context(BufferError::RangeCheckFailed)
            .attach(FailureReason::new(
                "The requested UTF-16 string could not be read because the range check failed.",
            ))?;

        let start = self.pos();
        let buf = self.remaining_slice();

        let needle = if LE {
            DELIMITER.to_le_bytes()
        } else {
            DELIMITER.to_be_bytes()
        };

        let end_pos = memchr::memmem::find(buf, &needle).ok_or_else(|| {
            Report::new(BufferError::DelimiterNotFound)
                .attach(FailureReason::new(
                    "The requested UTF-16 string could not be read because the delimiter was not \
                     found before the end of the buffer.",
                ))
                .attach(ContextComponent::new("Delimiter", DELIMITER))
                .attach(HexDump::new(
                    "Buffer (Delimiter Not Found)",
                    self.inner.clone(),
                    Some(start),
                ))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO)
        })?;

        if (end_pos % 2) != 0 {
            return Err(Report::new(BufferError::InvalidUTF16String)
                .attach(FailureReason::new(
                    "The UTF-16 delimiter was found at an unaligned offset.",
                ))
                .attach(ContextComponent::new("Delimiter", DELIMITER))
                .attach(ContextComponent::new("Delimiter Offset", end_pos))
                .attach(HexDump::new(
                    "Buffer (Unaligned UTF-16 Delimiter)",
                    self.inner.clone(),
                    Some(start),
                ))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO));
        }

        let data = &buf[.. end_pos];

        let unit_count = data.len() / 2;
        let mut units = Vec::with_capacity(unit_count);

        for i in 0 .. unit_count {
            let b0 = data[2 * i];
            let b1 = data[2 * i + 1];

            units.push(
                if LE {
                    u16::from_le_bytes([b0, b1])
                } else {
                    u16::from_be_bytes([b0, b1])
                },
            );
        }

        let s = if STRICT {
            String::from_utf16(&units)
                .change_context(BufferError::InvalidUTF16String)
                .attach(FailureReason::new(
                    "Invalid UTF-16 sequence found during string read.",
                ))
                .attach(ContextComponent::new("Delimiter", DELIMITER))
                .attach(ContextComponent::new("Bytes Read", end_pos))
                .attach(HexDump::new(
                    "Buffer (Invalid UTF-16 Sequence)",
                    self.inner.clone(),
                    Some(start),
                ))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO)?
        } else {
            String::from_utf16_lossy(&units)
        };

        self.cursor += end_pos + 2;

        Ok(s)
    }

    /// Reads a `UTF 16` string in big endian (`BE`) order until a `2 byte` delimiter is encountered.
    ///
    /// # Parameters
    ///
    /// - `DELIMITER`: A `u16` value marking the end of the string.  
    ///
    /// - `STRICT`: Determines how invalid `UTF 16` sequences are handled.
    ///
    ///   - `true`: Uses `from_utf16`. Any invalid `UTF 16` sequence causes an error.
    ///
    ///   - `false`: Uses `from_utf16_lossy`, replacing invalid sequences with `�`.
    ///
    /// After reading up to the delimiter, the cursor advances by the number of bytes read plus the delimiter length (2 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range goes out of bounds.
    /// - The UTF-16 isn't properly aligned.
    /// - `STRICT` is `true` and invalid `UTF 16` data is encountered.
    pub(crate) fn read_string_utf16_be<const DELIMITER: u16, const STRICT: bool>(
        &mut self,
    ) -> Result<String, Report<BufferError>> {
        dev_trace_fmt!("GAMEDIG::CORE::BUFFER::<READ_STRING_UTF16_BE>: {:?}", |f| {
            f.debug_struct("Args")
                .field("delimiter", &DELIMITER)
                .field("strict", &STRICT)
                .finish()
        });

        self.read_string_utf16::<DELIMITER, STRICT, false>()
    }

    /// Reads a `UTF 16` string in little endian (`LE`) order until a `2 byte` delimiter is encountered.
    ///
    /// # Parameters
    ///
    /// - `DELIMITER`: A `u16` value marking the end of the string.
    ///
    /// - `STRICT`: Determines how invalid `UTF 16` sequences are handled.
    ///
    ///   - `true`: Uses `from_utf16`. Any invalid `UTF 16` sequence causes an error.
    ///
    ///   - `false`: Uses `from_utf16_lossy`, replacing invalid sequences with `�`.
    ///
    /// After reading up to the delimiter, the cursor advances by the number of bytes read plus the delimiter length (2 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range goes out of bounds.
    /// - The UTF-16 isn't properly aligned.
    /// - `STRICT` is `true` and invalid `UTF 16` data is encountered.
    pub(crate) fn read_string_utf16_le<const DELIMITER: u16, const STRICT: bool>(
        &mut self,
    ) -> Result<String, Report<BufferError>> {
        dev_trace_fmt!("GAMEDIG::CORE::BUFFER::<READ_STRING_UTF16_LE>: {:?}", |f| {
            f.debug_struct("Args")
                .field("delimiter", &DELIMITER)
                .field("strict", &STRICT)
                .finish()
        });

        self.read_string_utf16::<DELIMITER, STRICT, true>()
    }

    /// Reads a `UCS 2` encoded string from the buffer.
    ///
    /// `UCS 2` is essentially `UTF 16` without surrogate pairs. This method delegates to
    /// `read_string_utf16_le` as `UCS 2` data is always handled as `UTF 16 LE`.
    ///
    /// # Parameters
    ///
    /// - `DELIMITER`: A `u16` value marking the end of the string.
    ///
    /// - `STRICT`: Determines how invalid sequences are handled.
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
    /// - The UTF-16 isn't properly aligned.
    /// - `STRICT` is `true` and invalid `UTF 16` data is encountered.
    pub(crate) fn read_string_ucs2<const DELIMITER: u16, const STRICT: bool>(
        &mut self,
    ) -> Result<String, Report<BufferError>> {
        dev_trace_fmt!("GAMEDIG::CORE::BUFFER::<READ_STRING_UCS2>: {:?}", |f| {
            f.debug_struct("Args")
                .field("delimiter", &DELIMITER)
                .field("strict", &STRICT)
                .finish()
        });

        self.read_string_utf16_le::<DELIMITER, STRICT>()
    }

    /// Reads a `Latin 1` (`Windows 1252` under the hood) encoded string from the buffer until a delimiter is reached.
    ///
    /// # Parameters
    ///
    /// - `DELIMITER`: A byte value marking the end of the string.  
    ///
    /// - `STRICT`: Determines how invalid sequences are handled.
    ///
    ///   - `true`: Returns an error if decoding encounters invalid sequences.
    ///
    ///   - `false`: Replaces invalid sequences with `�`.
    ///
    /// After reading up to the delimiter, the cursor advances by the number of bytes read plus the delimiter length.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested range goes out of bounds.
    /// - `STRICT` is `true` and invalid `Latin 1` data is encountered.
    #[cfg(feature = "_BUFFER_READ_LATIN_1")]
    pub(crate) fn read_string_latin1<const DELIMITER: u8, const STRICT: bool>(
        &mut self,
    ) -> Result<String, Report<BufferError>> {
        self.check_range(..)
            .change_context(BufferError::RangeCheckFailed)
            .attach(FailureReason::new(
                "The requested Latin 1 string could not be read because the range check failed.",
            ))?;

        let start = self.pos();
        let buf = self.remaining_slice();

        let end_pos = memchr::memchr(DELIMITER, buf).ok_or_else(|| {
            Report::new(BufferError::DelimiterNotFound)
                .attach(FailureReason::new(
                    "The requested Latin 1 string could not be read because the delimiter was not \
                     found before the end of the buffer.",
                ))
                .attach(ContextComponent::new("Delimiter", DELIMITER))
                .attach(HexDump::new(
                    "Buffer (Delimiter Not Found)",
                    self.inner.clone(),
                    Some(start),
                ))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO)
        })?;

        let slice = &buf[.. end_pos];
        let (decoded, _, had_errors) = encoding_rs::WINDOWS_1252.decode(slice);

        if STRICT && had_errors {
            return Err(Report::new(BufferError::InvalidLatin1String)
                .attach(FailureReason::new(
                    "Invalid Latin 1 sequence found during string read.",
                ))
                .attach(HexDump::new(
                    "Buffer (Invalid Latin 1 Sequence)",
                    self.inner.clone(),
                    Some(start),
                ))
                .attach(ContextComponent::new("Delimiter", DELIMITER))
                .attach(ContextComponent::new("Bytes Read", end_pos))
                .attach(HexDump::new(
                    "Buffer (Invalid Latin 1 Sequence)",
                    self.inner.clone(),
                    Some(start),
                ))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO));
        }

        let out = decoded.into_owned();

        self.cursor += end_pos + 1;

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_properties_heap() {
        let data = vec![1, 2, 3, 4, 5];

        let buf = Buffer::new(data.clone());

        assert_eq!(buf.pos(), 0);
        assert_eq!(buf.len(), data.len());
        assert_eq!(buf.remaining(), data.len());
    }

    #[test]
    fn test_new_and_properties_stack() {
        let data = [10, 20, 30, 40];

        let buf = Buffer::new(data);

        assert_eq!(buf.pos(), 0);
        assert_eq!(buf.len(), 4);
        assert_eq!(buf.remaining(), 4);
    }

    #[test]
    fn test_move_pos_valid() {
        let data = vec![1, 2, 3, 4, 5];

        let mut buf = Buffer::new(data);

        assert_eq!(buf.pos(), 0);

        buf.move_pos(2).unwrap();
        assert_eq!(buf.pos(), 2);

        buf.move_pos(-1).unwrap();
        assert_eq!(buf.pos(), 1);
    }

    #[test]
    #[should_panic]
    fn test_move_pos_out_of_bounds() {
        let data = vec![1, 2, 3];

        let mut buf = Buffer::new(data);

        // Beyond the buffer length.
        let _ = buf.move_pos(10).unwrap();
    }

    #[test]
    fn test_peek_valid() {
        let data = vec![100, 101, 102];

        let buf = Buffer::new(data);

        let slice = buf.peek(2).unwrap();
        assert_eq!(slice, &[100, 101]);
    }

    #[test]
    #[should_panic]
    fn test_peek_out_of_bounds() {
        let data = vec![1, 2];

        let buf = Buffer::new(data);

        // Beyond the buffer length.
        let _ = buf.peek(3).unwrap();
    }

    #[test]
    fn test_unpack() {
        let data = vec![42, 43];

        let buf = Buffer::new(data.clone());

        let unpacked = buf.unpack();
        assert_eq!(unpacked, data);
    }

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

    #[test]
    fn test_read_string_utf8_strict_heap() {
        let data = b"Hello\x00World";

        let mut buf = Buffer::new(data.to_vec());

        let s = buf.read_string_utf8(None, true).unwrap();
        assert_eq!(s, "Hello");
        assert_eq!(buf.pos(), 6);
    }

    #[test]
    fn test_read_string_utf8_strict_stack() {
        let data = *b"Hello\x00World";

        let mut buf = Buffer::new(data);

        let s = buf.read_string_utf8(None, true).unwrap();
        assert_eq!(s, "Hello");
        assert_eq!(buf.pos(), 6);
    }

    #[test]
    fn test_read_string_utf8_non_strict_invalid() {
        let data = vec![0xFF, 0x00];

        let mut buf = Buffer::new(data);

        let s = buf.read_string_utf8(None, false).unwrap();
        assert_eq!(s, "\u{FFFD}");
    }

    #[test]
    fn test_read_string_utf8_len_prefixed_strict() {
        let data = vec![6, b'H', b'e', b'l', b'l', b'o'];

        let mut buf = Buffer::new(data);

        let s = buf.read_string_utf8_len_prefixed(true).unwrap();
        assert_eq!(s, "Hello");
        assert_eq!(buf.pos(), 6);
    }

    #[test]
    fn test_read_string_utf16_be_strict() {
        let data = vec![0x00, 0x48, 0x00, 0x69, 0x00, 0x00];

        let mut buf = Buffer::new(data);

        let s = buf.read_string_utf16_be(None, true).unwrap();
        assert_eq!(s, "Hi");
        assert_eq!(buf.pos(), 6);
    }

    #[test]
    fn test_read_string_utf16_le_strict() {
        let data = vec![0x48, 0x00, 0x69, 0x00, 0x00, 0x00];

        let mut buf = Buffer::new(data);

        let s = buf.read_string_utf16_le(None, true).unwrap();
        assert_eq!(s, "Hi");
        assert_eq!(buf.pos(), 6);
    }

    #[test]
    fn test_read_string_ucs2_strict() {
        let data = vec![0x48, 0x00, 0x69, 0x00, 0x00, 0x00];

        let mut buf = Buffer::new(data);

        let s = buf.read_string_ucs2(None, true).unwrap();
        assert_eq!(s, "Hi");
        assert_eq!(buf.pos(), 6);
    }

    #[cfg(feature = "_BUFFER_READ_LATIN_1")]
    #[test]
    fn test_read_string_latin1_strict() {
        let data = vec![b'H', 0xEB, b'l', b'l', b'o', 0x00];

        let mut buf = Buffer::new(data);

        let s = buf.read_string_latin1(None, true).unwrap();
        assert_eq!(s, "Hëllo");
        assert_eq!(buf.pos(), 6);
    }
}
