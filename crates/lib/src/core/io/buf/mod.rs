use {
    crate::error::{
        diagnostic::{FailureReason, HexDump, OpenGitHubIssue},
        ErrorKind,
        IoError,
        Report,
        Result,
    },

    std::ops::{Bound, RangeBounds},
};

mod num;
mod string;

pub(crate) enum Direction {
    Forward,
    Backward,
}

pub(crate) struct Buffer {
    inner: Vec<u8>,
    len: usize,
    pos: usize,
}

impl Buffer {
    /// Creates a new Buffer.
    #[allow(dead_code)]
    pub(crate) const fn new(vec: Vec<u8>, len: usize) -> Self {
        Self {
            inner: vec,
            len,
            pos: 0,
        }
    }

    pub(crate) fn peek(&self, bytes: usize) -> Result<&[u8]> {
        self.check_range(.. bytes)?;

        Ok(&self.inner[self.pos .. self.pos + bytes])
    }

    pub(crate) fn move_pos(&mut self, direction: Direction, bytes: usize) -> Result<()> {
        let new_pos = match direction {
            Direction::Forward => self.pos + bytes,
            Direction::Backward => {
                self.pos.checked_sub(bytes).ok_or_else(|| {
                    Report::new(ErrorKind::from(IoError::BufferOutOfBoundsError {
                        attempted: bytes,
                        available: self.len - self.pos,
                    }))
                    .attach_printable(FailureReason::new(
                        "Attempted to move the buffer position out of bounds. (Attempt is \
                         backwards)",
                    ))
                    .attach_printable(HexDump::new(
                        "Position moved OOB (new_pos < 0)",
                        self.inner.clone(),
                        Some(self.pos),
                    ))
                    .attach_printable(OpenGitHubIssue())
                })?
            }
        };

        if new_pos > self.len {
            return Err(
                Report::new(ErrorKind::from(IoError::BufferOutOfBoundsError {
                    attempted: new_pos,
                    available: self.len - self.pos,
                }))
                .attach_printable(FailureReason::new(
                    "Attempted to move the buffer position out of bounds.",
                ))
                .attach_printable(HexDump::new(
                    "Position moved OOB",
                    self.inner.clone(),
                    Some(self.pos),
                ))
                .attach_printable(OpenGitHubIssue()),
            );
        }

        self.pos = new_pos;

        Ok(())
    }

    fn check_range<R>(&self, range: R) -> Result<()>
    where R: RangeBounds<usize> {
        let start = match range.start_bound() {
            Bound::Included(&start) => self.pos + start,
            Bound::Excluded(_) => unreachable!(),
            Bound::Unbounded => self.pos,
        };

        let end = match range.end_bound() {
            Bound::Included(_) => unreachable!(),
            Bound::Excluded(&end) => self.pos + end,
            Bound::Unbounded => self.len,
        };

        if start > end {
            return Err(
                Report::new(ErrorKind::from(IoError::BufferInvalidRangeError {
                    start,
                    end,
                }))
                .attach_printable(FailureReason::new(
                    "Invalid range provided to buffer read operation.",
                ))
                .attach_printable(HexDump::new(
                    format!("Invalid range: [{start}..{end}]"),
                    self.inner.clone(),
                    Some(self.pos),
                ))
                .attach_printable(OpenGitHubIssue()),
            );
        }

        if start > self.len || end > self.len {
            return Err(
                Report::new(ErrorKind::from(IoError::BufferOutOfBoundsError {
                    attempted: end - start,
                    available: self.len - start,
                }))
                .attach_printable(FailureReason::new(
                    "Attempted to access out-of-bounds range in the buffer.",
                ))
                .attach_printable(HexDump::new(
                    format!("Range is OOB: [{start}..{end}]"),
                    self.inner.clone(),
                    Some(self.pos),
                ))
                .attach_printable(OpenGitHubIssue()),
            );
        }

        Ok(())
    }
}

impl From<(Vec<u8>, usize)> for Buffer {
    /// Creates a new Buffer from a socket read / recv tuple.
    fn from(tuple: (Vec<u8>, usize)) -> Self { Self::new(tuple.0, tuple.1) }
}

impl Into<Vec<u8>> for Buffer {
    /// Consumes the Buffer and returns the inner Vec<u8>.
    fn into(self) -> Vec<u8> { self.inner }
}
