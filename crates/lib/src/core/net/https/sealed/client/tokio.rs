use std::collections::HashMap;
use std::time::Duration;

use error_stack::ResultExt;
use reqwest::{Client, Url};
use serde_json::Value;

pub type Headers = HashMap<String, String>;
pub type Query = HashMap<String, String>;
pub type Form = HashMap<String, String>;

pub enum Payload {
    Json(Value),
    Form(Form),
}

pub struct TokioHttpsClient {
    client: Client,
    base_url: Url,
}

impl TokioHttpsClient {
    pub fn new(host_addr: &str, timeout: Duration) -> crate::error::Result<Self> {
        let base_url = Url::parse(host_addr).change_context(todo!())?;

        let client = Client::builder()
            .timeout(timeout)
            .build()
            .change_context(todo!())?;

        Ok(Self { client, base_url })
    }

    fn url_for(&self, path: &str) -> crate::error::Result<Url> {
        self.base_url.join(path).change_context(todo!())
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        headers: &Headers,
        query: &Query,
    ) -> crate::error::Result<T> {
        let url = self.url_for(path)?;
        let mut request = self.client.get(url);

        for (k, v) in headers {
            request = request.header(k, v);
        }

        if !query.is_empty() {
            request = request.query(query);
        }

        let response = request.send().await.change_context(todo!())?;

        let status = response.status();
        let json = response.json::<T>().await.change_context(todo!())?;

        if !status.is_success() {
            todo!()
        }

        Ok(json)
    }

    pub async fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        headers: &Headers,
        query: &Query,
        payload: Option<Payload>,
    ) -> crate::error::Result<T> {
        let url = self.url_for(path)?;
        let mut request = self.client.post(url);

        for (k, v) in headers {
            request = request.header(k, v);
        }

        if !query.is_empty() {
            request = request.query(query);
        }

        if let Some(p) = payload {
            match p {
                Payload::Json(j) => request = request.json(&j),
                Payload::Form(f) => request = request.form(&f),
            }
        }

        let response = request.send().await.change_context(todo!())?;

        let status = response.status();
        let json = response.json::<T>().await.change_context(todo!())?;

        if !status.is_success() {
            todo!()
        }

        Ok(json)
    }
}
