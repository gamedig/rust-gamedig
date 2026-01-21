use {
    super::super::{Headers, Payload, Query},
    crate::core::error::{
        Report,
        ResultExt,
        diagnostic::{CRATE_INFO, ContextComponent, FailureReason, SYSTEM_INFO},
    },
    reqwest::Client,
    serde::de::DeserializeOwned,
    std::time::Duration,
};

#[derive(Debug, thiserror::Error)]
pub enum TokioHttpError {
    #[error("[GameDig]::[HTTP::TOKIO::INIT]: failed to initialize reqwest client")]
    Init,

    #[error("[GameDig]::[HTTP::TOKIO::REQUEST]: Request execution failed")]
    RequestFailed,

    #[error("[GameDig]::[HTTP::TOKIO::STATUS]: Response returned a non success status")]
    Status,

    #[error("[GameDig]::[HTTP::TOKIO::DESERIALIZE]: Failed to deserialize response body")]
    Deserialize,
}

pub(crate) struct TokioHttpClient {
    client: Client,
}

#[maybe_async::async_impl]
impl super::AbstractHttp for TokioHttpClient {
    type Error = Report<TokioHttpError>;

    async fn new(timeout: Duration) -> Result<Self, Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::HTTP::CLIENT::TOKIO::<NEW>: {:?}", |f| {
            f.debug_struct("Args").field("timeout", &timeout).finish()
        });

        Ok(Self {
            client: Client::builder()
                .timeout(timeout)
                .user_agent(Self::USER_AGENT)
                .build()
                .change_context(TokioHttpError::Init)
                .attach(FailureReason::new("Failed to build reqwest client"))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO)?,
        })
    }

    async fn get<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
    ) -> Result<T, Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::HTTP::CLIENT::TOKIO::<GET>: {:?}", |f| {
            f.debug_struct("Args")
                .field("url", &url)
                .field("query", &query)
                .field("headers", &headers)
                .finish()
        });

        let mut req = self.client.get(url);

        if let Some(q) = query {
            req = req.query(q);
        }

        if let Some(h) = headers {
            for &(k, v) in h {
                req = req.header(k, v);
            }
        }

        let resp = req
            .send()
            .await
            .change_context(TokioHttpError::RequestFailed)
            .attach(FailureReason::new(
                "An error occurred while sending the request",
            ))
            .attach(SYSTEM_INFO)
            .attach(CRATE_INFO)?;

        let status = resp.status();
        if !status.is_success() {
            return Err(Report::new(TokioHttpError::Status))
                .attach(FailureReason::new("Response returned a non success status"))
                .attach(ContextComponent::new("Status code", status.as_u16()))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO);
        }

        Ok(resp
            .json::<T>()
            .await
            .change_context(TokioHttpError::Deserialize)
            .attach(FailureReason::new(
                "An error occurred while deserializing the response body",
            ))
            .attach(SYSTEM_INFO)
            .attach(CRATE_INFO)?)
    }

    async fn post<'a, T: DeserializeOwned>(
        &'a self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
        payload: Option<Payload<'a>>,
    ) -> Result<T, Self::Error> {
        dev_trace_fmt!("GAMEDIG::CORE::HTTP::CLIENT::TOKIO::<POST>: {:?}", |f| {
            f.debug_struct("Args")
                .field("url", &url)
                .field("query", &query)
                .field("headers", &headers)
                .field("payload", &format_args!("{:?}", payload))
                .finish()
        });

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

        let resp = req
            .send()
            .await
            .change_context(TokioHttpError::RequestFailed)
            .attach(FailureReason::new(
                "An error occurred while sending the request",
            ))
            .attach(SYSTEM_INFO)
            .attach(CRATE_INFO)?;

        let status = resp.status();
        if !status.is_success() {
            return Err(Report::new(TokioHttpError::Status))
                .attach(FailureReason::new("Response returned a non success status"))
                .attach(ContextComponent::new("Status code", status.as_u16()))
                .attach(SYSTEM_INFO)
                .attach(CRATE_INFO);
        }

        Ok(resp
            .json::<T>()
            .await
            .change_context(TokioHttpError::Deserialize)
            .attach(FailureReason::new(
                "An error occurred while deserializing the response body",
            ))
            .attach(SYSTEM_INFO)
            .attach(CRATE_INFO)?)
    }
}
