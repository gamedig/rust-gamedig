use std::time::Duration;
use crate::GDResult;
use crate::GDError::InvalidInput;

/// Timeout settings for socket operations
#[derive(Clone)]
pub struct TimeoutSettings {
    read: Option<Duration>,
    write: Option<Duration>
}

impl TimeoutSettings {
    /// Construct new settings, passing None will block indefinitely. Passing zero Duration throws GDError::[InvalidInput](InvalidInput).
    pub fn new(read: Option<Duration>, write: Option<Duration>) -> GDResult<Self> {
        if let Some(read_duration) = read {
            if read_duration == Duration::new(0, 0) {
                return Err(InvalidInput)
            }
        }

        if let Some(write_duration) = write {
            if write_duration == Duration::new(0, 0) {
                return Err(InvalidInput)
            }
        }

        Ok(Self {
            read,
            write
        })
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
            write: Some(Duration::from_secs(4))
        }
    }
}
