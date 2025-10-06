use std::{collections::HashMap, net::SocketAddr, time::Duration};

use crate::{
    core::{Buffer, UdpClient},
    error::Result,
    protocol::query::valve::model::{
        ExtraData,
        ExtraDataFlag,
        ExtraDataFlags,
        Player,
        Response,
        ServerEnvironment,
        ServerType,
        SourceTV,
        TheShip,
        TheShipMode,
    },
};

/// A client for querying Valve game servers using the Valve Query Protocol.
pub struct ValveQueryClient {
    /// The underlying network client
    net: UdpClient,
}

#[maybe_async::maybe_async]
impl ValveQueryClient {
    pub async fn new(
        address: SocketAddr,
        read_timeout: Option<Duration>,
        write_timeout: Option<Duration>,
    ) -> Result<Self> {
        Ok(Self {
            net: UdpClient::new(address, read_timeout, write_timeout).await?,
        })
    }

    async fn net_send(&mut self, payload: &[u8]) -> Result<Buffer<Vec<u8>>> {
        self.net.send(payload).await?;

        let mut heap = Vec::with_capacity(1400);
        self.net.recv(&mut heap).await?;

        let mut datagram = Buffer::new(heap);

        match datagram.read_i32_le()? {
            // Singular
            -1 => Ok(datagram),

            // Fragmented
            -2 => todo!(),

            // Invalid response
            _ => todo!(),
        }
    }

    pub async fn get_info(&mut self) -> Result<Response> {
        const INFO_REQUEST_PAYLOAD: &[u8; 25] = b"\xFF\xFF\xFF\xFFTSource Engine Query\0";

        let mut datagram_payload = self.net_send(INFO_REQUEST_PAYLOAD).await?;

        loop {
            // Match payload header
            match datagram_payload.read_u8()? {
                // Challenge
                0x41 => {
                    let mut challenge_payload = [0u8; 29];
                    challenge_payload[.. 25].copy_from_slice(INFO_REQUEST_PAYLOAD);
                    // Remaining slice is the challenge
                    // This way avoids range check and the unnecessary conversion to i32 and back
                    challenge_payload[25 ..].copy_from_slice(datagram_payload.remaining_slice());

                    datagram_payload = self.net_send(&challenge_payload).await?;

                    continue;
                }

                // Source
                0x49 => {
                    let protocol_version = datagram_payload.read_u8()?;
                    let server_name = datagram_payload.read_string_utf8(Some(0), true)?;
                    let map_name = datagram_payload.read_string_utf8(Some(0), true)?;
                    let game_folder_name = datagram_payload.read_string_utf8(Some(0), true)?;
                    let game_name = datagram_payload.read_string_utf8(Some(0), true)?;
                    let game_app_id = datagram_payload.read_u16_le()?;
                    let num_players = datagram_payload.read_u8()?;
                    let max_players = datagram_payload.read_u8()?;
                    let num_bots = datagram_payload.read_u8()?;
                    let server_type = ServerType::from_u8(datagram_payload.read_u8()?);
                    let server_environment =
                        ServerEnvironment::from_u8(datagram_payload.read_u8()?);
                    let password = datagram_payload.read_u8()? != 0;
                    let vac_enabled = datagram_payload.read_u8()? != 0;
                    let the_ship = if game_app_id == 2400 {
                        Some(TheShip {
                            mode: TheShipMode::from_u8(datagram_payload.read_u8()?),
                            witnesses: datagram_payload.read_u8()?,
                            duration: datagram_payload.read_u8()?,
                        })
                    } else {
                        None
                    };
                    let game_version = datagram_payload.read_string_utf8(Some(0), true)?;
                    let extra_data_flag = if !datagram_payload.is_empty() {
                        Some(ExtraDataFlag(datagram_payload.read_u8()?))
                    } else {
                        None
                    };
                    let extra_data = if let Some(edf) = extra_data_flag {
                        let mut data = ExtraData {
                            game_app_id_64: None,
                            server_steam_id: None,
                            keywords: None,
                            source_tv: None,
                            port: None,
                        };

                        if edf.contains(ExtraDataFlags::Port) {
                            data.port = Some(datagram_payload.read_u16_le()?);
                        }

                        if edf.contains(ExtraDataFlags::SteamID) {
                            data.server_steam_id = Some(datagram_payload.read_u64_le()?);
                        }

                        if edf.contains(ExtraDataFlags::SourceTV) {
                            let port = datagram_payload.read_u16_le()?;
                            let name = datagram_payload.read_string_utf8(Some(0), true)?;

                            data.source_tv = Some(SourceTV { port, name });
                        }

                        if edf.contains(ExtraDataFlags::Keywords) {
                            data.keywords = Some(datagram_payload.read_string_utf8(Some(0), true)?);
                        }

                        if edf.contains(ExtraDataFlags::GameID) {
                            data.game_app_id_64 = Some(datagram_payload.read_u64_le()?);
                        }

                        Some(data)
                    } else {
                        None
                    };

                    return Ok(Response {
                        protocol_version,
                        server_name,
                        map_name,
                        game_folder_name,
                        game_name,
                        game_app_id: Some(game_app_id),
                        num_players,
                        max_players,
                        num_bots,
                        server_type,
                        server_environment,
                        password,
                        vac_enabled,
                        the_ship,
                        game_version: Some(game_version),
                        extra_data_flag,
                        extra_data,

                        // not provided by this request
                        rules: None,
                        players: None,
                        gold_src_mod: None,
                    });
                }

                // GoldSrc
                0x6D => todo!(),

                // Invalid
                _ => todo!(),
            }
        }
    }

    pub async fn get_players(&self) -> Result<Vec<Player>> {
        todo!();
    }

    pub async fn get_rules(&self) -> Result<HashMap<String, String>> {
        todo!();
    }

    pub async fn get_all(&self) -> Result<Response> {
        todo!();
    }
}
