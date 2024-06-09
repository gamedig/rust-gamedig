use std::time::Duration;

pub struct Timeout {
    pub connect: Duration,
    pub read: Duration,
    pub write: Duration,
    pub retries: u8,
}

impl Timeout {
    pub const DEFAULT_DURATION: Duration = Duration::from_secs(5);
    pub const DEFAULT_RETRIES: u8 = 0;

    pub const fn new(
        connect: Option<Duration>,
        read: Option<Duration>,
        write: Option<Duration>,
        retries: Option<u8>,
    ) -> Self {
        Self {
            connect: match connect {
                Some(d) if Self::is_non_zero(d) => d,
                _ => Self::DEFAULT_DURATION,
            },
            read: match read {
                Some(d) if Self::is_non_zero(d) => d,
                _ => Self::DEFAULT_DURATION,
            },
            write: match write {
                Some(d) if Self::is_non_zero(d) => d,
                _ => Self::DEFAULT_DURATION,
            },
            retries: match retries {
                Some(r) => r,
                None => Self::DEFAULT_RETRIES,
            },
        }
    }

    const fn is_non_zero(duration: Duration) -> bool {
        duration.as_secs() != 0 || duration.subsec_nanos() != 0
    }

    const fn const_default() -> Self {
        Self {
            connect: Self::DEFAULT_DURATION,
            read: Self::DEFAULT_DURATION,
            write: Self::DEFAULT_DURATION,
            retries: Self::DEFAULT_RETRIES,
        }
    }
}

impl Default for Timeout {
    fn default() -> Self { Self::const_default() }
}
