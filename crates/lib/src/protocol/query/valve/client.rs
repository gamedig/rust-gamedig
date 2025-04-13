use std::{net::SocketAddr, time::Duration};

use crate::{core::UdpClient, error::Result};

/// Valve Query Protocol configuration options.
pub struct ValveQueryConfig {
    /// Whether to include additional player information in the query response.
    pub include_players: bool,

    /// Whether to include server rules in the query response.
    pub include_rules: bool,
}

impl Default for ValveQueryConfig {
    fn default() -> Self {
        Self {
            include_players: true,
            include_rules: true,
        }
    }
}

/// A client for querying Valve game servers using the Valve Query Protocol.
pub struct ValveQueryClient {
    /// The underlying network client
    net: UdpClient,
    /// The configuration for the Valve Query client.
    config: ValveQueryConfig,
}

#[maybe_async::maybe_async]
impl ValveQueryClient {
    pub async fn new(
        address: &SocketAddr,
        read_timeout: Option<&Duration>,
        write_timeout: Option<&Duration>,
    ) -> Result<Self> {
        Ok(Self {
            net: UdpClient::new(&address, read_timeout, write_timeout).await?,
            config: ValveQueryConfig::default(),
        })
    }

    pub fn set_config(&mut self, config: ValveQueryConfig) { self.config = config; }

    pub async fn query() -> () {
        todo!();
    }
}
