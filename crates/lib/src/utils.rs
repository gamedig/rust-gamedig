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

/// Run gather_fn based on the value of gather_toggle.
///
/// # Parameters
/// - `gather_toggle` should be an expression resolving to a
///   [crate::protocols::types::GatherToggle].
/// - `gather_fn` should be an expression that returns a [crate::GDResult].
///
/// # States
/// - [DontGather](crate::protocols::types::GatherToggle::DontGather) - Don't
///   run gather function, returns None.
/// - [AttemptGather](crate::protocols::types::GatherToggle::AttemptGather) -
///   Runs the gather function, if it returns an error return None, else return
///   Some.
/// - [Required](crate::protocols::types::GatherToggle::Required) - Runs the
///   gather function, if it returns an error propagate it using the `?`
///   operator, else return Some.
///
/// # Examples
///
/// ```ignore,Doctests cannot access private items
/// use gamedig::protocols::types::GatherToggle;
/// use gamedig::utils::maybe_gather;
///
/// let query_fn = || { Err("Query error") };
///
/// // query_fn() is not called
/// let response = maybe_gather!(GatherToggle::DontGather, query_fn());
/// assert!(response.is_none());
///
/// // query_fn() is called but Err is converted to None
/// let response = maybe_gather!(GatherToggle::AttemptGather, query_fn());
/// assert!(response.is_none());
///
/// // query_fn() is called and Err is propagated.
/// let response = maybe_gather!(GatherToggle::Required, query_fn());
/// unreachable!();
/// ```
macro_rules! maybe_gather {
    ($gather_toggle: expr, $gather_fn: expr) => {
        match $gather_toggle {
            crate::protocols::types::GatherToggle::DontGather => None,
            crate::protocols::types::GatherToggle::AttemptGather => $gather_fn.ok(),
            crate::protocols::types::GatherToggle::Required => Some($gather_fn?),
        }
    };
}

pub(crate) use maybe_gather;

#[cfg(test)]
mod tests {
    use super::retry_on_timeout;
    use crate::{
        protocols::types::GatherToggle,
        GDError,
        GDErrorKind::{self, PacketBad, PacketReceive, PacketSend},
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

    fn gather_success(n: i32) -> GDResult<i32> { Ok(n) }

    fn gather_fail(err: &'static str) -> GDResult<i32> { Err(GDErrorKind::PacketSend.context(err)) }

    #[test]
    fn gather_success_dont_gather() -> GDResult<()> {
        let result = maybe_gather!(GatherToggle::DontGather, gather_success(5));
        assert!(result.is_none());
        Ok(())
    }

    #[test]
    fn gather_success_attempt_gather() -> GDResult<()> {
        let result = maybe_gather!(GatherToggle::AttemptGather, gather_success(10));
        assert_eq!(result, Some(10));
        Ok(())
    }

    #[test]
    fn gather_success_required() -> GDResult<()> {
        let result = maybe_gather!(GatherToggle::Required, gather_success(15));
        assert_eq!(result, Some(15));
        Ok(())
    }

    #[test]
    fn gather_fail_dont_gather() -> GDResult<()> {
        let result = maybe_gather!(GatherToggle::DontGather, gather_fail("dont"));
        assert!(result.is_none());
        Ok(())
    }

    #[test]
    fn gather_fail_attempt_gather() -> GDResult<()> {
        let result = maybe_gather!(GatherToggle::AttemptGather, gather_fail("attempt"));
        assert!(result.is_none());
        Ok(())
    }

    #[test]
    fn gather_fail_required() {
        let inner = || {
            let result = maybe_gather!(GatherToggle::Required, gather_fail("required"));
            assert_eq!(result, Some(10));
            Ok::<(), GDError>(())
        };
        assert!(inner().is_err());
    }
}
