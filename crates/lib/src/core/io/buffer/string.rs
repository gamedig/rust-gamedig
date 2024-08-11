use super::{Buffer, MaybeAsyncRead};

#[cfg(feature = "client_std")]
use std::io::Read;
#[cfg(feature = "client_tokio")]
use tokio::io::AsyncReadExt;

// TODO: just example code, impl/improve old fns
#[maybe_async::maybe_async]
pub(crate) trait StringReader {
    async fn read_fixed_length_string(&mut self, length: usize) -> String;

    async fn read_null_terminated_string(&mut self) -> String;
    async fn read_prefixed_length_string(&mut self) -> String;

    async fn read_utf16_le_string(&mut self, length: usize) -> String;
    async fn read_utf16_be_string(&mut self, length: usize) -> String;
}

#[maybe_async::maybe_async]
impl<R: MaybeAsyncRead> StringReader for Buffer<R> {
    async fn read_fixed_length_string(&mut self, length: usize) -> String {
        let mut buf = vec![0u8; length];
        self.reader.read_exact(&mut buf).await.unwrap();

        String::from_utf8_lossy(&buf).into_owned()
    }

    async fn read_null_terminated_string(&mut self) -> String {
        let mut buf = Vec::new();

        loop {
            let mut byte = [0];
            self.reader.read_exact(&mut byte).await.unwrap();

            if byte[0] == 0 {
                break;
            }

            buf.push(byte[0]);
        }

        String::from_utf8_lossy(&buf).into_owned()
    }

    async fn read_prefixed_length_string(&mut self) -> String {
        let mut length_buf = [0u8; 2];
        self.reader.read_exact(&mut length_buf).await.unwrap();
        let length = u16::from_le_bytes(length_buf) as usize;
        self.read_fixed_length_string(length).await
    }

    async fn read_utf16_le_string(&mut self, length: usize) -> String {
        let mut buf = vec![0u8; length * 2];
        self.reader.read_exact(&mut buf).await.unwrap();

        let utf16_iter = buf
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]));
        String::from_utf16_lossy(&utf16_iter.collect::<Vec<u16>>())
    }

    async fn read_utf16_be_string(&mut self, length: usize) -> String {
        let mut buf = vec![0u8; length * 2];
        self.reader.read_exact(&mut buf).await.unwrap();

        let utf16_iter = buf
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]));
        String::from_utf16_lossy(&utf16_iter.collect::<Vec<u16>>())
    }
}
