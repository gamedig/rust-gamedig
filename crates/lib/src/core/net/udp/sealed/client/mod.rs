use ::std::net::SocketAddr;

use crate::{error::Result, settings::Timeout};

#[cfg(feature = "client_async_std")]
mod async_std;
#[cfg(feature = "client_std")]
mod std;
#[cfg(feature = "client_tokio")]
mod tokio;

#[allow(dead_code)]
#[maybe_async::maybe_async]
pub(crate) trait Udp {
    //TODO: add correct values
    // Maximum Transmission Unit for Ethernet
    const ETHERNET_MTU: u16 = 65_535;
    // IP and UDP header sizes
    const IP_HEADER_SIZE: u16 = 20;
    const UDP_HEADER_SIZE: u16 = 8;
    // Maximum UDP payload size
    const MAX_UDP_PACKET_SIZE: u16 =
        Self::ETHERNET_MTU - Self::IP_HEADER_SIZE - Self::UDP_HEADER_SIZE;

    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self>
    where Self: Sized;

    async fn send(&mut self, data: &[u8]) -> Result<()>;
    async fn recv(&mut self, size: Option<usize>) -> Result<Vec<u8>>;
}
