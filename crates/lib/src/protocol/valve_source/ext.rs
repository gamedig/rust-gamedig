use {
    super::{
        ExtraData,
        ExtraDataFlag,
        Player,
        Server,
        ServerEnvironment,
        ServerType,
        TheShip,
        TheShipPlayer,
        ValveSourceClient,
        ValveSourceClientError,
    },
    crate::{
        converters::{
            GenericPlayerExt,
            GenericPlayerWithAdditionalData,
            GenericQueryExt,
            GenericServerExt,
            GenericServerWithAdditionalData,
            GenericTimeoutExt,
            UdpMarker,
        },
        core::error::Report,
    },
    std::{collections::HashMap, net::SocketAddr},
};

pub struct AdditionalPlayerData {
    pub score: i32,
    pub duration: f32,
    pub the_ship: Option<TheShipPlayer>,
}

impl GenericPlayerExt for Player {
    type AdditionalPlayerData = AdditionalPlayerData;

    fn into_generic_player_with_additional_data(
        self,
    ) -> GenericPlayerWithAdditionalData<Self::AdditionalPlayerData> {
        GenericPlayerWithAdditionalData {
            id: self.index as u16,
            name: if self.name.is_empty() {
                None
            } else {
                Some(self.name)
            },

            additional_data: AdditionalPlayerData {
                score: self.score,
                duration: self.duration,
                the_ship: self.the_ship,
            },
        }
    }
}

pub struct AdditionalServerData {
    pub protocol: u8,
    pub folder: String,
    pub game: String,
    pub app_id: u16,
    pub bots: u8,
    pub server_type: ServerType,
    pub environment: ServerEnvironment,
    pub the_ship: Option<TheShip>,
    pub edf: ExtraDataFlag,
    pub extra_data: ExtraData,
    pub rules: Option<HashMap<String, String>>,
}

impl GenericServerExt for Server {
    type AdditionalServerData = AdditionalServerData;
    type Player = Player;

    fn into_generic_server_with_additional_data(
        self,
    ) -> GenericServerWithAdditionalData<Self::AdditionalServerData, Self::Player> {
        GenericServerWithAdditionalData {
            name: self.info.name,
            description: None,
            map: Some(self.info.map),
            mode: None,
            version: Some(self.info.version),
            anti_cheat: Some(self.info.vac_enabled),
            has_password: Some(self.info.password_protected),
            max_players: self.info.max_players as u16,
            current_players: self.info.players as u16,
            players: self.players,
            additional_data: AdditionalServerData {
                protocol: self.info.protocol,
                folder: self.info.folder,
                game: self.info.game,
                app_id: self.info.app_id,
                bots: self.info.bots,
                server_type: self.info.server_type,
                environment: self.info.environment,
                the_ship: self.info.the_ship,
                edf: self.info.edf,
                extra_data: self.info.extra_data,
                rules: self.rules,
            },
        }
    }
}

#[maybe_async::maybe_async]
impl GenericQueryExt for ValveSourceClient {
    type Response = Server;
    type Error = ValveSourceClientError;
    type Timeout = UdpMarker;

    async fn query_addr(addr: &SocketAddr) -> Result<Self::Response, Report<Self::Error>> {
        let mut client: ValveSourceClient = ValveSourceClient::new(addr).await?;

        client.query().await
    }

    async fn query_addr_with_timeout(
        addr: &SocketAddr,
        timeout: impl GenericTimeoutExt<Self::Timeout>,
    ) -> Result<Self::Response, Report<Self::Error>> {
        let (read_timeout, write_timeout) = timeout.into_marker();

        let mut client: ValveSourceClient =
            ValveSourceClient::new_with_timeout(addr, read_timeout, write_timeout).await?;

        client.query().await
    }
}
