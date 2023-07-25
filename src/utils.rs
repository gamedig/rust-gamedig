use crate::{GDResult, GDRichError};
use std::cmp::Ordering;

pub fn error_by_expected_size(expected: usize, size: usize) -> GDResult<()> {
    match size.cmp(&expected) {
        Ordering::Greater => Err(GDRichError::packet_underflow(None)),
        Ordering::Less => Err(GDRichError::packet_underflow(None)),
        Ordering::Equal => Ok(()),
    }
}

pub fn u8_lower_upper(n: u8) -> (u8, u8) { (n & 15, n >> 4) }

#[cfg(test)]
mod tests {
    #[test]
    fn u8_lower_upper() {
        assert_eq!(super::u8_lower_upper(171), (11, 10));
    }

    #[test]
    fn error_by_expected_size() {
        assert!(super::error_by_expected_size(69, 69).is_ok());
        assert!(super::error_by_expected_size(69, 68).is_err());
        assert!(super::error_by_expected_size(69, 70).is_err());
    }
}
