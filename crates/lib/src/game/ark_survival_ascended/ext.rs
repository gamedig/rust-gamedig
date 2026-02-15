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
            name: self.attributes.server_name,
            description: None,
            map: Some(self.attributes.map_name),
            mode: Some(
                if self.attributes.session_is_pve {
                    "PvE".into()
                } else {
                    "PvP".into()
                },
            ),
            version: Some(format!(
                "v{}.{}",
                self.attributes.build_id_major, self.attributes.build_id_minor,
            )),
            anti_cheat: if self.attributes.server_uses_battleye {
                Some("BattleEye".into())
            } else {
                None
            },
            has_password: Some(self.attributes.server_password),
            max_players: self.settings.max_public_players as u16,
            current_players: self.total_players as u16,
            players: None,
            additional_data: Some(GenericDataMap::from_iter([
                ("allow_invites".into(), self.settings.allow_invites.into()),
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
                ("session_name".into(), self.attributes.session_name.into()),
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
                    "server_platform_type".into(),
                    self.attributes.server_platform_type.into(),
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
