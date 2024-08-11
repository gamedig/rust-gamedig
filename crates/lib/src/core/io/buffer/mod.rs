pub(crate) mod numeric;
pub(crate) mod string;

#[cfg(feature = "client_std")]
use std::io::{BufReader, Read};
#[cfg(feature = "client_tokio")]
use tokio::io::{AsyncRead, BufReader};

#[cfg(feature = "client_std")]
pub(crate) trait MaybeAsyncRead: Read {}
#[cfg(feature = "client_std")]
impl<T: Read> MaybeAsyncRead for T {}

#[cfg(feature = "client_tokio")]
pub(crate) trait MaybeAsyncRead: AsyncRead + Send + Unpin {}
#[cfg(feature = "client_tokio")]
impl<T: AsyncRead + Send + Unpin> MaybeAsyncRead for T {}

pub(crate) struct Buffer<R: MaybeAsyncRead> {
    reader: BufReader<R>,
}

impl<R: MaybeAsyncRead> From<BufReader<R>> for Buffer<R> {
    fn from(reader: BufReader<R>) -> Self { Self { reader } }
}
