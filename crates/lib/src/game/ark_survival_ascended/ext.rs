use {
    super::{ArkSurvivalAscendedClient, ArkSurvivalAscendedClientError, MatchmakingSession},
    crate::{
        converters::{
            GenericPlayerExt,
            GenericPlayerWithAdditionalData,
            GenericQueryExt,
            GenericServerExt,
            GenericServerWithAdditionalData,
            GenericTimeoutExt,
            HttpMarker,
        },
        core::error::Report,
    },
    std::net::{IpAddr, SocketAddr},
};

pub struct NonePlayer;

// impl to satisfy trait but never used
impl GenericPlayerExt for NonePlayer {
    type AdditionalPlayerData = ();

    fn into_generic_player_with_additional_data(
        self,
    ) -> GenericPlayerWithAdditionalData<Self::AdditionalPlayerData> {
        GenericPlayerWithAdditionalData {
            id: 0,
            name: None,
            additional_data: (),
        }
    }
}

pub struct AdditionalServerData {
    pub allow_invites: bool,
    pub allow_join_in_progress: bool,
    pub allow_join_via_presence: bool,
    pub address: IpAddr,
    pub address_bound: SocketAddr,
    pub session_name: String,
    pub day_time: u32,
    pub enabled_mods: Vec<u32>,
    pub sotf_match_started: bool,
    pub allow_download_chars: bool,
    pub allow_download_dinos: bool,
    pub allow_download_items: bool,
    pub server_platform_type: Vec<String>,
    pub eos_server_ping: u16,
}

impl GenericServerExt for MatchmakingSession {
    type AdditionalServerData = AdditionalServerData;
    type Player = NonePlayer;

    fn into_generic_server_with_additional_data(
        self,
    ) -> GenericServerWithAdditionalData<Self::AdditionalServerData, Self::Player> {
        GenericServerWithAdditionalData {
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
            anti_cheat: Some(self.attributes.server_uses_battleye),
            has_password: Some(self.attributes.server_password),
            max_players: self.settings.max_public_players as u16,
            current_players: self.total_players as u16,
            players: None,
            additional_data: AdditionalServerData {
                allow_invites: self.settings.allow_invites,
                allow_join_in_progress: self.settings.allow_join_in_progress,
                allow_join_via_presence: self.settings.allow_join_via_presence,
                address: self.attributes.address,
                address_bound: self.attributes.address_bound,
                session_name: self.attributes.session_name,
                day_time: self.attributes.day_time,
                enabled_mods: self.attributes.enabled_mods,
                sotf_match_started: self.attributes.sotf_match_started,
                allow_download_chars: self.attributes.allow_download_chars,
                allow_download_dinos: self.attributes.allow_download_dinos,
                allow_download_items: self.attributes.allow_download_items,
                server_platform_type: self.attributes.server_platform_type,
                eos_server_ping: self.attributes.eos_server_ping,
            },
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
