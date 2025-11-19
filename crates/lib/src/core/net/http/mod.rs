use crate::error::Result;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

mod sealed;

use sealed::client::AbstractHttp;

// todo: figure out what types are best for these
pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;
pub type Form<'a> = HashMap<&'a str, &'a str>;

pub enum Payload<'a> {
    Json(&'a Value),
    Form(&'a Form<'a>),
}

//todo: docs
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

    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        query: Option<&Query<'_>>,
        headers: Option<&Headers>,
    ) -> Result<T> {
        self.client.inner.get(url, query, headers).await
    }

    pub(crate) async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        query: Option<&Query<'_>>,
        headers: Option<&Headers>,
        payload: Option<Payload<'_>>,
    ) -> Result<T> {
        self.client.inner.post(url, query, headers, payload).await
    }
}
