use crate::error::Result;
use ::std::time::Duration;

#[cfg(feature = "https_tokio")]
mod tokio;
#[cfg(feature = "https_std")]
mod std;

#[maybe_async::maybe_async]
pub(crate) trait AbstractHttps {
    async fn new(addr: &str, timeout: Duration) -> Result<Self>
    where Self: Sized;
}
