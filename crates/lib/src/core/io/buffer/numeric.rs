use super::{Buffer, MaybeAsyncRead};

#[cfg(feature = "client_std")]
use std::io::Read;
#[cfg(feature = "client_tokio")]
use tokio::io::AsyncReadExt;

//TODO: Add error handling
#[maybe_async::maybe_async]
pub(crate) trait Numeric {
    async fn read_u8(&mut self) -> u8;
    async fn read_i8(&mut self) -> i8;

    async fn read_u16(&mut self) -> u16;
    async fn read_i16(&mut self) -> i16;
    async fn read_u16_le(&mut self) -> u16;
    async fn read_i16_le(&mut self) -> i16;

    async fn read_u32(&mut self) -> u32;
    async fn read_i32(&mut self) -> i32;
    async fn read_u32_le(&mut self) -> u32;
    async fn read_i32_le(&mut self) -> i32;

    async fn read_u64(&mut self) -> u64;
    async fn read_i64(&mut self) -> i64;
    async fn read_u64_le(&mut self) -> u64;
    async fn read_i64_le(&mut self) -> i64;

    async fn read_u128(&mut self) -> u128;
    async fn read_i128(&mut self) -> i128;
    async fn read_u128_le(&mut self) -> u128;
    async fn read_i128_le(&mut self) -> i128;
}

#[maybe_async::async_impl]
impl<R: MaybeAsyncRead> Numeric for Buffer<R> {
    async fn read_u8(&mut self) -> u8 { self.reader.read_u8().await.unwrap() }

    async fn read_i8(&mut self) -> i8 { self.reader.read_i8().await.unwrap() }

    async fn read_u16(&mut self) -> u16 { self.reader.read_u16().await.unwrap() }

    async fn read_i16(&mut self) -> i16 { self.reader.read_i16().await.unwrap() }

    async fn read_u16_le(&mut self) -> u16 { self.reader.read_u16_le().await.unwrap() }

    async fn read_i16_le(&mut self) -> i16 { self.reader.read_i16_le().await.unwrap() }

    async fn read_u32(&mut self) -> u32 { self.reader.read_u32().await.unwrap() }

    async fn read_i32(&mut self) -> i32 { self.reader.read_i32().await.unwrap() }

    async fn read_u32_le(&mut self) -> u32 { self.reader.read_u32_le().await.unwrap() }

    async fn read_i32_le(&mut self) -> i32 { self.reader.read_i32_le().await.unwrap() }

    async fn read_u64(&mut self) -> u64 { self.reader.read_u64().await.unwrap() }

    async fn read_i64(&mut self) -> i64 { self.reader.read_i64().await.unwrap() }

    async fn read_u64_le(&mut self) -> u64 { self.reader.read_u64_le().await.unwrap() }

    async fn read_i64_le(&mut self) -> i64 { self.reader.read_i64_le().await.unwrap() }

    async fn read_u128(&mut self) -> u128 { self.reader.read_u128().await.unwrap() }

    async fn read_i128(&mut self) -> i128 { self.reader.read_i128().await.unwrap() }

    async fn read_u128_le(&mut self) -> u128 { self.reader.read_u128_le().await.unwrap() }

    async fn read_i128_le(&mut self) -> i128 { self.reader.read_i128_le().await.unwrap() }
}

#[maybe_async::sync_impl]
impl<R: MaybeAsyncRead> Numeric for Buffer<R> {
    fn read_u8(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf).unwrap();
        buf[0]
    }

    fn read_i8(&mut self) -> i8 {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf).unwrap();
        buf[0] as i8
    }

    fn read_u16(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).unwrap();
        u16::from_be_bytes(buf)
    }

    fn read_i16(&mut self) -> i16 {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).unwrap();
        i16::from_be_bytes(buf)
    }

    fn read_u16_le(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).unwrap();
        u16::from_le_bytes(buf)
    }

    fn read_i16_le(&mut self) -> i16 {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).unwrap();
        i16::from_le_bytes(buf)
    }

    fn read_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).unwrap();
        u32::from_be_bytes(buf)
    }

    fn read_i32(&mut self) -> i32 {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).unwrap();
        i32::from_be_bytes(buf)
    }

    fn read_u32_le(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).unwrap();
        u32::from_le_bytes(buf)
    }

    fn read_i32_le(&mut self) -> i32 {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).unwrap();
        i32::from_le_bytes(buf)
    }

    fn read_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).unwrap();
        u64::from_be_bytes(buf)
    }

    fn read_i64(&mut self) -> i64 {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).unwrap();
        i64::from_be_bytes(buf)
    }

    fn read_u64_le(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).unwrap();
        u64::from_le_bytes(buf)
    }

    fn read_i64_le(&mut self) -> i64 {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).unwrap();
        i64::from_le_bytes(buf)
    }

    fn read_u128(&mut self) -> u128 {
        let mut buf = [0u8; 16];
        self.reader.read_exact(&mut buf).unwrap();
        u128::from_be_bytes(buf)
    }

    fn read_i128(&mut self) -> i128 {
        let mut buf = [0u8; 16];
        self.reader.read_exact(&mut buf).unwrap();
        i128::from_be_bytes(buf)
    }

    fn read_u128_le(&mut self) -> u128 {
        let mut buf = [0u8; 16];
        self.reader.read_exact(&mut buf).unwrap();
        u128::from_le_bytes(buf)
    }

    fn read_i128_le(&mut self) -> i128 {
        let mut buf = [0u8; 16];
        self.reader.read_exact(&mut buf).unwrap();
        i128::from_le_bytes(buf)
    }
}
