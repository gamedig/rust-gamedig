mod num;
mod string;

pub(crate) struct Buffer {
    inner: Vec<u8>,
    pos: usize,
}

impl Buffer {
    /// Creates a new Buffer.
    #[allow(dead_code)]
    pub(crate) const fn new(vec: Vec<u8>) -> Self { Self { inner: vec, pos: 0 } }

    /// Consumes the Buffer and returns the inner Vec<u8>.
    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> Vec<u8> { self.inner }
}