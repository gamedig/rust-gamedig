use {
    super::super::super::{Headers, Payload, Query},
    crate::error::{NetworkError, Report, Result},
    reqwest::Client,
    serde::de::DeserializeOwned,
    std::time::Duration,
};

pub struct TokioHttpClient {
    client: Client,
}

#[maybe_async::async_impl]
impl super::AbstractHttp for TokioHttpClient {
    async fn new(timeout: Duration) -> Result<Self> {
        let client = Client::builder().timeout(timeout).build().map_err(|e| {
            Report::from(e).change_context(NetworkError::HttpReqwestClientError {}.into())
        })?;

        Ok(Self { client })
    }

    async fn get<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
    ) -> Result<T> {
        let mut req = self.client.get(url);

        if let Some(query) = query {
            req = req.query(query);
        }

        if let Some(headers) = headers {
            for &(k, v) in headers {
                req = req.header(k, v);
            }
        }

        let resp = req.send().await.map_err(|e| {
            Report::from(e).change_context(NetworkError::HttpReqwestClientError {}.into())
        })?;

        if !resp.status().is_success() {
            todo!()
        }

        let value = resp.json::<T>().await.map_err(|e| {
            Report::from(e).change_context(NetworkError::HttpReqwestClientError {}.into())
        })?;

        Ok(value)
    }

    async fn post<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
        payload: Option<Payload<'a>>,
    ) -> Result<T> {
        let mut req = self.client.post(url);

        if let Some(query) = query {
            req = req.query(query);
        }

        if let Some(headers) = headers {
            for &(k, v) in headers {
                req = req.header(k, v);
            }
        }

        if let Some(p) = payload {
            match p {
                Payload::Json(j) => {
                    req = req.json(j);
                }
                Payload::Form(f) => {
                    req = req.form(f);
                }
            }
        }

        let resp = req.send().await.map_err(|e| {
            Report::from(e).change_context(NetworkError::HttpReqwestClientError {}.into())
        })?;

        if !resp.status().is_success() {
            todo!()
        }

        let value = resp.json::<T>().await.map_err(|e| {
            Report::from(e).change_context(NetworkError::HttpReqwestClientError {}.into())
        })?;

        Ok(value)
    }
}
