use {super::super::request::RequestBuilder, crate::error::Result, serde::de::DeserializeOwned};

#[cfg(feature = "http_tokio")]
mod reqwest;
#[cfg(feature = "http_std")]
mod ureq;

#[maybe_async::maybe_async]
pub(crate) trait AbstractHttp {
    async fn get<T: DeserializeOwned>(&self, request: &RequestBuilder) -> Result<T>;
    async fn post<T: DeserializeOwned>(&self, request: &RequestBuilder) -> Result<T>;
    async fn put<T: DeserializeOwned>(&self, request: &RequestBuilder) -> Result<T>;
    async fn delete<T: DeserializeOwned>(&self, request: &RequestBuilder) -> Result<T>;
}
