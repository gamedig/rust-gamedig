use crate::{
    converters::{DictMarker, GenericQueryExt, GenericServer, GenericServerExt, GenericTimeoutExt},
    core::{
        ToSocketAddr,
        error::{Report, ResultExt},
    },
    game::ark_survival_ascended::ArkSurvivalAscendedClient,
};

#[derive(Debug, thiserror::Error)]
pub enum DictError {
    #[error("[GameDig]::[DICT::QUERY]: Failed to query game server")]
    Query,
    #[error("[GameDig]::[DICT::UNKNOWN_IDENTIFIER]: Unknown game identifier provided")]
    UnknownGameIdentifier { game_id: String },
    #[error("[GameDig]::[DICT::UNKNOWN_STEAM_ID]: Unknown steam id provided")]
    UnknownSteamId { steam_id: u32 },
}

#[derive(Debug, Clone, Copy)]
enum SupportedGame {
    #[cfg(feature = "game_ark_survival_ascended")]
    ArkSurvivalAscended,
}

pub struct Dict;

#[maybe_async::maybe_async]
impl Dict {
    fn game_id_lookup(game_id: &str) -> Option<SupportedGame> {
        // Keep id below 50 bytes to ensure fast lookup
        // tiny_map is x4 faster than phf_map here
        hashify::tiny_map! {
            game_id.as_bytes(),
            #[cfg(feature = "game_ark_survival_ascended")]
            "ark_survival_ascended" => SupportedGame::ArkSurvivalAscended,
        }
        // Node GameDig compatibility mapping
        .or(hashify::tiny_map! {
            game_id.as_bytes(),
            #[cfg(feature = "game_ark_survival_ascended")]
            "asa" => SupportedGame::ArkSurvivalAscended
        })
    }

    fn game_steam_id_lookup(steam_id: u32) -> Option<SupportedGame> {
        // <steam_id>
        // or
        // <steam_id> | <dedicated_steam_id>
        static STEAM_IDS: phf::Map<u32, SupportedGame> = phf::phf_map! {
            #[cfg(feature = "game_ark_survival_ascended")]
            2399830 | 2430930 => SupportedGame::ArkSurvivalAscended,
        };

        STEAM_IDS.get(&steam_id).copied()
    }

    async fn query_game<A: ToSocketAddr>(
        game: SupportedGame,
        addr: A,
        timeout: Option<impl GenericTimeoutExt<DictMarker> + Default>,
    ) -> Result<GenericServer, Report<DictError>> {
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

    pub async fn query_by_game_id<A: ToSocketAddr>(
        game_id: &str,
        addr: A,
        timeout: Option<impl GenericTimeoutExt<DictMarker> + Default>,
    ) -> Result<GenericServer, Report<DictError>> {
        Self::query_game(
            Self::game_id_lookup(game_id).ok_or_else(|| {
                Report::new(DictError::UnknownGameIdentifier {
                    game_id: game_id.to_string(),
                })
            })?,
            addr,
            timeout,
        )
        .await
    }

    pub async fn query_by_steam_id<A: ToSocketAddr>(
        steam_id: u32,
        addr: A,
        timeout: Option<impl GenericTimeoutExt<DictMarker> + Default>,
    ) -> Result<GenericServer, Report<DictError>> {
        Self::query_game(
            Self::game_steam_id_lookup(steam_id)
                .ok_or_else(|| Report::new(DictError::UnknownSteamId { steam_id }))?,
            addr,
            timeout,
        )
        .await
    }
}
