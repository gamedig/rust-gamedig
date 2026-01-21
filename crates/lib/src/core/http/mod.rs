use {
    crate::core::error::{Report, ResultExt},
    client::{AbstractHttp, InnerHttpClient},
    serde::de::DeserializeOwned,
    serde_json::Value,
    std::time::Duration,
};

mod client;

/// Errors produced by the [`HttpClient`].
///
/// Lower level runtime/transport details (DNS failure, TLS failure, non 2xx
/// status, JSON decode, etc.) are expected to be captured as *attached context*
/// in the [`error_stack::Report`] originating from the inner client.
///
/// Variants map to the stage of the request lifecycle:
/// - [`HttpClientError::Init`]: constructing the inner client.
/// - [`HttpClientError::Get`]: executing a GET or processing its response.
/// - [`HttpClientError::Post`]: executing a POST or processing its response.
#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    /// The inner HTTP backend failed to initialize.
    ///
    /// Typical causes include invalid client configuration, TLS backend issues,
    /// or failures constructing the underlying runtime client.
    #[error("[GameDig]::[HTTP::INIT]: Failed to initialize the HTTP client")]
    Init,

    /// A GET request failed during execution or response processing.
    ///
    /// This includes request send errors, non success HTTP status codes, and
    /// response deserialization failures. See the attached context on the
    /// returned [`Report`] for details.
    #[error("[GameDig]::[HTTP::GET]: request failed during execution or response processing")]
    Get,

    /// A POST request failed during execution or response processing.
    ///
    /// This includes request send errors, non success HTTP status codes, and
    /// response deserialization failures. See the attached context on the
    /// returned [`Report`] for details.
    #[error("[GameDig]::[HTTP::POST]: request failed during execution or response processing")]
    Post,
}

/// Convenience type for HTTP headers.
///
/// The header *names* are `'static` so they can be declared as constants.
/// The header *values* are borrowed and tied to the lifetime `'a`.
///
/// # Example
/// ```ignore
/// let headers: Headers<'_> = &[
///     ("accept", "application/json"),
///     ("authorization", token.as_str()),
/// ];
/// ```
pub(crate) type Headers<'a> = &'a [(&'static str, &'a str)];

/// Convenience type for query string parameters.
///
/// Represented as a slice of `(key, value)` pairs; keys are `'static` and values
/// are borrowed.
///
/// # Example
/// ```ignore
/// let query: Query<'_> = &[("search", "foo"), ("limit", "10")];
/// ```
pub(crate) type Query<'a> = &'a [(&'static str, &'a str)];

/// Convenience type for `application/x-www-form-urlencoded` payloads.
///
/// This shares the same representation as [`Query`], but is separated as a
/// distinct alias to make call sites more descriptive.
/// 
/// # Example
/// ```ignore
/// let form: Form<'_> = &[("username", "user"), ("password", "pass")];
/// ```
pub(crate) type Form<'a> = &'a [(&'static str, &'a str)];

/// Request body payload for requests.
///
/// Only the payload *shape* is modeled here, encoding is performed by the
/// selected runtime backend.
///
/// - [`Payload::Json`] sends JSON using a borrowed [`serde_json::Value`].
/// - [`Payload::Form`] sends form data using `application/x-www-form-urlencoded`.
pub(crate) enum Payload<'a> {
    /// JSON payload (`application/json`), provided as a borrowed [`serde_json::Value`].
    Json(&'a Value),

    /// Form payload (`application/x-www-form-urlencoded`), provided as key/value pairs.
    Form(Form<'a>),
}

/// HTTP client.
///
/// This provides a small, stable API surface over a feature selected
/// runtime implementation (`tokio`, `std`).
///
/// # Supported schemes
/// The client supports both `http://` and `https://` URLs.
pub(crate) struct HttpClient {
    client: InnerHttpClient,
}

#[maybe_async::maybe_async]
impl HttpClient {
    /// Create a new [`HttpClient`].
    ///
    /// If `timeout` is `None`, a default timeout of 30 seconds is used.
    ///
    /// # Parameters
    /// - `timeout`: Maximum duration allowed for a request (connect, tls handshake,
    ///   send, wait for response, read response, deserialize).
    pub(crate) async fn new(timeout: Option<Duration>) -> Result<Self, Report<HttpClientError>> {
        dev_trace_fmt!("GAMEDIG::CORE::HTTP::<NEW>: {:?}", |f| {
            f.debug_struct("Args").field("timeout", &timeout).finish()
        });

        Ok(Self {
            client: InnerHttpClient::new(timeout.unwrap_or(Duration::from_secs(30)))
                .await
                .change_context(HttpClientError::Init)?,
        })
    }

    /// Perform an HTTP GET request and deserialize the JSON response body into `T`.
    ///
    /// # Type Parameters
    /// - `T`: The response type to deserialize from JSON.
    ///
    /// # Parameters
    /// - `url`: Absolute URL to request.
    /// - `query`: Optional query string key/value pairs.
    /// - `headers`: Optional headers to attach to the request.
    pub(crate) async fn get<'a, T: DeserializeOwned>(
        &self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
    ) -> Result<T, Report<HttpClientError>> {
        dev_trace_fmt!("GAMEDIG::CORE::HTTP::<GET>: {:?}", |f| {
            f.debug_struct("Args")
                .field("url", &url)
                .field("query", &query)
                .field("headers", &headers)
                .finish()
        });

        self.client
            .get(url, query, headers)
            .await
            .change_context(HttpClientError::Get)
    }

    /// Perform an HTTP POST request (optionally with a payload) and deserialize
    /// the JSON response body into `T`.
    ///
    /// # Type Parameters
    /// - `T`: The response type to deserialize from JSON.
    ///
    /// # Parameters
    /// - `url`: Absolute URL to request.
    /// - `query`: Optional query string key/value pairs.
    /// - `headers`: Optional headers to attach to the request.
    /// - `payload`: Optional request body payload (JSON or form encoded).
    pub(crate) async fn post<'a, T: DeserializeOwned>(
        &self,
        url: &'a str,
        query: Option<Query<'a>>,
        headers: Option<Headers<'a>>,
        payload: Option<Payload<'a>>,
    ) -> Result<T, Report<HttpClientError>> {
        dev_trace_fmt!("GAMEDIG::CORE::HTTP::<POST>: {:?}", |f| {
            f.debug_struct("Args")
                .field("url", &url)
                .field("query", &query)
                .field("headers", &headers)
                .field("payload", &format_args!("{:?}", payload))
                .finish()
        });

        self.client
            .post(url, query, headers, payload)
            .await
            .change_context(HttpClientError::Post)
    }
}
