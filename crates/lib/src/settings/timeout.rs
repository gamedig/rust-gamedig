use std::time::Duration;

#[derive(Debug)]
pub struct Timeout {
    pub connect: Duration,
    pub read: Duration,
    pub write: Duration,
    pub retries: u8,
}

impl Timeout {
    pub const DEFAULT_DURATION: Duration = Duration::from_secs(5);
    pub const DEFAULT_RETRIES: u8 = 0;
    pub const DEFAULT: Self = Self {
        connect: Self::DEFAULT_DURATION,
        read: Self::DEFAULT_DURATION,
        write: Self::DEFAULT_DURATION,
        retries: Self::DEFAULT_RETRIES,
    };

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let timeout = Timeout::DEFAULT;

        assert_eq!(timeout.connect, Duration::from_secs(5));
        assert_eq!(timeout.read, Duration::from_secs(5));
        assert_eq!(timeout.write, Duration::from_secs(5));
        assert_eq!(timeout.retries, 0);
    }

    #[test]
    fn test_new_with_some_values() {
        let timeout = Timeout::new(
            Some(Duration::from_secs(10)),
            Some(Duration::from_secs(15)),
            Some(Duration::from_secs(20)),
            Some(3),
        );

        assert_eq!(timeout.connect, Duration::from_secs(10));
        assert_eq!(timeout.read, Duration::from_secs(15));
        assert_eq!(timeout.write, Duration::from_secs(20));
        assert_eq!(timeout.retries, 3);
    }

    #[test]
    fn test_new_with_none_values() {
        let timeout = Timeout::new(None, None, None, None);

        assert_eq!(timeout.connect, Duration::from_secs(5));
        assert_eq!(timeout.read, Duration::from_secs(5));
        assert_eq!(timeout.write, Duration::from_secs(5));
        assert_eq!(timeout.retries, 0);
    }

    #[test]
    fn test_new_with_mixed_values() {
        let timeout = Timeout::new(
            Some(Duration::from_secs(10)),
            None,
            Some(Duration::from_secs(20)),
            None,
        );

        assert_eq!(timeout.connect, Duration::from_secs(10));
        assert_eq!(timeout.read, Duration::from_secs(5));
        assert_eq!(timeout.write, Duration::from_secs(20));
        assert_eq!(timeout.retries, 0);
    }

    #[test]
    fn test_new_with_zero_duration() {
        let timeout = Timeout::new(
            Some(Duration::from_secs(0)),
            Some(Duration::from_secs(0)),
            Some(Duration::from_secs(0)),
            Some(2),
        );

        assert_eq!(timeout.connect, Duration::from_secs(5));
        assert_eq!(timeout.read, Duration::from_secs(5));
        assert_eq!(timeout.write, Duration::from_secs(5));
        assert_eq!(timeout.retries, 2);
    }
}
