use {
    super::{ArkSurvivalAscendedClientError, Matchmaking, MatchmakingSession},
    crate::{
        core::error::{Report, ResultExt},
        protocol::epic_api::{
            client::EpicApiClient,
            model::{Credentials, RoutingScope},
        },
    },
    std::{net::SocketAddr, time::Duration},
};

pub struct ArkSurvivalAscendedClient {
    protocol: EpicApiClient,
}

#[maybe_async::maybe_async]
impl ArkSurvivalAscendedClient {
    const CREDENTIALS: Credentials = Credentials {
        id: "xyza7891muomRmynIIHaJB9COBKkwj6n",
        secret: "PP5UGxysEieNfSrEicaD1N2Bb3TdXuD7xHYcsdUHZ7s",
        deployment: "ad9a8feffb3b4b2ca315546f038c3ae2",
        routing_scope: RoutingScope::Wildcard,
    };

    pub async fn new() -> Result<Self, Report<ArkSurvivalAscendedClientError>> {
        Ok(Self {
            protocol: EpicApiClient::new(Self::CREDENTIALS)
                .await
                .change_context(ArkSurvivalAscendedClientError::Init)?,
        })
    }

    pub async fn new_with_timeout(
        timeout: Duration,
    ) -> Result<Self, Report<ArkSurvivalAscendedClientError>> {
        Ok(Self {
            protocol: EpicApiClient::new_with_timeout(Self::CREDENTIALS, timeout)
                .await
                .change_context(ArkSurvivalAscendedClientError::Init)?,
        })
    }

    pub async fn query(
        &mut self,
        addr: &SocketAddr,
    ) -> Result<MatchmakingSession, Report<ArkSurvivalAscendedClientError>> {
        Ok(self
            .protocol
            .query_as::<Matchmaking>(addr)
            .await
            .change_context(ArkSurvivalAscendedClientError::MatchmakingSession)?
            .session)
    }
}
