mod timeout;

pub use timeout::Timeout;

#[derive(Debug, Default)]
pub struct Settings {
    pub timeout: Timeout,
}
