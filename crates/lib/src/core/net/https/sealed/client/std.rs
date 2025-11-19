use std::collections::HashMap;
use std::time::Duration;

use error_stack::ResultExt;
use serde::de::DeserializeOwned;
use serde_json::Value;
use ureq::Agent;
use url::Url;

pub type Headers = HashMap<String, String>;
pub type Query = HashMap<String, String>;
pub type Form = HashMap<String, String>;

pub enum Payload {
    Json(Value),
    Form(Form),
}

pub struct UreqHttpsClient {
    agent: Agent,
    base_url: Url,
}

impl UreqHttpsClient {
    pub fn new(host_addr: &str, timeout: Duration) -> crate::error::Result<Self> {
        let base_url = Url::parse(host_addr).change_context(todo!())?;

        let config = Agent::config_builder()
            .timeout_global(Some(timeout))
            .build();

        let agent: Agent = config.into();

        Ok(Self { agent, base_url })
    }

    fn url_for(&self, path: &str) -> crate::error::Result<Url> {
        self.base_url.join(path).change_context(todo!())
    }

    pub fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        headers: &Headers,
        query: &Query,
    ) -> crate::error::Result<T> {
        let url = self.url_for(path)?;
        let mut req = self.agent.get(url.as_str());

        for (k, v) in headers {
            req = req.header(k, v);
        }

        for (k, v) in query {
            req = req.query(k, v);
        }

        let mut resp = req.call().change_context(todo!())?;

        let value = resp.body_mut().read_json::<T>().change_context(todo!())?;

        Ok(value)
    }

    pub fn post<T>(
        &self,
        path: &str,
        headers: &Headers,
        query: &Query,
        payload: Option<Payload>,
    ) -> crate::error::Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.url_for(path)?;
        let mut req = self.agent.post(url.as_str());

        for (k, v) in headers {
            req = req.header(k, v);
        }

        for (k, v) in query {
            req = req.query(k, v);
        }

        let mut resp = match payload {
            None => req.send_empty().change_context(todo!())?,
            Some(Payload::Json(j)) => req.send_json(&j).change_context(todo!())?,
            Some(Payload::Form(f)) => {
                req.send_form(f.iter().map(|(k, v)| (k.as_str(), v.as_str())))
                    .change_context(todo!())?
            }
        };

        let value = resp.body_mut().read_json::<T>().change_context(todo!())?;
        
        Ok(value)
    }
}
