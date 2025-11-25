use {
    super::super::super::{Headers, Payload, Query},
    crate::error::{NetworkError, Report, Result},
    serde::de::DeserializeOwned,
    std::time::Duration,
    ureq::Agent,
};

pub struct StdHttpClient {
    agent: Agent,
}

#[maybe_async::sync_impl]
impl super::AbstractHttp for StdHttpClient {
    fn new(timeout: Duration) -> Result<Self> {
        let config = Agent::config_builder()
            .timeout_global(Some(timeout))
            .build();

        Ok(Self {
            agent: config.into(),
        })
    }

    fn get<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
    ) -> Result<T> {
        let mut req = self.agent.get(url);

        if let Some(query) = query {
            for &(k, v) in query {
                req = req.query(k, v);
            }
        }

        if let Some(headers) = headers {
            for &(k, v) in headers {
                req = req.header(k, v);
            }
        }

        let mut resp = req.call().map_err(|e| {
            Report::from(e).change_context(NetworkError::HttpUreqClientError {}.into())
        })?;

        let value = resp.body_mut().read_json::<T>().map_err(|e| {
            Report::from(e).change_context(NetworkError::HttpUreqClientError {}.into())
        })?;

        Ok(value)
    }

    fn post<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
        payload: Option<Payload<'a>>,
    ) -> Result<T> {
        let mut req = self.agent.post(url);

        if let Some(query) = query {
            for &(k, v) in query {
                req = req.query(k, v);
            }
        }

        if let Some(headers) = headers {
            for &(k, v) in headers {
                req = req.header(k, v);
            }
        }

        let mut resp = match payload {
            None => {
                req.send_empty().map_err(|e| {
                    Report::from(e).change_context(NetworkError::HttpUreqClientError {}.into())
                })?
            }
            Some(Payload::Json(j)) => {
                req.send_json(j).map_err(|e| {
                    Report::from(e).change_context(NetworkError::HttpUreqClientError {}.into())
                })?
            }
            Some(Payload::Form(f)) => {
                req.send_form(f.iter().copied()).map_err(|e| {
                    Report::from(e).change_context(NetworkError::HttpUreqClientError {}.into())
                })?
            }
        };

        let value = resp.body_mut().read_json::<T>().map_err(|e| {
            Report::from(e).change_context(NetworkError::HttpUreqClientError {}.into())
        })?;

        Ok(value)
    }
}
