use std::ops::Add;
use crate::GDError;

pub fn concat_u8(first: &[u8], second: &[u8]) -> Vec<u8> {
    [first, second].concat()
}

pub fn find_first_string(arr: &[u8]) -> String {
    std::str::from_utf8(&arr[..arr.iter().position(|&x| x == 0).unwrap()]).unwrap().to_string()
}

pub fn complete_address(address: &str, port: u16) -> String {
    String::from(address.to_owned() + ":").add(&*port.to_string())
}

pub fn combine_two_u8(high: u8, low: u8) -> u16 {
    u16::from_be_bytes([high, low])
}

pub fn combine_eight_u8(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u64 {
    u64::from_be_bytes([a, b, c, d, e, f, g, h])
}

pub mod buffer {
    use super::*;

    pub fn get_u8(buf: &[u8], pos: &mut usize) -> Result<u8, GDError> {
        let value = buf[*pos];
        *pos += 1;
        Ok(value)
    }

    pub fn get_u16(buf: &[u8], pos: &mut usize) -> u16 {
        let value = combine_two_u8(buf[*pos + 1], buf[*pos]);
        *pos += 2;
        value
    }

    pub fn get_u64(buf: &[u8], pos: &mut usize) -> u64 {
        let value = combine_eight_u8(buf[*pos + 7], buf[*pos + 6], buf[*pos + 5], buf[*pos + 4], buf[*pos + 3], buf[*pos + 2], buf[*pos + 1], buf[*pos]);
        *pos += 8;
        value
    }

    pub fn get_string(buf: &[u8], pos: &mut usize) -> String {
        let value = find_first_string(&buf[*pos..]);
        *pos += value.len() + 1;
        value
    }
}

#[cfg(test)]
mod utils {
    use super::*;

    #[test]
    fn concat_u8_test() {
        let a: [u8; 2] = [1, 2];
        let b: [u8; 2] = [3, 4];
        let combined = concat_u8(&a, &b);
        assert_eq!(a[0], combined[0]);
        assert_eq!(a[1], combined[1]);
        assert_eq!(b[0], combined[2]);
        assert_eq!(b[1], combined[3]);
    }

    #[test]
    fn complete_address_test() {
        let address = "192.168.0.1";
        let port = 27015;
        assert_eq!(complete_address(address, port), "192.168.0.1:27015");
    }
}
