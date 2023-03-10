use crate::GDError::InvalidInput;
use crate::GDResult;
use std::time::Duration;

/// Timeout settings for socket operations
#[derive(Clone, Debug)]
pub struct TimeoutSettings {
    read: Option<Duration>,
    write: Option<Duration>,
}

impl TimeoutSettings {
    /// Construct new settings, passing None will block indefinitely. Passing zero Duration throws GDError::[InvalidInput](InvalidInput).
    pub fn new(read: Option<Duration>, write: Option<Duration>) -> GDResult<Self> {
        if let Some(read_duration) = read {
            if read_duration == Duration::new(0, 0) {
                return Err(InvalidInput);
            }
        }

        if let Some(write_duration) = write {
            if write_duration == Duration::new(0, 0) {
                return Err(InvalidInput);
            }
        }

        Ok(Self { read, write })
    }

    /// Get the read timeout.
    pub fn get_read(&self) -> Option<Duration> {
        self.read
    }

    /// Get the write timeout.
    pub fn get_write(&self) -> Option<Duration> {
        self.write
    }
}

impl Default for TimeoutSettings {
    /// Default values are 4 seconds for both read and write.
    fn default() -> Self {
        Self {
            read: Some(Duration::from_secs(4)),
            write: Some(Duration::from_secs(4)),
        }
    }
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
        let timeout_settings = TimeoutSettings::new(Some(read_duration), Some(write_duration))?;

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

        // Try to create new TimeoutSettings with the zero read duration (this should fail)
        let result = TimeoutSettings::new(Some(read_duration), Some(write_duration));

        // Verify that the function returned an error and that the error type is InvalidInput
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), InvalidInput);
    }

    // Test that the default TimeoutSettings values are correct
    #[test]
    fn test_default_values() {
        // Get the default TimeoutSettings values
        let default_settings = TimeoutSettings::default();

        // Verify that the get_read and get_write methods return the expected default values
        assert_eq!(default_settings.get_read(), Some(Duration::from_secs(4)));
        assert_eq!(default_settings.get_write(), Some(Duration::from_secs(4)));
    }
}
