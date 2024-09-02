use std::time::Duration;

/// A `Timeout` struct represents the various timeout durations and retry
/// attempts that is used for network and I/O operations.
///
/// This struct provides customization options for connection, read, and write
/// timeouts, as well as a retry count. Default values are also provided.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "attribute_serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "attribute_extended_derive", derive(Clone, Copy, Eq, PartialOrd, Ord, Hash))]
pub struct Timeout {
    /// Duration to establish a connection.
    pub connect: Duration,
    /// Duration to complete a read operation.
    pub read: Duration,
    /// Duration to complete a write operation.
    pub write: Duration,
    /// Number of retry attempts.
    pub retries: u8,
}

impl Timeout {
    /// The default duration for timeouts.
    pub const DEFAULT_DURATION: Duration = Duration::from_secs(5);
    /// The default number of retries.
    pub const DEFAULT_RETRIES: u8 = 0;
    /// The default `Timeout` instance.
    pub const DEFAULT: Self = Self {
        connect: Self::DEFAULT_DURATION,
        read: Self::DEFAULT_DURATION,
        write: Self::DEFAULT_DURATION,
        retries: Self::DEFAULT_RETRIES,
    };

    /// Creates a new `Timeout` instance with the specified durations and
    /// retries.
    ///
    /// If any of the durations are `None` or zero, the default duration of `5
    /// seconds` is used. If the number of retries is `None`, it defaults to
    /// `0`.
    ///
    /// # Parameters
    ///
    /// - `connect`: The duration for the connection timeout.
    /// - `read`: The duration for the read timeout.
    /// - `write`: The duration for the write timeout.
    /// - `retries`: The number of retry attempts.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use gamedig::settings::Timeout;
    ///
    /// let timeout = Timeout::new(
    ///     // If 0 the default duration of 5 seconds is used.
    ///     Some(Duration::from_secs(0)),
    ///     Some(Duration::from_secs(15)),
    ///     Some(Duration::from_secs(20)),
    ///     Some(3),
    /// );
    ///
    /// assert_eq!(timeout.connect, Duration::from_secs(5));
    /// assert_eq!(timeout.connect, Timeout::DEFAULT_DURATION);
    /// assert_eq!(timeout.read, Duration::from_secs(15));
    /// assert_eq!(timeout.write, Duration::from_secs(20));
    /// assert_eq!(timeout.retries, 3);
    ///
    /// // Need to define as a constant to use in a const context?
    ///
    /// const TIMEOUT: Timeout = Timeout::new(
    ///     Some(Duration::from_secs(0)),
    ///     Some(Duration::from_secs(15)),
    ///     Some(Duration::from_secs(20)),
    ///     Some(3),
    /// );
    ///
    /// assert_eq!(TIMEOUT.connect, Duration::from_secs(5));
    /// assert_eq!(TIMEOUT.connect, Timeout::DEFAULT_DURATION);
    /// assert_eq!(TIMEOUT.read, Duration::from_secs(15));
    /// assert_eq!(TIMEOUT.write, Duration::from_secs(20));
    /// assert_eq!(TIMEOUT.retries, 3);
    /// ```
    #[inline]
    pub const fn new(
        connect: Option<Duration>,
        read: Option<Duration>,
        write: Option<Duration>,
        retries: Option<u8>,
    ) -> Self {
        Self {
            connect: match connect {
                Some(d) if !d.is_zero() => d,
                _ => Self::DEFAULT_DURATION,
            },
            read: match read {
                Some(d) if !d.is_zero() => d,
                _ => Self::DEFAULT_DURATION,
            },
            write: match write {
                Some(d) if !d.is_zero() => d,
                _ => Self::DEFAULT_DURATION,
            },
            retries: match retries {
                Some(r) => r,
                None => Self::DEFAULT_RETRIES,
            },
        }
    }

    /// Validates that all durations are non-zero, resetting any that are zero
    /// to the default.
    ///
    /// This method returns a new `Timeout` instance with the validated values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use gamedig::settings::Timeout;
    ///
    /// const TIMEOUT: Timeout = Timeout {
    ///     connect: Duration::from_secs(10),
    ///     read: Duration::from_secs(0),
    ///     write: Duration::from_secs(20),
    ///     retries: 3,
    /// }.validate_copy();
    ///
    /// assert_eq!(TIMEOUT.connect, Duration::from_secs(10));
    /// assert_eq!(TIMEOUT.read, Duration::from_secs(5));
    /// assert_eq!(TIMEOUT.read, Timeout::DEFAULT_DURATION);
    /// assert_eq!(TIMEOUT.write, Duration::from_secs(20));
    /// assert_eq!(TIMEOUT.retries, 3);
    /// ```
    pub const fn validate_copy(self) -> Self {
        Self {
            connect: match self.connect.is_zero() {
                true => Self::DEFAULT_DURATION,
                false => self.connect,
            },
            read: match self.read.is_zero() {
                true => Self::DEFAULT_DURATION,
                false => self.read,
            },
            write: match self.write.is_zero() {
                true => Self::DEFAULT_DURATION,
                false => self.write,
            },
            retries: self.retries,
        }
    }

    /// Validates that all durations are non-zero, resetting any that are zero
    /// to the default.
    ///
    /// This method mutates the `Timeout` instance in place.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use gamedig::settings::Timeout;
    ///
    /// let mut timeout = Timeout::new(
    ///     Some(Duration::from_secs(10)),
    ///     Some(Duration::from_secs(0)),
    ///     Some(Duration::from_secs(20)),
    ///     Some(3),
    /// );
    ///
    /// timeout.validate_mut();
    ///
    /// assert_eq!(timeout.connect, Duration::from_secs(10));
    /// assert_eq!(timeout.read, Duration::from_secs(5));
    /// assert_eq!(timeout.read, Timeout::DEFAULT_DURATION);
    /// assert_eq!(timeout.write, Duration::from_secs(20));
    /// assert_eq!(timeout.retries, 3);
    /// ```
    pub fn validate_mut(&mut self) {
        if self.connect.is_zero() {
            self.connect = Self::DEFAULT_DURATION;
        }
        if self.read.is_zero() {
            self.read = Self::DEFAULT_DURATION;
        }
        if self.write.is_zero() {
            self.write = Self::DEFAULT_DURATION;
        }
    }
}

impl Default for Timeout {
    /// Provides a default `Timeout` instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use gamedig::settings::Timeout;
    ///
    /// let timeout = Timeout::default();
    ///
    /// assert_eq!(timeout, Timeout::DEFAULT);
    ///
    /// // DEFAULT_DURATION = 5 seconds
    /// assert_eq!(timeout.connect, Timeout::DEFAULT_DURATION);
    /// assert_eq!(timeout.read, Timeout::DEFAULT_DURATION);
    /// assert_eq!(timeout.write, Timeout::DEFAULT_DURATION);
    /// // DEFAULT_RETRIES = 0
    /// assert_eq!(timeout.retries, Timeout::DEFAULT_RETRIES);
    /// ```
    fn default() -> Self { Self::DEFAULT }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        assert_eq!(Timeout::DEFAULT, Timeout::default());
    }

    #[test]
    fn test_new_with_some_values() {
        const TIMEOUT_CUSTOM: Timeout = Timeout::new(
            Some(Duration::from_secs(10)),
            Some(Duration::from_secs(15)),
            Some(Duration::from_secs(20)),
            Some(3),
        );

        assert_eq!(TIMEOUT_CUSTOM.connect, Duration::from_secs(10));
        assert_eq!(TIMEOUT_CUSTOM.read, Duration::from_secs(15));
        assert_eq!(TIMEOUT_CUSTOM.write, Duration::from_secs(20));
        assert_eq!(TIMEOUT_CUSTOM.retries, 3);
    }

    #[test]
    fn test_new_with_none_values() {
        const TIMEOUT: Timeout = Timeout::new(None, None, None, None);

        assert_eq!(TIMEOUT, Timeout::DEFAULT);
    }

    #[test]
    fn test_new_with_mixed_values() {
        const TIMEOUT: Timeout = Timeout::new(
            Some(Duration::from_secs(10)),
            None,
            Some(Duration::from_secs(20)),
            None,
        );

        assert_eq!(TIMEOUT.connect, Duration::from_secs(10));
        assert_eq!(TIMEOUT.read, Timeout::DEFAULT_DURATION);
        assert_eq!(TIMEOUT.write, Duration::from_secs(20));
        assert_eq!(TIMEOUT.retries, Timeout::DEFAULT_RETRIES);
    }

    #[test]
    fn test_new_with_zero_duration() {
        const TIMEOUT: Timeout = Timeout::new(
            Some(Duration::from_secs(0)),
            Some(Duration::from_secs(0)),
            Some(Duration::from_secs(0)),
            Some(2),
        );

        assert_eq!(TIMEOUT.connect, Timeout::DEFAULT_DURATION);
        assert_eq!(TIMEOUT.read, Timeout::DEFAULT_DURATION);
        assert_eq!(TIMEOUT.write, Timeout::DEFAULT_DURATION);
        assert_eq!(TIMEOUT.retries, 2);
    }

    #[test]
    fn test_validate_copy_with_zero_duration() {
        const TIMEOUT: Timeout = Timeout {
            connect: Duration::from_secs(10),
            read: Duration::from_secs(0),
            write: Duration::from_secs(20),
            retries: 3,
        }
        .validate_copy();

        assert_eq!(TIMEOUT.connect, Duration::from_secs(10));
        assert_eq!(TIMEOUT.read, Timeout::DEFAULT_DURATION);
        assert_eq!(TIMEOUT.write, Duration::from_secs(20));
        assert_eq!(TIMEOUT.retries, 3);
    }

    #[test]
    fn test_validate_mut_with_zero_duration() {
        // Test validate_mut with zero durations.
        let mut timeout = Timeout::new(
            Some(Duration::from_secs(10)),
            Some(Duration::from_secs(0)),
            Some(Duration::from_secs(20)),
            Some(3),
        );

        timeout.validate_mut();
        assert_eq!(timeout.connect, Duration::from_secs(10));
        assert_eq!(timeout.read, Timeout::DEFAULT_DURATION);
        assert_eq!(timeout.write, Duration::from_secs(20));
        assert_eq!(timeout.retries, 3);
    }
}
