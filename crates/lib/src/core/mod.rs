mod io;
mod net;

#[allow(unused_imports)]
#[cfg(feature = "_BUFFER")]
pub(crate) use io::buf::Buffer;

#[allow(unused_imports)]
#[cfg(feature = "_HTTP")]
pub(crate) use net::http::{Form, Headers, HttpClient, Payload, Query};

#[allow(unused_imports)]
#[cfg(feature = "_TCP")]
pub(crate) use net::tcp::TcpClient;

#[allow(unused_imports)]
#[cfg(feature = "_UDP")]
pub(crate) use net::udp::UdpClient;
