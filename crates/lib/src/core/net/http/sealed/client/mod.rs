use serde::de::DeserializeOwned;

use crate::{
    core::net::http::{Headers, Payload, Query},
    error::Result,
};
use ::std::time::Duration;

#[cfg(feature = "http_std")]
mod std;
#[cfg(feature = "http_tokio")]
mod tokio;

#[maybe_async::maybe_async]
pub(crate) trait AbstractHttp {
    async fn new(timeout: Duration) -> Result<Self>
    where Self: Sized;

    async fn get<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
    ) -> Result<T>;

    async fn post<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
        payload: Option<Payload<'a>>,
    ) -> Result<T>;
}

pub(crate) struct Inner {
    #[cfg(feature = "http_std")]
    pub(crate) inner: std::StdHttpClient,
    #[cfg(feature = "http_tokio")]
    pub(crate) inner: tokio::TokioHttpClient,
}

#[maybe_async::maybe_async]
impl Inner {
    pub(crate) async fn new(timeout: Duration) -> Result<Self> {
        #[cfg(feature = "http_std")]
        {
            Ok(Self {
                inner: std::StdHttpClient::new(timeout).await?,
            })
        }

        #[cfg(feature = "http_tokio")]
        {
            Ok(Self {
                inner: tokio::TokioHttpClient::new(timeout).await?,
            })
        }
    }
}
