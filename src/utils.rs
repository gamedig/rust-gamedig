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
    ((high as u16) << 8) | low as u16
}
