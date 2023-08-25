use crate::GDError;

/// Result of Type and GDError.
pub type GDResult<T> = Result<T, GDError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GDErrorKind;

    // Testing Ok variant of the GDResult type
    #[test]
    fn test_gdresult_ok() {
        let result: GDResult<u32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    // Testing Err variant of the GDResult type
    #[test]
    fn test_gdresult_err() {
        let result: GDResult<u32> = Err(GDErrorKind::InvalidInput.into());
        assert!(result.is_err());
    }
}
