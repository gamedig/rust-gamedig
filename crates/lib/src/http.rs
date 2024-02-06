use crate::GDErrorKind::{InvalidInput, PacketReceive, PacketSend, ProtocolFormat};
use crate::{GDResult, TimeoutSettings};

use std::io::Read;
use std::net::SocketAddr;

use ureq::{Agent, AgentBuilder};
use url::Url;

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Serialize};

/// Max length of HTTP responses in bytes: 1GB
const MAX_RESPONSE_LENGTH: usize = 1024 * 1024 * 1024;

/// HTTP request client. Define parameters host parameters on new, then re-use
/// for each request.
pub struct HttpClient {
    client: Agent,
    address: Url,
    headers: Vec<(String, String)>,
}

/// HTTP Protocols.
///
/// Note: if the `tls` feature is disabled this will only contain Http.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Protocol {
    #[default]
    Http,
    #[cfg(feature = "tls")]
    Https,
}

impl Protocol {
    /// Convert [Protocol] to a static str for use in a [Url].
    /// e.g. "http:"
    pub const fn as_str(&self) -> &'static str {
        use Protocol::*;
        match self {
            Http => "http:",
            #[cfg(feature = "tls")]
            Https => "https:",
        }
    }
}

/// Additional settings for HTTPClients.
///
/// # Can be created using builder functions:
/// ```ignore, We cannot test private functionality
/// use gamedig::http::{HttpSettings, Protocol};
///
/// let _ = HttpSettings::default()
///   .protocol(Protocol::Http)
///   .hostname(String::from("test.com"))
///   .header(String::from("Authorization"), String::from("Bearer Token"));
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct HttpSettings<S: Into<String>> {
    /// Choose whether to use HTTP or HTTPS.
    pub protocol: Protocol,
    /// Choose a hostname override (used to set the [Host](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Host) header) and for TLS.
    pub hostname: Option<S>,
    /// Choose HTTP headers to send with requests.
    pub headers: Vec<(S, S)>,
}

impl<S: Into<String>> HttpSettings<S> {
    /// Set the HTTP protocol (defaults to HTTP).
    pub const fn protocol(mut self, protocol: Protocol) -> HttpSettings<S> {
        self.protocol = protocol;
        self
    }

    /// Set the desired HTTP host name: used for the HTTP Host header and for
    /// TLS negotiation.
    pub fn hostname(mut self, hostname: S) -> HttpSettings<S> {
        self.hostname = Some(hostname);
        self
    }

    /// Overwrite all the current HTTP headers with new headers.
    pub fn headers(mut self, headers: Vec<(S, S)>) -> HttpSettings<S> {
        self.headers = headers;
        self
    }

    /// Set one HTTP header value.
    pub fn header(mut self, name: S, value: S) -> HttpSettings<S> {
        self.headers.push((name, value));
        self
    }
}

impl HttpClient {
    /// Creates a new HTTPClient that can be used to send requests.
    ///
    /// # Parameters
    /// - [address](SocketAddr): The IP and port the HTTP request will connect
    ///   to.
    /// - [timeout_settings](TimeoutSettings): Used to set the connect and
    ///   socket timeouts for the requests.
    /// - [http_settings](HttpSettings): Additional settings for the HTTPClient.
    pub fn new<S: Into<String>>(
        address: &SocketAddr,
        timeout_settings: &Option<TimeoutSettings>,
        http_settings: HttpSettings<S>,
    ) -> GDResult<Self>
    where
        Self: Sized,
    {
        let mut client_builder = AgentBuilder::new();

        // Set timeout settings
        let (read_timeout, write_timeout) = TimeoutSettings::get_read_and_write_or_defaults(timeout_settings);

        if let Some(read_timeout) = read_timeout {
            client_builder = client_builder.timeout_read(read_timeout);
        }

        if let Some(write_timeout) = write_timeout {
            client_builder = client_builder.timeout_write(write_timeout);
        }

        if let Some(connect_timeout) = TimeoutSettings::get_connect_or_default(timeout_settings) {
            client_builder = client_builder.timeout_connect(connect_timeout);
        }

        // Every request sent from this client will connect to the address set
        {
            let address = *address;
            client_builder = client_builder.resolver(move |_: &str| Ok(vec![address]));
        }

        // Set a friendly user-agent string
        client_builder = client_builder.user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ));

        let client = client_builder.build();

        let host = http_settings
            .hostname
            .map(S::into)
            .unwrap_or(address.ip().to_string());

        Ok(Self {
            client,
            // TODO: Use Url from_parts if it gets added
            address: Url::parse(&format!(
                "{}//{}:{}",
                http_settings.protocol.as_str(),
                host,
                address.port()
            ))
            .map_err(|e| InvalidInput.context(e))?,
            headers: http_settings
                .headers
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        })
    }

    /// Send a HTTP GET request and return the response data as a buffer.
    pub fn get(&mut self, path: &str) -> GDResult<Vec<u8>> { self.request("GET", path) }

    /// Send a HTTP GET request and parse the JSON resonse.
    #[cfg(feature = "serde")]
    pub fn get_json<T: DeserializeOwned>(&mut self, path: &str) -> GDResult<T> { self.request_json("GET", path) }

    /// Send a HTTP Post request with JSON data and parse a JSON response.
    #[cfg(feature = "serde")]
    pub fn post_json<T: DeserializeOwned, S: Serialize>(&mut self, path: &str, data: S) -> GDResult<T> {
        self.request_with_json_data("POST", path, data)
    }

    // NOTE: More methods can be added here as required using the request_json or
    // request_with_json methods

    #[inline]
    fn request(&mut self, method: &str, path: &str) -> GDResult<Vec<u8>> {
        // Append the path to the pre-parsed URL and create a request object.
        self.address.set_path(path);
        let mut request = self.client.request_url(method, &self.address);

        // Set the request headers.
        for (key, value) in self.headers.iter() {
            request = request.set(key, value);
        }

        // Send the request.
        let http_response = request.call().map_err(|e| PacketSend.context(e))?;

        let length = if let Some(length) = http_response.header("Content-Length") {
            length
                .parse::<usize>()
                .map_err(|e| ProtocolFormat.context(e))?
                .min(MAX_RESPONSE_LENGTH)
        } else {
            5012 // Sensible default allocation
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        let _ = http_response
            .into_reader()
            .take(MAX_RESPONSE_LENGTH as u64)
            .read_to_end(&mut buffer)
            .map_err(|e| PacketReceive.context(e))?;

        Ok(buffer)
    }

    /// Send a HTTP request without any data and parse the JSON response.
    #[inline]
    #[cfg(feature = "serde")]
    fn request_json<T: DeserializeOwned>(&mut self, method: &str, path: &str) -> GDResult<T> {
        // Append the path to the pre-parsed URL and create a request object.
        self.address.set_path(path);
        let mut request = self.client.request_url(method, &self.address);

        // Set the request headers.
        for (key, value) in self.headers.iter() {
            request = request.set(key, value);
        }

        // Send the request and parse the response as JSON.
        request
            .call()
            .map_err(|e| PacketSend.context(e))?
            .into_json::<T>()
            .map_err(|e| ProtocolFormat.context(e))
    }

    /// Send a HTTP request with JSON data and parse the JSON response.
    #[inline]
    #[cfg(feature = "serde")]
    fn request_with_json_data<T: DeserializeOwned, S: Serialize>(
        &mut self,
        method: &str,
        path: &str,
        data: S,
    ) -> GDResult<T> {
        self.address.set_path(path);
        let mut request = self.client.request_url(method, &self.address);

        for (key, value) in self.headers.iter() {
            request = request.set(key, value);
        }

        request
            .send_json(data)
            .map_err(|e| PacketSend.context(e))?
            .into_json::<T>()
            .map_err(|e| ProtocolFormat.context(e))
    }
}
