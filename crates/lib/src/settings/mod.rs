mod timeout;

pub use timeout::Timeout;

pub struct Settings {
    pub timeout: Timeout,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            timeout: Timeout::default(),
        }
    }
}
