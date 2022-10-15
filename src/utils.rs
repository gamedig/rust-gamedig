use std::ops::Add;

pub fn concat_u8(first: &[u8], second: &[u8]) -> Vec<u8> {
    [first, second].concat()
}

pub fn find_null_in_array(arr: &[u8]) -> usize {
    match arr.iter().position(|&x| x == 0) {
        None => arr.len(),
        Some(position) => position
    }
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

pub fn get_u64_from_buf(buf: &[u8]) -> u64 {
    combine_eight_u8(buf[7], buf[6], buf[5], buf[4], buf[3], buf[2], buf[1], buf[0])
}
