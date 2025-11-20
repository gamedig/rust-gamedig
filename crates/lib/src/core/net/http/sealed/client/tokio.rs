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

    async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        query: Option<&Query>,
        headers: Option<&Headers>,
    ) -> Result<T> {
        let mut req = self.client.get(url);

        if let Some(query) = query {
            req = req.query(query);
        }

        if let Some(headers) = headers {
            for (k, v) in headers {
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

    async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        query: Option<&Query>,
        headers: Option<&Headers>,
        payload: Option<Payload<'_>>,
    ) -> Result<T> {
        let mut req = self.client.post(url);

        if let Some(query) = query {
            req = req.query(query);
        }

        if let Some(headers) = headers {
            for (k, v) in headers {
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
            //todo: unlike ureq, 4xx and 5xx do not produce errors automatically
            todo!()
        }

        let value = resp.json::<T>().await.map_err(|e| {
            Report::from(e).change_context(NetworkError::HttpReqwestClientError {}.into())
        })?;

        Ok(value)
    }
}

