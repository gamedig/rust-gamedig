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

    pub(crate) fn move_pos(&mut self, pos: isize) -> Result<()> {
        let new_pos = self.pos as isize + pos;

        if new_pos < 0 || new_pos as usize > self.len {
            return Err(Report::new(ErrorKind::from(IoError::BufferUnderflowError {
                attempted: pos as usize,
                available: self.len - self.pos,
            }))
            .attach_printable(FailureReason::new(
                "Attempted to move the buffer position out of bounds.",
            ))
            .attach_printable(HexDump::new(
                format!("Current buffer state (pos: {})", self.pos),
                self.inner.clone(),
            ))
            .attach_printable(OpenGitHubIssue()));
        }

        self.pos = new_pos as usize;

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

        if start > self.len || end > self.len {
            return Err(Report::new(ErrorKind::from(IoError::BufferUnderflowError {
                attempted: end - start,
                available: self.len - start,
            }))
            .attach_printable(FailureReason::new(
                "Attempted to access out-of-bounds range in the buffer.",
            ))
            .attach_printable(HexDump::new(
                format!("Current buffer state (pos: {})", self.pos),
                self.inner.clone(),
            ))
            .attach_printable(OpenGitHubIssue()));
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
