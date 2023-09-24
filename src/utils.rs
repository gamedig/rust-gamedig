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
    use super::retry_on_timeout;
    use crate::{
        GDErrorKind::{PacketBad, PacketReceive, PacketSend},
        GDResult,
    };

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

    #[test]
    fn retry_success_on_first() {
        let r = retry_on_timeout(0, || Ok(()));
        assert!(r.is_ok());
    }

    #[test]
    fn retry_no_success() {
        let r: GDResult<()> = retry_on_timeout(100, || Err(PacketSend.context("test")));
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().kind, PacketSend);
    }

    #[test]
    fn retry_success_on_third() {
        let mut i = 0u8;
        let r = retry_on_timeout(2, || {
            i += 1;
            if i < 3 {
                Err(PacketReceive.context("test"))
            } else {
                Ok(())
            }
        });
        assert!(r.is_ok());
    }

    #[test]
    fn retry_success_on_third_but_less_retries() {
        let mut i = 0u8;
        let r = retry_on_timeout(1, || {
            i += 1;
            if i < 3 {
                Err(PacketReceive.context("test"))
            } else {
                Ok(())
            }
        });
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().kind, PacketReceive);
    }

    #[test]
    fn retry_with_non_timeout_error() {
        let mut i = 0u8;
        let r = retry_on_timeout(50, || {
            i += 1;
            match i {
                1 => Err(PacketSend.context("test")),
                2 => Err(PacketBad.context("test")),
                _ => Ok(()),
            }
        });
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().kind, PacketBad);
    }
}
