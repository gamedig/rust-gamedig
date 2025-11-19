use {
    super::super::super::{Headers, Payload, Query},
    crate::error::{Result, ResultExt},
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

    fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        query: Option<&Query>,
        headers: Option<&Headers>,
    ) -> Result<T> {
        let mut req = self.agent.get(url);

        if let Some(query) = query {
            for (k, v) in query {
                req = req.query(k, v);
            }
        }

        if let Some(headers) = headers {
            for (k, v) in headers {
                req = req.header(k, v);
            }
        }

        let mut resp = req.call().change_context(todo!())?;

        let value = resp.body_mut().read_json::<T>().change_context(todo!())?;

        Ok(value)
    }

    fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        query: Option<&Query>,
        headers: Option<&Headers>,
        payload: Option<Payload>,
    ) -> Result<T> {
        let mut req = self.agent.post(url);

        if let Some(query) = query {
            for (k, v) in query {
                req = req.query(k, v);
            }
        }

        if let Some(headers) = headers {
            for (k, v) in headers {
                req = req.header(k, v);
            }
        }

        let mut resp = match payload {
            None => req.send_empty().change_context(todo!())?,
            Some(Payload::Json(j)) => req.send_json(j).change_context(todo!())?,
            Some(Payload::Form(f)) => req.send_form(f).change_context(todo!())?,
        };

        let value = resp.body_mut().read_json::<T>().change_context(todo!())?;

        Ok(value)
    }
}
