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
    fn as_generic_server(&self) -> GenericServer {
        GenericServer {
            players: None,
            data: Some(GenericDataMap::from_iter([
                ("total_players".into(), Some(self.total_players.into())),
                (
                    "allow_invites".into(),
                    Some(self.settings.allow_invites.into()),
                ),
                (
                    "max_public_players".into(),
                    Some(self.settings.max_public_players.into()),
                ),
                (
                    "allow_join_in_progress".into(),
                    Some(self.settings.allow_join_in_progress.into()),
                ),
                (
                    "allow_join_via_presence".into(),
                    Some(self.settings.allow_join_via_presence.into()),
                ),
                ("address".into(), Some(self.attributes.address.into())),
                (
                    "address_bound".into(),
                    Some(self.attributes.address_bound.into()),
                ),
                (
                    "map_name".into(),
                    Some(self.attributes.map_name.clone().into()),
                ),
                (
                    "session_name".into(),
                    Some(self.attributes.session_name.clone().into()),
                ),
                (
                    "server_name".into(),
                    Some(self.attributes.server_name.clone().into()),
                ),
                (
                    "build_id_major".into(),
                    Some(self.attributes.build_id_major.into()),
                ),
                (
                    "build_id_minor".into(),
                    Some(self.attributes.build_id_minor.into()),
                ),
                ("day_time".into(), Some(self.attributes.day_time.into())),
                // enabled_mods => StringList (u32 => String)
                (
                    "enabled_mods".into(),
                    Some(
                        self.attributes
                            .enabled_mods
                            .iter()
                            .map(|id| id.to_string())
                            .collect::<Vec<String>>()
                            .into(),
                    ),
                ),
                (
                    "session_is_pve".into(),
                    Some(self.attributes.session_is_pve.into()),
                ),
                (
                    "sotf_match_started".into(),
                    Some(self.attributes.sotf_match_started.into()),
                ),
                (
                    "allow_download_chars".into(),
                    Some(self.attributes.allow_download_chars.into()),
                ),
                (
                    "allow_download_dinos".into(),
                    Some(self.attributes.allow_download_dinos.into()),
                ),
                (
                    "allow_download_items".into(),
                    Some(self.attributes.allow_download_items.into()),
                ),
                (
                    "server_password".into(),
                    Some(self.attributes.server_password.into()),
                ),
                (
                    "server_platform_type".into(),
                    Some(self.attributes.server_platform_type.clone().into()),
                ),
                (
                    "server_uses_battleye".into(),
                    Some(self.attributes.server_uses_battleye.into()),
                ),
                (
                    "eos_server_ping".into(),
                    Some(self.attributes.eos_server_ping.into()),
                ),
            ])),
        }
    }
}

#[maybe_async::maybe_async]
impl GenericQueryExt for ArkSurvivalAscendedClient {
    type Response = MatchmakingSession;
    type Error = Report<ArkSurvivalAscendedClientError>;

    async fn query_addr(addr: &SocketAddr) -> Result<Self::Response, Self::Error> {
        ArkSurvivalAscendedClient::new().await?.query(addr).await
    }
}
