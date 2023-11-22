//! Types useful for all parts of the library.

use std::time::Duration;

use crate::errors::GDErrorKind::InvalidInput;
use crate::GDResult;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Timeout settings for socket operations
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeoutSettings {
    read: Option<Duration>,
    write: Option<Duration>,
    retries: usize,
}

impl TimeoutSettings {
    /// Construct new settings, passing None will block indefinitely.
    /// Passing zero Duration throws GDErrorKind::[InvalidInput].
    ///
    /// The retry count is the number of extra tries once the original request
    /// fails, so a value of "0" will only make a single request, whereas
    /// "1" will try the request again once if it fails.
    /// The retry count is per-request so for multi-request queries (valve) if a
    /// single part fails that part can be retried up to `retries` times.
    pub fn new(read: Option<Duration>, write: Option<Duration>, retries: usize) -> GDResult<Self> {
        if let Some(read_duration) = read {
            if read_duration == Duration::new(0, 0) {
                return Err(InvalidInput.context("Read duration must not be 0"));
            }
        }

        if let Some(write_duration) = write {
            if write_duration == Duration::new(0, 0) {
                return Err(InvalidInput.context("Write duration must not be 0"));
            }
        }

        Ok(Self {
            read,
            write,
            retries,
        })
    }

    /// Get the read timeout.
    pub const fn get_read(&self) -> Option<Duration> { self.read }

    /// Get the write timeout.
    pub const fn get_write(&self) -> Option<Duration> { self.write }

    /// Get number of retries
    pub const fn get_retries(&self) -> usize { self.retries }

    /// Get the number of retries if there are timeout settings else fall back
    /// to the default
    pub const fn get_retries_or_default(timeout_settings: &Option<TimeoutSettings>) -> usize {
        if let Some(timeout_settings) = timeout_settings {
            timeout_settings.get_retries()
        } else {
            TimeoutSettings::const_default().get_retries()
        }
    }

    /// Get the read and write durations if there are timeout settings else fall
    /// back to the defaults
    pub const fn get_read_and_write_or_defaults(
        timeout_settings: &Option<TimeoutSettings>,
    ) -> (Option<Duration>, Option<Duration>) {
        if let Some(timeout_settings) = timeout_settings {
            (timeout_settings.get_read(), timeout_settings.get_write())
        } else {
            let default = TimeoutSettings::const_default();
            (default.get_read(), default.get_write())
        }
    }

    /// Default values are 4 seconds for both read and write, no retries.
    pub const fn const_default() -> Self {
        Self {
            read: Some(Duration::from_secs(4)),
            write: Some(Duration::from_secs(4)),
            retries: 0,
        }
    }
}

impl Default for TimeoutSettings {
    /// Default values are 4 seconds for both read and write, no retries.
    fn default() -> Self { Self::const_default() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // Test creating new TimeoutSettings with valid durations
    #[test]
    fn test_new_with_valid_durations() -> GDResult<()> {
        // Define valid read and write durations
        let read_duration = Duration::from_secs(1);
        let write_duration = Duration::from_secs(2);

        // Create new TimeoutSettings with the valid durations
        let timeout_settings = TimeoutSettings::new(Some(read_duration), Some(write_duration), 0)?;

        // Verify that the get_read and get_write methods return the expected values
        assert_eq!(timeout_settings.get_read(), Some(read_duration));
        assert_eq!(timeout_settings.get_write(), Some(write_duration));

        Ok(())
    }

    // Test creating new TimeoutSettings with a zero duration
    #[test]
    fn test_new_with_zero_duration() {
        // Define a zero read duration and a valid write duration
        let read_duration = Duration::new(0, 0);
        let write_duration = Duration::from_secs(2);

        // Try to create new TimeoutSettings with the zero read duration (this should
        // fail)
        let result = TimeoutSettings::new(Some(read_duration), Some(write_duration), 0);

        // Verify that the function returned an error and that the error type is
        // InvalidInput
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), crate::GDErrorKind::InvalidInput.into());
    }

    // Test that the default TimeoutSettings values are correct
    #[test]
    fn test_default_values() {
        // Get the default TimeoutSettings values
        let default_settings = TimeoutSettings::default();

        // Verify that the get_read and get_write methods return the expected default
        // values
        assert_eq!(default_settings.get_read(), Some(Duration::from_secs(4)));
        assert_eq!(default_settings.get_write(), Some(Duration::from_secs(4)));
    }
}
