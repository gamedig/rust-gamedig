use super::super::{Body, Header, Query};

use crate::error::Result;

use serde_json::Value;

#[allow(dead_code)]
#[maybe_async::maybe_async]
pub(crate) trait Http {
    async fn get(
        &self,
        url_base: &str,
        url_query: Option<&Query>,
        header: Option<&Header>,
        body: Option<&Body>,
    ) -> Result<Value>;

    async fn post(
        &self,
        url_base: &str,
        url_query: Option<&Query>,
        header: Option<&Header>,
        body: Option<&Body>,
    ) -> Result<Value>;

    async fn put(
        &self,
        url_base: &str,
        url_query: Option<&Query>,
        header: Option<&Header>,
        body: Option<&Body>,
    ) -> Result<Value>;

    async fn delete(
        &self,
        url_base: &str,
        url_query: Option<&Query>,
        header: Option<&Header>,
        body: Option<&Body>,
    ) -> Result<Value>;
}
