use {
    super::{Headers, Payload, Query},
    ::std::time::Duration,
    serde::de::DeserializeOwned,
};

#[cfg(feature = "http_std")]
mod std;
#[cfg(feature = "http_tokio")]
mod tokio;

#[cfg(feature = "http_std")]
pub(crate) type InnerHttpClient = std::StdHttpClient;

#[cfg(feature = "http_tokio")]
pub(crate) type InnerHttpClient = tokio::TokioHttpClient;

#[maybe_async::maybe_async]
pub(crate) trait AbstractHttp {
    type Error;

    const USER_AGENT: &str = concat!(
        "GameDig/",
        env!("CARGO_PKG_VERSION"),
        " (https://github.com/gamedig/rust-gamedig)"
    );

    async fn new(timeout: Duration) -> Result<Self, Self::Error>
    where Self: Sized;

    async fn get<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
    ) -> Result<T, Self::Error>;

    async fn post<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
        payload: Option<Payload<'a>>,
    ) -> Result<T, Self::Error>;
}
