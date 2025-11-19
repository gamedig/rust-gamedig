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

    async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        query: Option<&Query>,
        headers: Option<&Headers>,
    ) -> Result<T>;

    async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        query: Option<&Query>,
        headers: Option<&Headers>,
        payload: Option<Payload<'_>>,
    ) -> Result<T>;
}

pub(crate) struct Inner {
    #[cfg(feature = "http_std")]
    pub inner: std::StdHttpClient,
    #[cfg(feature = "http_tokio")]
    pub inner: tokio::TokioHttpClient,
}
