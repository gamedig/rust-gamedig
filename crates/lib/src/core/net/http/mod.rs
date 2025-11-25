use crate::error::Result;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;

mod sealed;

use sealed::client::AbstractHttp;

// todo: figure out what types are best for these
pub type Headers<'a> = &'a [(&'static str, &'a str)];
pub type Query<'a> = &'a [(&'static str, &'a str)];
pub type Form<'a> = &'a [(&'static str, &'a str)];

pub enum Payload<'a> {
    Json(&'a Value),
    Form(Form<'a>),
}

// todo: docs
// Supports both HTTPS and HTTP
pub(crate) struct HttpClient {
    client: sealed::client::Inner,
}

#[maybe_async::maybe_async]
impl HttpClient {
    pub(crate) async fn new(timeout: Duration) -> Result<Self> {
        Ok(Self {
            client: sealed::client::Inner::new(timeout).await?,
        })
    }

    pub(crate) async fn get<'a, T: DeserializeOwned>(
        &self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
    ) -> Result<T> {
        self.client.inner.get(url, query, headers).await
    }

    pub(crate) async fn post<'a, T: DeserializeOwned>(
        &self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
        payload: Option<Payload<'a>>,
    ) -> Result<T> {
        self.client.inner.post(url, query, headers, payload).await
    }
}
