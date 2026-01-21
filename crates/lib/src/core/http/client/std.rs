use {
    super::super::{Headers, Payload, Query},
    crate::core::error::{
        Report,
        ResultExt,
        diagnostic::{CRATE_INFO, FailureReason, SYSTEM_INFO},
    },
    serde::de::DeserializeOwned,
    std::time::Duration,
    ureq::Agent,
};

#[derive(Debug, thiserror::Error)]
pub enum StdHttpError {
    #[error("[GameDig]::[HTTP::STD::REQUEST]: Request execution failed")]
    RequestFailed,

    #[error("[GameDig]::[HTTP::STD::DESERIALIZE]: Failed to deserialize response body")]
    Deserialize,
}

pub(crate) struct StdHttpClient {
    agent: Agent,
}

#[maybe_async::sync_impl]
impl super::AbstractHttp for StdHttpClient {
    type Error = Report<StdHttpError>;

    fn new(timeout: Duration) -> Result<Self, Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::HTTP::CLIENT::STD::<NEW>: {:?}", |f| {
            f.debug_struct("Args").field("timeout", &timeout).finish()
        });

        Ok(Self {
            agent: Agent::config_builder()
                .timeout_global(Some(timeout))
                .user_agent(Self::USER_AGENT)
                .build()
                // does not fail, no need to handle an error.
                // infallible would be more correct here but
                // would need to make error more complex than it needs to be
                .into(),
        })
    }

    fn get<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
    ) -> Result<T, Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::HTTP::CLIENT::STD::<GET>: {:?}", |f| {
            f.debug_struct("Args")
                .field("url", &url)
                .field("query", &query)
                .field("headers", &headers)
                .finish()
        });

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

        Ok(req
            .call()
            .change_context(StdHttpError::RequestFailed)
            .attach(FailureReason::new(
                "An error occurred while sending the request",
            ))
            .attach(SYSTEM_INFO)
            .attach(CRATE_INFO)?
            .body_mut()
            .read_json::<T>()
            .change_context(StdHttpError::Deserialize)
            .attach(FailureReason::new(
                "An error occurred while deserializing the response body",
            ))
            .attach(SYSTEM_INFO)
            .attach(CRATE_INFO)?)
    }

    fn post<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
        payload: Option<Payload<'a>>,
    ) -> Result<T, Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::HTTP::CLIENT::STD::<POST>: {:?}", |f| {
            f.debug_struct("Args")
                .field("url", &url)
                .field("query", &query)
                .field("headers", &headers)
                .field("payload", &format_args!("{:?}", payload))
                .finish()
        });

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
                req.send_empty()
                    .change_context(StdHttpError::RequestFailed)
                    .attach(FailureReason::new(
                        "An error occurred while sending the request",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO)?
            }

            Some(Payload::Json(j)) => {
                req.send_json(j)
                    .change_context(StdHttpError::RequestFailed)
                    .attach(FailureReason::new(
                        "An error occurred while sending the request",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO)?
            }

            Some(Payload::Form(f)) => {
                req.send_form(f.iter().copied())
                    .change_context(StdHttpError::RequestFailed)
                    .attach(FailureReason::new(
                        "An error occurred while sending the request",
                    ))
                    .attach(SYSTEM_INFO)
                    .attach(CRATE_INFO)?
            }
        };

        Ok(resp
            .body_mut()
            .read_json::<T>()
            .change_context(StdHttpError::Deserialize)
            .attach(FailureReason::new(
                "An error occurred while deserializing the response body",
            ))
            .attach(SYSTEM_INFO)
            .attach(CRATE_INFO)?)
    }
}
