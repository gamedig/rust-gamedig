use std::ops::{Bound, RangeBounds};

use crate::error::{
    diagnostic::{FailureReason, HexDump, OpenGitHubIssue},
    ErrorKind,
    IoError,
    Report,
    Result,
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

    fn check_range<R>(&self, range: R) -> Result<()>
    where R: RangeBounds<usize> {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.len,
        };

        if start > self.len || end > self.len {
            return Err(Report::new(ErrorKind::from(IoError::UnderflowError {
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
