use {
    super::{ArkSurvivalAscendedClientError, Matchmaking, MatchmakingSession},
    crate::{
        core::{
            ToSocketAddr,
            error::{Report, ResultExt},
        },
        protocol::epic_api::{
            Credentials,
            Criteria,
            CriteriaOp,
            Criterion,
            CriterionKey,
            EpicApiClient,
            RoutingScope,
        },
    },
    std::time::Duration,
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
        timeout: Option<Duration>,
    ) -> Result<Self, Report<ArkSurvivalAscendedClientError>> {
        Ok(Self {
            protocol: EpicApiClient::new_with_timeout(Self::CREDENTIALS, timeout)
                .await
                .change_context(ArkSurvivalAscendedClientError::Init)?,
        })
    }

    pub async fn query<A: ToSocketAddr>(
        &mut self,
        addr: A,
    ) -> Result<MatchmakingSession, Report<ArkSurvivalAscendedClientError>> {
        let addr = addr
            .to_socket_addr()
            .await
            .change_context(ArkSurvivalAscendedClientError::MatchmakingSession)?;

        Ok(self
            .protocol
            .query_as::<Matchmaking>(Criteria {
                criteria: {
                    let mut v = Vec::with_capacity(1);

                    v.push(Criterion {
                        key: CriterionKey::AddressBound,
                        op: CriteriaOp::Equal,
                        value: addr.to_string().into(),
                    });

                    v
                },
            })
            .await
            .change_context(ArkSurvivalAscendedClientError::MatchmakingSession)?
            .session)
    }
}
