use {
    crate::error::{
        diagnostic::{FailureReason, OpenGitHubIssue},
        ErrorKind,
        IoError,
        Report,
        Result,
    },

    std::ops::{Bound, RangeBounds},
};

mod num;
mod string;

/// `Buffer` is a lightweight, runtime agnostic abstraction over a `Vec<u8>`
/// that provides safe indexing and cursor based read operations. It does not rely
/// on any specific runtime, Instead it focuses on providing a safe, ergonomic API for
/// in-memory data access.
// TODO: It would be nice to have a IO pipeline from net but i think this is something for future
// TODO: ^^ Need to figure out a way to cleanly implement this without making it too complex
pub(crate) struct Buffer {
    /// The underlying byte storage.
    inner: Vec<u8>,
    /// The current position in the buffer.
    cursor: usize,
}

impl Buffer {
    /// Creates a new `Buffer` from a provided `Vec<u8>`.
    ///
    /// # Arguments
    ///
    /// * `vec` - A vector of bytes that will back the `Buffer`.
    ///
    /// # Examples
    ///
    /// ```
    /// let data = vec![0x10, 0x20, 0x30];
    /// let buffer = Buffer::new(data);
    ///
    /// assert_eq!(buffer.len(), 3);
    /// assert_eq!(buffer.pos(), 0);
    /// ```
    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn new(vec: Vec<u8>) -> Self {
        Self {
            inner: vec,
            cursor: 0,
        }
    }
    /// This position is zero-based and increments as you read or move through the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// let buffer = Buffer::new(vec![1, 2, 3, 4]);
    /// assert_eq!(buffer.pos(), 0);
    /// ```
    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn pos(&self) -> usize { self.cursor }

    /// Returns the total number of bytes stored in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// let buffer = Buffer::new(vec![1, 2, 3]);
    /// assert_eq!(buffer.len(), 3);
    /// ```
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn len(&self) -> usize { self.inner.len() }

    /// Returns the number of bytes remaining from the current position to the end of the buffer.
    ///
    /// This gives you how many more bytes can be read without going out-of-bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut buffer = Buffer::new(vec![1, 2, 3]);
    /// // Initially, all 3 bytes remain.
    /// assert_eq!(buffer.remaining(), 3);
    ///
    /// // After moving the position forward by 1,
    /// // there should be 2 bytes remaining.
    /// buffer.move_pos(1).unwrap();
    /// assert_eq!(buffer.remaining(), 2);
    /// ```
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn remaining(&self) -> usize { self.len() - self.pos() }

    /// Checks if a given range is valid within the buffer, optionally relative to the current position.
    ///
    /// This internal helper function ensures that range bounds are correctly within the buffer’s size,
    /// and that no overflow occurs when calculating start/end positions. It is used to prevent out-of-bounds
    /// reads and to provide descriptive error messages if the requested range is invalid.
    ///
    /// # Arguments
    ///
    /// * `range` - The `RangeBounds` object specifying the range to check. Supports `..`, `..end`,
    ///   `start..`, and `start..end` forms, with `Included` and `Excluded` variants.
    /// * `pos_ctx` - If `true`, the range is interpreted relative to the current buffer position.
    ///   If `false`, it is interpreted as absolute indices into the buffer.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if:
    /// * The range results in arithmetic overflow or underflow.
    /// * The range is invalid (e.g., start > end).
    /// * The range extends beyond the length of the buffer.
    fn check_range(&self, range: impl RangeBounds<usize>, pos_ctx: bool) -> Result<()> {
        let len = self.len();
        let pos = if pos_ctx { self.pos() } else { 0 };

        let check_overflow = |res: Option<usize>| {
            res.ok_or_else(|| {
                return Report::new(ErrorKind::from(IoError::BufferRangeOverflowError {}))
                    .attach_printable(FailureReason::new(
                        "Attempted to read a range that overflows usize.",
                    ))
                    .attach_printable(OpenGitHubIssue());
            })
        };

        let start = check_overflow(match range.start_bound() {
            Bound::Included(&n) => pos.checked_add(n),
            Bound::Excluded(&n) => pos.checked_add(n + 1),
            Bound::Unbounded => Some(pos),
        })?;

        let end = check_overflow(match range.end_bound() {
            Bound::Included(&n) => pos.checked_add(n + 1),
            Bound::Excluded(&n) => pos.checked_add(n),
            Bound::Unbounded => Some(len),
        })?;

        if start > end {
            return Err(
                Report::new(ErrorKind::from(IoError::BufferRangeInvalidError {
                    start,
                    end,
                }))
                .attach_printable(FailureReason::new(
                    "Invalid range provided to buffer read operation.",
                ))
                .attach_printable(OpenGitHubIssue()),
            );
        }

        if start > len || end > len {
            return Err(
                Report::new(ErrorKind::from(IoError::BufferOutOfBoundsError {
                    attempted: end - start,
                    available: len - start,
                }))
                .attach_printable(FailureReason::new(
                    "Attempted to access out-of-bounds range in the buffer.",
                ))
                .attach_printable(OpenGitHubIssue()),
            );
        }

        Ok(())
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
    /// * The resulting position would be out-of-bounds.
    /// * Addition overflows or underflows `isize`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut buffer = Buffer::new(vec![10, 20, 30, 40]);
    /// buffer.move_pos(2).unwrap();
    /// assert_eq!(buffer.pos(), 2);
    /// buffer.move_pos(-1).unwrap();
    /// assert_eq!(buffer.pos(), 1);
    /// ```
    pub(crate) fn move_pos(&mut self, off: isize) -> Result<()> {
        // just in case someone tries to move 0
        if off == 0 {
            return Ok(());
        }

        match (self.pos() as isize).checked_add(off) {
            None => {
                //TODO: Handle this error
                todo!(
                    "The addition was not successful (i.e., it resulted in an overflow or \
                     underflow of isize)"
                )
            }

            Some(n) if n < 0 || n as usize > self.len() => {
                //TODO: Handle this error
                todo!("The new cursor position is out of bounds")
            }

            Some(n) => {
                self.cursor = n as usize;
                Ok(())
            }
        }
    }

    /// Peeks at the given number of bytes from the current position without advancing the cursor.
    ///
    /// This method checks that `cnt` bytes are available from the current position before returning
    /// the requested slice. If insufficient data is available, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `cnt` - The number of bytes to view from the current position.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the requested number of bytes (`cnt`) is not available from the current position.
    ///
    /// # Examples
    ///
    /// ```
    /// let buffer = Buffer::new(vec![0x01, 0x02, 0x03]);
    /// let slice = buffer.peek(2).unwrap();
    /// assert_eq!(slice, &[0x01, 0x02]);
    ///
    /// // position remains unchanged
    /// assert_eq!(buffer.pos(), 0);
    /// ```
    pub(crate) fn peek(&self, cnt: usize) -> Result<&[u8]> {
        self.check_range(.. cnt, true)?;

        let pos = self.pos();

        Ok(&self.inner[pos .. pos + cnt])
    }
}

impl Into<Vec<u8>> for Buffer {
    /// Consumes the `Buffer` and returns the inner `Vec<u8>`.
    ///
    /// This conversion moves the underlying vector out of the `Buffer`,
    /// effectively discarding the `Buffer` wrapper.
    ///
    /// # Examples
    ///
    /// ```
    /// let buffer = Buffer::new(vec![4, 5, 6]);
    /// let inner_vec: Vec<u8> = buffer.into();
    /// assert_eq!(inner_vec, vec![4, 5, 6]);
    /// ```
    fn into(self) -> Vec<u8> { self.inner }
}
