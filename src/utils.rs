use std::ops::Add;

pub fn concat_u8(first: &[u8], second: &[u8]) -> Vec<u8> {
    [first, second].concat()
}

pub fn find_first_null(arr: &[u8]) -> usize {
    match arr.iter().position(|&x| x == 0) {
        None => arr.len(),
        Some(position) => position
    }
}

pub fn find_first_string(arr: &[u8]) -> String {
    arr.iter().take_while(|&&b| b != 0).map(|&e| e as char).collect::<String>()
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

    pub fn get_u8(buf: &[u8], pos: usize) -> (u8, usize) {
        (buf[pos], pos + 1)
    }

    pub fn get_u16(buf: &[u8], pos: usize) -> (u16, usize) {
        (combine_two_u8(buf[pos + 1], buf[pos]), pos + 2)
    }

    pub fn get_u64(buf: &[u8], pos: usize) -> (u64, usize) {
        (combine_eight_u8(buf[pos + 7], buf[pos + 6], buf[pos + 5], buf[pos + 4], buf[pos + 3], buf[pos + 2], buf[pos + 1], buf[pos]), pos + 8)
    }

    pub fn get_string(buf: &[u8], pos: usize) -> (String, usize) {
        let string = find_first_string(&buf[pos..]);
        let string_size = string.len();
        (string, pos + string_size + 1)
    }
}

pub fn get_u64_from_buf(buf: &[u8]) -> u64 {
    combine_eight_u8(buf[7], buf[6], buf[5], buf[4], buf[3], buf[2], buf[1], buf[0])
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
    fn find_null_in_array_test() {
        let arr: [u8; 4] = [0x64, 0x32, 0x00, 0x20];
        assert_eq!(2, find_first_null(&arr));
    }

    #[test]
    fn complete_address_test() {
        let address = "192.168.0.1";
        let port = 27015;
        assert_eq!(complete_address(address, port), "192.168.0.1:27015");
    }
}
