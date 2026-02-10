use {
    super::{ArkSurvivalAscendedClient, ArkSurvivalAscendedClientError, MatchmakingSession},
    crate::{
        converters::{
            ErrorCategory,
            ErrorCategoryExt,
            GenericDataMap,
            GenericQueryExt,
            GenericServer,
            GenericServerExt,
            GenericTimeoutExt,
            HttpMarker,
        },
        core::error::Report,
    },

    std::net::SocketAddr,
};

impl ErrorCategoryExt for ArkSurvivalAscendedClientError {
    fn category(&self) -> ErrorCategory {
        match self {
            ArkSurvivalAscendedClientError::Init => ErrorCategory::Init,
            ArkSurvivalAscendedClientError::MatchmakingSession => ErrorCategory::Networking,
        }
    }
}

impl GenericServerExt for MatchmakingSession {
    fn into_generic_server(self) -> GenericServer {
        GenericServer {
            players: None,
            data: Some(GenericDataMap::from_iter([
                ("total_players".into(), self.total_players.into()),
                ("allow_invites".into(), self.settings.allow_invites.into()),
                (
                    "max_public_players".into(),
                    self.settings.max_public_players.into(),
                ),
                (
                    "allow_join_in_progress".into(),
                    self.settings.allow_join_in_progress.into(),
                ),
                (
                    "allow_join_via_presence".into(),
                    self.settings.allow_join_via_presence.into(),
                ),
                ("address".into(), self.attributes.address.into()),
                ("address_bound".into(), self.attributes.address_bound.into()),
                ("map_name".into(), self.attributes.map_name.into()),
                ("session_name".into(), self.attributes.session_name.into()),
                ("server_name".into(), self.attributes.server_name.into()),
                (
                    "build_id_major".into(),
                    self.attributes.build_id_major.into(),
                ),
                (
                    "build_id_minor".into(),
                    self.attributes.build_id_minor.into(),
                ),
                ("day_time".into(), self.attributes.day_time.into()),
                // enabled_mods => StringList (u32 => String)
                (
                    "enabled_mods".into(),
                    self.attributes
                        .enabled_mods
                        .iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<String>>()
                        .into(),
                ),
                (
                    "session_is_pve".into(),
                    self.attributes.session_is_pve.into(),
                ),
                (
                    "sotf_match_started".into(),
                    self.attributes.sotf_match_started.into(),
                ),
                (
                    "allow_download_chars".into(),
                    self.attributes.allow_download_chars.into(),
                ),
                (
                    "allow_download_dinos".into(),
                    self.attributes.allow_download_dinos.into(),
                ),
                (
                    "allow_download_items".into(),
                    self.attributes.allow_download_items.into(),
                ),
                (
                    "server_password".into(),
                    self.attributes.server_password.into(),
                ),
                (
                    "server_platform_type".into(),
                    self.attributes.server_platform_type.into(),
                ),
                (
                    "server_uses_battleye".into(),
                    self.attributes.server_uses_battleye.into(),
                ),
                (
                    "eos_server_ping".into(),
                    self.attributes.eos_server_ping.into(),
                ),
            ])),
        }
    }
}

#[maybe_async::maybe_async]
impl GenericQueryExt for ArkSurvivalAscendedClient {
    type Response = MatchmakingSession;
    type Error = ArkSurvivalAscendedClientError;
    type Timeout = HttpMarker;

    async fn query_addr(addr: &SocketAddr) -> Result<Self::Response, Report<Self::Error>> {
        ArkSurvivalAscendedClient::new().await?.query(addr).await
    }

    async fn query_addr_with_timeout(
        addr: &SocketAddr,
        timeout: impl GenericTimeoutExt<Self::Timeout>,
    ) -> Result<Self::Response, Report<Self::Error>> {
        ArkSurvivalAscendedClient::new_with_timeout(timeout.into_marker())
            .await?
            .query(addr)
            .await
    }
}
