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

    /// Creates a new Buffer from a socket read / recv tuple.
    pub(crate) fn from_socket(tuple: (Vec<u8>, usize)) -> Self { Self::new(tuple.0, tuple.1) }

    /// Consumes the Buffer and returns the inner Vec<u8>.
    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> Vec<u8> { self.inner }
}
