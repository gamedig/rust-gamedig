pub(crate) mod error;

#[cfg(feature = "_HTTP")]
mod http;
#[allow(unused)]
#[cfg(feature = "_HTTP")]
pub(crate) use http::{Form, Headers, HttpClient, Payload, Query};

#[cfg(feature = "_TCP")]
mod tcp;
#[allow(unused)]
#[cfg(feature = "_TCP")]
pub(crate) use tcp::TcpClient;

#[cfg(feature = "_UDP")]
mod udp;
#[cfg(feature = "_UDP")]
pub(crate) use udp::UdpClient;

#[cfg(feature = "_BUFFER")]
mod buffer;
#[cfg(feature = "_BUFFER")]
pub(crate) use buffer::Buffer;
