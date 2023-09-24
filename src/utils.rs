use crate::GDErrorKind::{PacketOverflow, PacketReceive, PacketSend, PacketUnderflow};
use crate::GDResult;
use std::cmp::Ordering;

pub fn error_by_expected_size(expected: usize, size: usize) -> GDResult<()> {
    match size.cmp(&expected) {
        Ordering::Greater => Err(PacketOverflow.into()),
        Ordering::Less => Err(PacketUnderflow.into()),
        Ordering::Equal => Ok(()),
    }
}

pub const fn u8_lower_upper(n: u8) -> (u8, u8) { (n & 15, n >> 4) }

/// Run a closure `retry_count+1` times while it returns [PacketReceive] or
/// [PacketSend] errors, returning the first success, other Error, or after
/// `retry_count+1` tries the last [PacketReceive] or [PacketSend] error.
pub fn retry_on_timeout<T>(mut retry_count: usize, mut fetch: impl FnMut() -> GDResult<T>) -> GDResult<T> {
    let mut last_err = PacketReceive.context("Retry count was 0");
    retry_count += 1;
    while retry_count > 0 {
        last_err = match fetch() {
            Ok(r) => return Ok(r),
            Err(e) if e.kind == PacketReceive || e.kind == PacketSend => e,
            Err(e) => return Err(e),
        };
        retry_count -= 1;
    }
    Err(last_err)
}

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
