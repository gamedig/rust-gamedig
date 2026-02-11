use {
    crate::{
        converters::{
            DictMarker,
            GenericQueryExt,
            GenericServer,
            GenericServerExt,
            GenericTimeoutExt,
        },
        core::error::{Report, ResultExt},
        game::ark_survival_ascended::ArkSurvivalAscendedClient,
    },
    std::net::SocketAddr,
};

#[derive(Debug, thiserror::Error)]
pub enum DictError {
    #[error("[GameDig]::[Dict::QUERY]: Failed to query game server")]
    Query,
    #[error("[GameDig]::[Dict::UNKNOWN_IDENTIFIER]: Unknown game identifier provided ({game_id})")]
    UnknownIdentifier { game_id: String },
}

enum SupportedGame {
    #[cfg(feature = "game_ark_survival_ascended")]
    ArkSurvivalAscended,
}

pub struct Dict;

#[maybe_async::maybe_async]
impl Dict {
    fn game_id_lookup(game_id: &str) -> Option<SupportedGame> {
        hashify::tiny_map! {
            game_id.as_bytes(),
            #[cfg(feature = "game_ark_survival_ascended")]
            "ark_survival_ascended" => SupportedGame::ArkSurvivalAscended,

        }
    }

    pub async fn query(
        game_id: &str,
        addr: &SocketAddr,
        timeout: Option<impl GenericTimeoutExt<DictMarker> + Default>,
    ) -> Result<GenericServer, Report<DictError>> {
        let game = Self::game_id_lookup(game_id).ok_or_else(|| {
            Report::new(DictError::UnknownIdentifier {
                game_id: game_id.to_string(),
            })
        })?;

        let timeout = timeout.unwrap_or_default().into_marker();

        match game {
            #[cfg(feature = "game_ark_survival_ascended")]
            SupportedGame::ArkSurvivalAscended => {
                Ok(
                    ArkSurvivalAscendedClient::query_addr_with_timeout(addr, timeout)
                        .await
                        .change_context(DictError::Query)?
                        .into_generic_server(),
                )
            }
        }
    }
}
