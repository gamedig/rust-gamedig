use crate::GDErrorKind::{InvalidInput, PacketSend, ProtocolFormat};
use crate::{GDResult, TimeoutSettings};

use std::net::SocketAddr;

use ureq::{Agent, AgentBuilder};
use url::Url;

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Serialize};

/// HTTP request client. Define parameters host parameters on new, then re-use
/// for each request.
pub struct HttpClient {
    client: Agent,
    address: Url,
}

/// HTTP Protocols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Protocol {
    #[default]
    HTTP,
    #[cfg(feature = "tls")]
    HTTPS,
}

impl Protocol {
    /// Convert [Protocol] to a static str for use in a [Url].
    /// e.g. "http:"
    pub const fn as_str(&self) -> &'static str {
        use Protocol::*;
        match self {
            HTTP => "http:",
            #[cfg(feature = "tls")]
            HTTPS => "https:",
        }
    }
}

/// Additional settings for HTTPClients.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct HTTPSettings {
    /// Choose whether to use HTTP or HTTPS.
    pub protocol: Protocol,
    /// Choose a hostname override (used to set the [Host](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Host) header) and for TLS.
    pub hostname: Option<String>,
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
    pub fn new(
        address: &SocketAddr,
        timeout_settings: &Option<TimeoutSettings>,
        http_settings: HTTPSettings,
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

        let host = http_settings.hostname.unwrap_or(address.ip().to_string());

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
        })
    }

    /// Send a HTTP GET request and parse the JSON resonse.
    #[cfg(feature = "serde")]
    pub fn get_json<T: DeserializeOwned>(&mut self, path: &str) -> GDResult<T> { self.request_json("GET", path) }

    /// Send a HTTP Post request with JSON data and parse a JSON response.
    #[cfg(feature = "serde")]
    pub fn post_json<T: DeserializeOwned, S: Serialize>(&mut self, path: &str, data: S) -> GDResult<T> {
        self.request_with_json_data("POST", path, data)
    }

    // NOTE: More methods can be added here as required

    /// Send a HTTP request without any data and parse the JSON response.
    #[inline]
    #[cfg(feature = "serde")]
    fn request_json<T: DeserializeOwned>(&mut self, method: &str, path: &str) -> GDResult<T> {
        self.address.set_path(path);
        self.client
            .request_url(method, &self.address)
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
        self.client
            .request_url(method, &self.address)
            .send_json(data)
            .map_err(|e| PacketSend.context(e))?
            .into_json::<T>()
            .map_err(|e| ProtocolFormat.context(e))
    }
}
