use std::collections::HashMap;
use bzip2_rs::decoder::Decoder;
use crate::GDResult;
use crate::bufferer::{Bufferer, Endianess};
use crate::GDError::{BadGame, Decompress, UnknownEnumCast};
use crate::protocols::types::TimeoutSettings;
use crate::protocols::valve::{App, ModData, SteamID};
use crate::protocols::valve::types::{Environment, ExtraData, GatheringSettings, Request, Response, Server, ServerInfo, ServerPlayer, TheShip};
use crate::socket::{Socket, UdpSocket};
use crate::utils::u8_lower_upper;

#[derive(Debug, Clone)]
struct Packet {
    pub header: u32,
    pub kind: u8,
    pub payload: Vec<u8>
}

impl Packet {
    fn new(buffer: &mut Bufferer) -> GDResult<Self> {
        Ok(Self {
            header: buffer.get_u32()?,
            kind: buffer.get_u8()?,
            payload: buffer.get_data_in_front_of_position()
        })
    }

    fn challenge(kind: Request, challenge: Vec<u8>) -> Self {
        let mut initial = Packet::initial(kind);

        Self {
            header: initial.header,
            kind: initial.kind,
            payload: match kind {
                Request::INFO => {
                    initial.payload.extend(challenge);
                    initial.payload
                },
                _ => challenge
            }
        }
    }

    fn initial(kind: Request) -> Self {
        Self {
            header: 4294967295, //FF FF FF FF
            kind: kind as u8,
            payload: match kind {
                Request::INFO => String::from("Source Engine Query\0").into_bytes(),
                _ => vec![0xFF, 0xFF, 0xFF, 0xFF]
            }
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::from(self.header.to_be_bytes());

        buf.push(self.kind);
        buf.extend(&self.payload);

        buf
    }
}

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
    payload: Vec<u8>
}

impl SplitPacket {
    fn new(app: &App, protocol: u8, buffer: &mut Bufferer) -> GDResult<Self> {
        let header = buffer.get_u32()?;
        let id = buffer.get_u32()?;
        let (total, number, size, compressed, decompressed_size, uncompressed_crc32) = match app {
            App::GoldSrc(_) => {
                let (lower, upper) = u8_lower_upper(buffer.get_u8()?);
                (lower, upper, 0, false, None, None)
            }
            App::Source(_) => {
                let total = buffer.get_u8()?;
                let number = buffer.get_u8()?;
                let size = match protocol == 7 && (*app == SteamID::CSS.as_app()) { //certain apps with protocol = 7 doesnt have this field
                    false => buffer.get_u16()?,
                    true => 1248
                };
                let compressed = ((id >> 31) & 1) == 1;
                let (decompressed_size, uncompressed_crc32) = match compressed {
                    false => (None, None),
                    true => (Some(buffer.get_u32()?), Some(buffer.get_u32()?))
                };
                (total, number, size, compressed, decompressed_size, uncompressed_crc32)
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
            payload: buffer.get_data_in_front_of_position()
        })
    }

    fn get_payload(&self) -> GDResult<Vec<u8>> {
        if self.compressed {
            let mut decoder = Decoder::new();
            decoder.write(&self.payload).map_err(|_| Decompress)?;

            let decompressed_size = self.decompressed_size.unwrap() as usize;

            let mut decompressed_payload = Vec::with_capacity(decompressed_size);
            decoder.read(&mut decompressed_payload).map_err(|_| Decompress)?;

            if decompressed_payload.len() != decompressed_size {
                Err(Decompress)
            }
            else if crc32fast::hash(&decompressed_payload) != self.uncompressed_crc32.unwrap() {
                Err(Decompress)
            }
            else {
                Ok(decompressed_payload)
            }
        } else {
            Ok(self.payload.clone())
        }
    }
}

struct ValveProtocol {
    socket: UdpSocket
}

static PACKET_SIZE: usize = 6144;

impl ValveProtocol {
    fn new(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address, port)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self {
            socket
        })
    }

    fn receive(&mut self, app: &App, protocol: u8, buffer_size: usize) -> GDResult<Packet> {
        let data = self.socket.receive(Some(buffer_size))?;
        let mut buffer = Bufferer::new_with_data(Endianess::Little, &data); 

        let header = buffer.get_u8()?;
        buffer.move_position_backward(1);
        if header == 0xFE { //the packet is split
            let mut main_packet = SplitPacket::new(&app, protocol, &mut buffer)?;

            for _ in 1..main_packet.total {
                let new_data = self.socket.receive(Some(buffer_size))?;
                buffer = Bufferer::new_with_data(Endianess::Little, &new_data);
                let chunk_packet = SplitPacket::new(&app, protocol, &mut buffer)?;
                main_packet.payload.extend(chunk_packet.payload);
            }

            let mut new_packet_buffer = Bufferer::new_with_data(Endianess::Little, &main_packet.get_payload()?);
            Ok(Packet::new(&mut new_packet_buffer)?)
        }
        else {
            Packet::new(&mut buffer)
        }
    }

    /// Ask for a specific request only.
    fn get_request_data(&mut self, app: &App, protocol: u8, kind: Request) -> GDResult<Bufferer> {
        let request_initial_packet = Packet::initial(kind).to_bytes();

        self.socket.send(&request_initial_packet)?;
        let packet = self.receive(app, protocol, PACKET_SIZE)?;

        if packet.kind != 0x41 { //'A'
            let data = packet.payload.clone();
            return Ok(Bufferer::new_with_data(Endianess::Little, &data));
        }

        let challenge = packet.payload;
        let challenge_packet = Packet::challenge(kind, challenge).to_bytes();

        self.socket.send(&challenge_packet)?;

        let data = self.receive(app, protocol, PACKET_SIZE)?.payload;
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
            68 => Server::Dedicated, //'D'
            76 => Server::NonDedicated, //'L'
            80 => Server::TV, //'P'
            _ => Err(UnknownEnumCast)?
        };
        let environment_type = match buffer.get_u8()? {
            76 => Environment::Linux, //'L'
            87 => Environment::Windows, //'W'
            _ => Err(UnknownEnumCast)?
        };
        let has_password = buffer.get_u8()? == 1;
        let is_mod = buffer.get_u8()? == 1;
        let mod_data = match is_mod {
            false => None,
            true => Some(ModData {
                link: buffer.get_string_utf8()?,
                download_link: buffer.get_string_utf8()?,
                version: buffer.get_u32()?,
                size: buffer.get_u32()?,
                multiplayer_only: buffer.get_u8()? == 1,
                has_own_dll: buffer.get_u8()? == 1
            })
        };
        let vac_secured = buffer.get_u8()? == 1;
        let bots = buffer.get_u8()?;

        Ok(ServerInfo {
            protocol,
            name,
            map,
            folder,
            game,
            appid: 0, //not present in the obsolete response
            players_online: players,
            players_maximum: max_players,
            players_bots: bots,
            server_type,
            environment_type,
            has_password,
            vac_secured,
            the_ship: None,
            version: "".to_string(), //a version field only for the mod
            extra_data: None,
            is_mod,
            mod_data
        })
    }

    /// Get the server information's.
    fn get_server_info(&mut self, app: &App) -> GDResult<ServerInfo> {
        let mut buffer = self.get_request_data(&app, 0, Request::INFO)?;
        
        if let App::GoldSrc(force) = app {
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
        let server_type = match buffer.get_u8()? {
            100 => Server::Dedicated, //'d'
            108 => Server::NonDedicated, //'l'
            112 => Server::TV, //'p'
            _ => Err(UnknownEnumCast)?
        };
        let environment_type = match buffer.get_u8()? {
            108 => Environment::Linux, //'l'
            119 => Environment::Windows, //'w'
            109 | 111 => Environment::Mac, //'m' or 'o'
            _ => Err(UnknownEnumCast)?
        };
        let has_password = buffer.get_u8()? == 1;
        let vac_secured = buffer.get_u8()? == 1;
        let the_ship = match *app == SteamID::TS.as_app() {
            false => None,
            true => Some(TheShip {
                mode: buffer.get_u8()?,
                witnesses: buffer.get_u8()?,
                duration: buffer.get_u8()?
            })
        };
        let version = buffer.get_string_utf8()?;
        let extra_data = match buffer.get_u8() {
            Err(_) => None,
            Ok(value) => Some(ExtraData {
                port: match (value & 0x80) > 0 {
                    false => None,
                    true => Some(buffer.get_u16()?)
                },
                steam_id: match (value & 0x10) > 0 {
                    false => None,
                    true => Some(buffer.get_u64()?)
                },
                tv_port: match (value & 0x40) > 0 {
                    false => None,
                    true => Some(buffer.get_u16()?)
                },
                tv_name: match (value & 0x40) > 0 {
                    false => None,
                    true => Some(buffer.get_string_utf8()?)
                },
                keywords: match (value & 0x20) > 0 {
                    false => None,
                    true => Some(buffer.get_string_utf8()?)
                },
                game_id: match (value & 0x01) > 0 {
                    false => None,
                    true => {
                        let gid = buffer.get_u64()?;
                        appid = (gid & ((1 << 24) - 1)) as u32;

                        Some(gid)
                    }
                }
            })
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
            mod_data: None
        })
    }

    /// Get the server player's.
    fn get_server_players(&mut self, app: &App, protocol: u8) -> GDResult<Vec<ServerPlayer>> {
        let mut buffer = self.get_request_data(&app, protocol, Request::PLAYERS)?;

        let count = buffer.get_u8()? as usize;
        let mut players: Vec<ServerPlayer> = Vec::with_capacity(count);

        for _ in 0..count {
            buffer.move_position_ahead(1); //skip the index byte

            let name = buffer.get_string_utf8()?;
            let score = buffer.get_u32()?;
            let duration = buffer.get_f32()?;

            let deaths = match *app == SteamID::TS.as_app() {
                false => None,
                true => Some(buffer.get_u32()?)
            };
            let money = match *app == SteamID::TS.as_app() {
                false => None,
                true => Some(buffer.get_u32()?)
            };

            if name.len() > 0 {
                players.push(ServerPlayer {
                    name,
                    score,
                    duration,
                    deaths,
                    money
                });
            }
        }

        Ok(players)
    }

    /// Get the server's rules.
    fn get_server_rules(&mut self, app: &App, protocol: u8) -> GDResult<HashMap<String, String>> {
        let mut buffer = self.get_request_data(&app, protocol, Request::RULES)?;

        let count = buffer.get_u16()? as usize;
        let mut rules: HashMap<String, String> = HashMap::with_capacity(count);

        for _ in 0..count {
            let name = buffer.get_string_utf8()?;
            let value = buffer.get_string_utf8()?;

            rules.insert(name, value);
        }

        if *app == SteamID::ROR2.as_app() {
            rules.remove("Test");
        }

        Ok(rules)
    }
}

/// Query a server by providing the address, the port, the app, gather and timeout settings.
/// Providing None to the settings results in using the default values for them (GatherSettings::[default](GatheringSettings::default), TimeoutSettings::[default](TimeoutSettings::default)).
pub fn query(address: &str, port: u16, app: App, gather_settings: Option<GatheringSettings>, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let response_gather_settings = gather_settings.unwrap_or(GatheringSettings::default());
    get_response(address, port, app, response_gather_settings, timeout_settings)
}

fn get_response(address: &str, port: u16, app: App, gather_settings: GatheringSettings, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let mut client = ValveProtocol::new(address, port, timeout_settings)?;

    let info = client.get_server_info(&app)?;
    let protocol = info.protocol;

    if let App::Source(source_app) = &app {
        if let Some(appid) = source_app {
            if *appid != info.appid {
                return Err(BadGame(format!("AppId: {}", appid)));
            }
        }
    }

    Ok(Response {
        info,
        players: match gather_settings.players {
            false => None,
            true => Some(client.get_server_players(&app, protocol)?)
        },
        rules: match gather_settings.rules {
            false => None,
            true => Some(client.get_server_rules(&app, protocol)?)
        }
    })
}
