use {
    crate::error::{
        ErrorKind,
        IoError,
        Report,
        Result,
        diagnostic::{FailureReason, HexDump, OpenGitHubIssue},
    },
    std::ops::{Bound, RangeBounds},
};

mod num;
mod string;

/// The `Bufferable` trait abstracts types that represent byte storage and provides
/// a method to retrieve the length of the underlying storage.
pub(crate) trait Bufferable: Clone + AsRef<[u8]> + Into<Vec<u8>> {
    /// If the underlying storage has a known size at compile time.
    const IS_FIXED_SIZE: bool;

    /// Returns the number of elements in the underlying byte storage.
    fn len(&self) -> usize;
}

impl Bufferable for Vec<u8> {
    const IS_FIXED_SIZE: bool = false;

    fn len(&self) -> usize { self.len() }
}

impl<const N: usize> Bufferable for [u8; N] {
    const IS_FIXED_SIZE: bool = true;

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
        dev_trace!(
            "GAMEDIG::CORE::IO::BUF::<NEW>: [inner: alloc({})]",
            if B::IS_FIXED_SIZE { "stack" } else { "heap" }
        );

        Self { inner, cursor: 0 }
    }

    /// Returns the current position in the buffer
    ///
    /// The position is zero based and increments as you read or move through the buffer.
    #[inline]
    pub(crate) fn pos(&self) -> usize {
        dev_trace!("GAMEDIG::CORE::IO::BUF::<POS>: []");

        self.cursor
    }

    /// Returns the number of elements in the underlying byte storage.
    #[inline]
    pub(crate) fn len(&self) -> usize {
        dev_trace!("GAMEDIG::CORE::IO::BUF::<LEN>: []");

        self.inner.len()
    }

    /// Returns the number of elements remaining from the current position to the end of the byte storage.
    ///
    /// This gives you how many more bytes can be read without going out of bounds.
    #[inline]
    pub(crate) fn remaining(&self) -> usize {
        dev_trace!("GAMEDIG::CORE::IO::BUF::<REMAINING>: []");

        self.len() - self.pos()
    }

    /// Consumes the `Buffer` and returns the underlying byte storage.
    ///
    /// This conversion moves the underlying byte storage out of the `Buffer`,
    /// effectively discarding the `Buffer` wrapper.
    #[inline]
    pub(crate) fn unpack(self) -> B {
        dev_trace!("GAMEDIG::CORE::IO::BUF::<UNPACK>: []");

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
    pub(crate) fn move_pos(&mut self, off: isize) -> Result<()> {
        dev_trace!("GAMEDIG::CORE::IO::BUF::<MOVE_POS>: [off: {off}]");

        // just in case someone tries to move 0
        if off == 0 {
            dev_warn!("GAMEDIG::CORE::IO::BUF::<MOVE_POS>: No movement (off = 0)");

            return Ok(());
        }

        match (self.pos() as isize).checked_add(off) {
            None => {
                return Err(Report::new(ErrorKind::from(
                    IoError::BufferPositionArithmeticError {},
                ))
                .attach_printable(FailureReason::new(
                    "Movement of the buffer position resulted in an arithmetic error.",
                ))
                .attach_printable(OpenGitHubIssue()));
            }

            Some(n) if n < 0 || n as usize > self.len() => {
                return Err(
                    Report::new(ErrorKind::from(IoError::BufferMovedOutOfBoundsError {
                        attempted: off,
                        position: self.pos(),
                        available: self.len(),
                    }))
                    .attach_printable(FailureReason::new(
                        "Attempted to move the buffer position out of bounds.",
                    ))
                    .attach_printable(OpenGitHubIssue()),
                );
            }

            Some(n) => {
                self.cursor = n as usize;

                Ok(())
            }
        }
    }

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
        dev_trace!("GAMEDIG::CORE::IO::BUF::<CHECK_RANGE>: [range: {range:?}, pos_ctx: {pos_ctx}]");

        let len = self.len();
        let pos = if pos_ctx { self.pos() } else { 0 };

        let check_overflow = |res: Option<usize>| {
            res.ok_or_else(|| {
                Report::new(ErrorKind::from(IoError::BufferRangeOverflowError {}))
                    .attach_printable(FailureReason::new(
                        "Attempted to read a range that overflows usize.",
                    ))
                    .attach_printable(OpenGitHubIssue())
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
                .attach_printable(HexDump::new("Buffer", self.inner.clone().into(), Some(pos)))
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
                    "Attempted to access out of bounds range in the buffer.",
                ))
                .attach_printable(HexDump::new("Buffer", self.inner.clone().into(), Some(pos)))
                .attach_printable(OpenGitHubIssue()),
            );
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
    pub(crate) fn peek(&self, cnt: usize) -> Result<&[u8]> {
        dev_trace!("GAMEDIG::CORE::IO::BUF::<PEEK>: [cnt: {cnt}]");

        self.check_range(.. cnt, true)?;

        let pos = self.pos();

        Ok(&self.inner.as_ref()[pos .. pos + cnt])
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
}
