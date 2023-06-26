use crate::{
    bufferer::{Bufferer, Endianess},
    protocols::{
        types::TimeoutSettings,
        valve::{
            types::{
                Environment,
                ExtraData,
                GatheringSettings,
                Request,
                Response,
                Server,
                ServerInfo,
                ServerPlayer,
                TheShip,
            },
            Engine,
            ModData,
            SteamApp,
        },
    },
    socket::{Socket, UdpSocket},
    utils::u8_lower_upper,
    GDError::{BadGame, Decompress, UnknownEnumCast},
    GDResult,
};

use bzip2_rs::decoder::Decoder;

use crate::protocols::valve::Packet;
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Debug)]
#[allow(dead_code)] //remove this later on
struct SplitPacket {
    pub header: u32,
    pub id: u32,
    pub total: u8,
    pub number: u8,
    pub size: u16,
    pub compressed: bool,
    pub decompressed_size: Option<u32>,
    pub uncompressed_crc32: Option<u32>,
    payload: Vec<u8>,
}

impl SplitPacket {
    fn new(engine: &Engine, protocol: u8, buffer: &mut Bufferer) -> GDResult<Self> {
        let header = buffer.get_u32()?;
        let id = buffer.get_u32()?;
        let (total, number, size, compressed, decompressed_size, uncompressed_crc32) = match engine {
            Engine::GoldSrc(_) => {
                let (lower, upper) = u8_lower_upper(buffer.get_u8()?);
                (lower, upper, 0, false, None, None)
            }
            Engine::Source(_) => {
                let total = buffer.get_u8()?;
                let number = buffer.get_u8()?;
                let size = match protocol == 7 && (*engine == SteamApp::CSS.as_engine()) {
                    // certain apps with protocol = 7 dont have this field
                    false => buffer.get_u16()?,
                    true => 1248,
                };
                let compressed = ((id >> 31) & 1) == 1;
                let (decompressed_size, uncompressed_crc32) = match compressed {
                    false => (None, None),
                    true => (Some(buffer.get_u32()?), Some(buffer.get_u32()?)),
                };
                (
                    total,
                    number,
                    size,
                    compressed,
                    decompressed_size,
                    uncompressed_crc32,
                )
            }
        };

        Ok(Self {
            header,
            id,
            total,
            number,
            size,
            compressed,
            decompressed_size,
            uncompressed_crc32,
            payload: buffer.remaining_data().to_vec(),
        })
    }

    fn get_payload(&self) -> GDResult<Vec<u8>> {
        if self.compressed {
            let mut decoder = Decoder::new();
            decoder.write(&self.payload).map_err(|_| Decompress)?;

            let decompressed_size = self.decompressed_size.unwrap() as usize;

            let mut decompressed_payload = vec![0; decompressed_size];

            decoder
                .read(&mut decompressed_payload)
                .map_err(|_| Decompress)?;

            if decompressed_payload.len() != decompressed_size
                || crc32fast::hash(&decompressed_payload) != self.uncompressed_crc32.unwrap()
            {
                Err(Decompress)
            } else {
                Ok(decompressed_payload)
            }
        } else {
            Ok(self.payload.clone())
        }
    }
}

pub(crate) struct ValveProtocol {
    socket: UdpSocket,
}

static PACKET_SIZE: usize = 6144;

impl ValveProtocol {
    pub fn new(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self { socket })
    }

    fn receive(&mut self, engine: &Engine, protocol: u8, buffer_size: usize) -> GDResult<Packet> {
        let data = self.socket.receive(Some(buffer_size))?;
        let mut buffer = Bufferer::new_with_data(Endianess::Little, &data);

        let header = buffer.get_u8()?;
        buffer.move_position_backward(1);
        if header == 0xFE {
            // the packet is split
            let mut main_packet = SplitPacket::new(engine, protocol, &mut buffer)?;
            let mut chunk_packets = Vec::with_capacity((main_packet.total - 1) as usize);

            for _ in 1 .. main_packet.total {
                let new_data = self.socket.receive(Some(buffer_size))?;
                buffer = Bufferer::new_with_data(Endianess::Little, &new_data);
                let chunk_packet = SplitPacket::new(engine, protocol, &mut buffer)?;
                chunk_packets.push(chunk_packet);
            }

            chunk_packets.sort_by(|a, b| a.number.cmp(&b.number));

            for chunk_packet in chunk_packets {
                main_packet.payload.extend(chunk_packet.payload);
            }

            let mut new_packet_buffer = Bufferer::new_with_data(Endianess::Little, &main_packet.get_payload()?);
            Ok(Packet::new_from_bufferer(&mut new_packet_buffer)?)
        } else {
            Packet::new_from_bufferer(&mut buffer)
        }
    }

    pub fn get_kind_request_data(&mut self, engine: &Engine, protocol: u8, kind: Request) -> GDResult<Bufferer> {
        self.get_request_data(engine, protocol, kind as u8, kind.get_default_payload())
    }

    /// Ask for a specific request only.
    pub fn get_request_data(
        &mut self,
        engine: &Engine,
        protocol: u8,
        kind: u8,
        payload: Vec<u8>,
    ) -> GDResult<Bufferer> {
        let request_initial_packet = Packet::new(kind, payload).to_bytes();
        self.socket.send(&request_initial_packet)?;

        let mut packet = self.receive(engine, protocol, PACKET_SIZE)?;
        while packet.kind == 0x41 {
            // 'A'
            let challenge = packet.payload;

            const INFO: u8 = Request::Info as u8; // hmm, this could be unwanted and problematic
            let challenge_packet = Packet::new(
                kind,
                match kind {
                    INFO => [Request::Info.get_default_payload(), challenge].concat(),
                    _ => challenge,
                },
            )
            .to_bytes();

            self.socket.send(&challenge_packet)?;

            packet = self.receive(engine, protocol, PACKET_SIZE)?;
        }

        let data = packet.payload;
        Ok(Bufferer::new_with_data(Endianess::Little, &data))
    }

    fn get_goldsrc_server_info(buffer: &mut Bufferer) -> GDResult<ServerInfo> {
        buffer.get_u8()?; //get the header (useless info)
        buffer.get_string_utf8()?; //get the server address (useless info)
        let name = buffer.get_string_utf8()?;
        let map = buffer.get_string_utf8()?;
        let folder = buffer.get_string_utf8()?;
        let game = buffer.get_string_utf8()?;
        let players = buffer.get_u8()?;
        let max_players = buffer.get_u8()?;
        let protocol = buffer.get_u8()?;
        let server_type = match buffer.get_u8()? {
            68 => Server::Dedicated,    //'D'
            76 => Server::NonDedicated, //'L'
            80 => Server::TV,           //'P'
            _ => Err(UnknownEnumCast)?,
        };
        let environment_type = match buffer.get_u8()? {
            76 => Environment::Linux,   //'L'
            87 => Environment::Windows, //'W'
            _ => Err(UnknownEnumCast)?,
        };
        let has_password = buffer.get_u8()? == 1;
        let is_mod = buffer.get_u8()? == 1;
        let mod_data = match is_mod {
            false => None,
            true => {
                Some(ModData {
                    link: buffer.get_string_utf8()?,
                    download_link: buffer.get_string_utf8()?,
                    version: buffer.get_u32()?,
                    size: buffer.get_u32()?,
                    multiplayer_only: buffer.get_u8()? == 1,
                    has_own_dll: buffer.get_u8()? == 1,
                })
            }
        };
        let vac_secured = buffer.get_u8()? == 1;
        let bots = buffer.get_u8()?;

        Ok(ServerInfo {
            protocol,
            name,
            map,
            folder,
            game,
            appid: 0, // not present in the obsolete response
            players_online: players,
            players_maximum: max_players,
            players_bots: bots,
            server_type,
            environment_type,
            has_password,
            vac_secured,
            the_ship: None,
            version: "".to_string(), // a version field only for the mod
            extra_data: None,
            is_mod,
            mod_data,
        })
    }

    /// Get the server information's.
    fn get_server_info(&mut self, engine: &Engine) -> GDResult<ServerInfo> {
        let mut buffer = self.get_kind_request_data(engine, 0, Request::Info)?;

        if let Engine::GoldSrc(force) = engine {
            if *force {
                return ValveProtocol::get_goldsrc_server_info(&mut buffer);
            }
        }

        let protocol = buffer.get_u8()?;
        let name = buffer.get_string_utf8()?;
        let map = buffer.get_string_utf8()?;
        let folder = buffer.get_string_utf8()?;
        let game = buffer.get_string_utf8()?;
        let mut appid = buffer.get_u16()? as u32;
        let players = buffer.get_u8()?;
        let max_players = buffer.get_u8()?;
        let bots = buffer.get_u8()?;
        let server_type = Server::from_gldsrc(buffer.get_u8()?)?;
        let environment_type = Environment::from_gldsrc(buffer.get_u8()?)?;
        let has_password = buffer.get_u8()? == 1;
        let vac_secured = buffer.get_u8()? == 1;
        let the_ship = match *engine == SteamApp::TS.as_engine() {
            false => None,
            true => {
                Some(TheShip {
                    mode: buffer.get_u8()?,
                    witnesses: buffer.get_u8()?,
                    duration: buffer.get_u8()?,
                })
            }
        };
        let version = buffer.get_string_utf8()?;
        let extra_data = match buffer.get_u8() {
            Err(_) => None,
            Ok(value) => {
                Some(ExtraData {
                    port: match (value & 0x80) > 0 {
                        false => None,
                        true => Some(buffer.get_u16()?),
                    },
                    steam_id: match (value & 0x10) > 0 {
                        false => None,
                        true => Some(buffer.get_u64()?),
                    },
                    tv_port: match (value & 0x40) > 0 {
                        false => None,
                        true => Some(buffer.get_u16()?),
                    },
                    tv_name: match (value & 0x40) > 0 {
                        false => None,
                        true => Some(buffer.get_string_utf8()?),
                    },
                    keywords: match (value & 0x20) > 0 {
                        false => None,
                        true => Some(buffer.get_string_utf8()?),
                    },
                    game_id: match (value & 0x01) > 0 {
                        false => None,
                        true => {
                            let gid = buffer.get_u64()?;
                            appid = (gid & ((1 << 24) - 1)) as u32;

                            Some(gid)
                        }
                    },
                })
            }
        };

        Ok(ServerInfo {
            protocol,
            name,
            map,
            folder,
            game,
            appid,
            players_online: players,
            players_maximum: max_players,
            players_bots: bots,
            server_type,
            environment_type,
            has_password,
            vac_secured,
            the_ship,
            version,
            extra_data,
            is_mod: false,
            mod_data: None,
        })
    }

    /// Get the server player's.
    fn get_server_players(&mut self, engine: &Engine, protocol: u8) -> GDResult<Vec<ServerPlayer>> {
        let mut buffer = self.get_kind_request_data(engine, protocol, Request::Players)?;

        let count = buffer.get_u8()? as usize;
        let mut players: Vec<ServerPlayer> = Vec::with_capacity(count);

        for _ in 0 .. count {
            buffer.move_position_ahead(1); //skip the index byte

            players.push(ServerPlayer {
                name: buffer.get_string_utf8()?,
                score: buffer.get_u32()?,
                duration: buffer.get_f32()?,
                deaths: match *engine == SteamApp::TS.as_engine() {
                    false => None,
                    true => Some(buffer.get_u32()?),
                },
                money: match *engine == SteamApp::TS.as_engine() {
                    false => None,
                    true => Some(buffer.get_u32()?),
                },
            });
        }

        Ok(players)
    }

    /// Get the server's rules.
    fn get_server_rules(&mut self, engine: &Engine, protocol: u8) -> GDResult<HashMap<String, String>> {
        let mut buffer = self.get_kind_request_data(engine, protocol, Request::Rules)?;

        let count = buffer.get_u16()? as usize;
        let mut rules: HashMap<String, String> = HashMap::with_capacity(count);

        for _ in 0 .. count {
            let name = buffer.get_string_utf8()?;
            let value = buffer.get_string_utf8()?;

            rules.insert(name, value);
        }

        if *engine == SteamApp::ROR2.as_engine() {
            rules.remove("Test");
        }

        Ok(rules)
    }
}

/// Query a server by providing the address, the port, the app, gather and
/// timeout settings. Providing None to the settings results in using the
/// default values for them
/// (GatherSettings::[default](GatheringSettings::default),
/// TimeoutSettings::[default](TimeoutSettings::default)).
pub fn query(
    address: &SocketAddr,
    engine: Engine,
    gather_settings: Option<GatheringSettings>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response> {
    let response_gather_settings = gather_settings.unwrap_or_default();
    get_response(address, engine, response_gather_settings, timeout_settings)
}

fn get_response(
    address: &SocketAddr,
    engine: Engine,
    gather_settings: GatheringSettings,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response> {
    let mut client = ValveProtocol::new(address, timeout_settings)?;

    let info = client.get_server_info(&engine)?;
    let protocol = info.protocol;

    if let Engine::Source(Some(appids)) = &engine {
        let mut is_specified_id = false;

        if appids.0 == info.appid {
            is_specified_id = true;
        } else if let Some(dedicated_appid) = appids.1 {
            if dedicated_appid == info.appid {
                is_specified_id = true;
            }
        }

        if !is_specified_id {
            return Err(BadGame(format!("AppId: {}", info.appid)));
        }
    }

    Ok(Response {
        info,
        players: match gather_settings.players {
            false => None,
            true => Some(client.get_server_players(&engine, protocol)?),
        },
        rules: match gather_settings.rules {
            false => None,
            true => Some(client.get_server_rules(&engine, protocol)?),
        },
    })
}
