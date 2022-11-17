use std::net::UdpSocket;
use bzip2_rs::decoder::Decoder;
use crate::{GDError, GDResult};
use crate::protocols::types::TimeoutSettings;
use crate::protocols::valve::{App, ModData, SteamID};
use crate::protocols::valve::types::{Environment, ExtraData, GatheringSettings, Request, Response, Server, ServerInfo, ServerPlayer, ServerRule, TheShip};
use crate::utils::{buffer, complete_address, u8_lower_upper};

#[derive(Debug, Clone)]
struct Packet {
    pub header: u32,
    pub kind: u8,
    pub payload: Vec<u8>
}

impl Packet {
    fn new(buf: &[u8]) -> GDResult<Self> {
        let mut pos = 0;
        Ok(Self {
            header: buffer::get_u32_le(&buf, &mut pos)?,
            kind: buffer::get_u8(&buf, &mut pos)?,
            payload: buf[pos..].to_vec()
        })
    }

    fn challenge(kind: Request, challenge: Vec<u8>) -> Self {
        let mut initial = Packet::initial(kind);

        Self {
            header: initial.header,
            kind: initial.kind,
            payload: match initial.kind {
                0x54 => {
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
            kind: kind.clone() as u8,
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
    fn new(app: &App, protocol: u8, buf: &[u8]) -> GDResult<Self> {
        let mut pos = 0;

        let header = buffer::get_u32_le(&buf, &mut pos)?;
        let id = buffer::get_u32_le(&buf, &mut pos)?;
        let (total, number, size, compressed, decompressed_size, uncompressed_crc32) = match app {
            App::GoldSrc(_) => {
                let (lower, upper) = u8_lower_upper(buffer::get_u8(&buf, &mut pos)?);
                (lower, upper, 0, false, None, None)
            }
            App::Source(_) => {
                let total = buffer::get_u8(&buf, &mut pos)?;
                let number = buffer::get_u8(&buf, &mut pos)?;
                let size = match protocol == 7 && (*app == SteamID::CSS.as_app()) { //certain apps with protocol = 7 doesnt have this field
                    false => buffer::get_u16_le(&buf, &mut pos)?,
                    true => 1248
                };
                let compressed = ((id >> 31) & 1) == 1;
                let (decompressed_size, uncompressed_crc32) = match compressed {
                    false => (None, None),
                    true => (Some(buffer::get_u32_le(&buf, &mut pos)?), Some(buffer::get_u32_le(&buf, &mut pos)?))
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
            payload: buf[pos..].to_vec()
        })
    }

    fn get_payload(&self) -> GDResult<Vec<u8>> {
        if self.compressed {
            let mut decoder = Decoder::new();
            decoder.write(&self.payload).map_err(|e| GDError::Decompress(e.to_string()))?;

            let decompressed_size = self.decompressed_size.unwrap() as usize;

            let mut decompressed_payload = Vec::with_capacity(decompressed_size);
            decoder.read(&mut decompressed_payload).map_err(|e| GDError::Decompress(e.to_string()))?;

            if decompressed_payload.len() != decompressed_size {
                Err(GDError::Decompress("Valve Protocol: The decompressed payload size doesn't match the expected one.".to_string()))
            }
            else if crc32fast::hash(&decompressed_payload) != self.uncompressed_crc32.unwrap() {
                Err(GDError::Decompress("Valve Protocol: The decompressed crc32 hash does not match the expected one.".to_string()))
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
    socket: UdpSocket,
    complete_address: String
}

static PACKET_SIZE: usize = 1400;

impl ValveProtocol {
    fn new(address: &str, port: u16, timeout_settings: TimeoutSettings) -> GDResult<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| GDError::SocketBind(e.to_string()))?;

        socket.set_read_timeout(timeout_settings.get_read()).unwrap();  //unwrapping because TimeoutSettings::new
        socket.set_write_timeout(timeout_settings.get_write()).unwrap();//checks if these are 0 and throws an error

        Ok(Self {
            socket,
            complete_address: complete_address(address, port)?
        })
    }

    fn send(&self, data: &[u8]) -> GDResult<()> {
        self.socket.send_to(&data, &self.complete_address).map_err(|e| GDError::PacketSend(e.to_string()))?;
        Ok(())
    }

    fn receive_raw(&self, buffer_size: usize) -> GDResult<Vec<u8>> {
        let mut buf: Vec<u8> = vec![0; buffer_size];
        let (amt, _) = self.socket.recv_from(&mut buf.as_mut_slice()).map_err(|e| GDError::PacketReceive(e.to_string()))?;

        if amt < 6 {
            return Err(GDError::PacketUnderflow("Any Valve Protocol response can't be under 6 bytes long.".to_string()));
        }

        Ok(buf[..amt].to_vec())
    }

    fn receive(&self, app: &App, protocol: u8, buffer_size: usize) -> GDResult<Packet> {
        let mut buf = self.receive_raw(buffer_size)?;

        if buf[0] == 0xFE { //the packet is split
            let mut main_packet = SplitPacket::new(&app, protocol, &buf)?;

            for _ in 1..main_packet.total {
                buf = self.receive_raw(buffer_size)?;
                let chunk_packet = SplitPacket::new(&app, protocol, &buf)?;
                main_packet.payload.extend(chunk_packet.payload);
            }

            Ok(Packet::new(&main_packet.get_payload()?)?)
        }
        else {
            Packet::new(&buf)
        }
    }

    /// Ask for a specific request only.
    fn get_request_data(&self, app: &App, protocol: u8, kind: Request) -> GDResult<Vec<u8>> {
        let request_initial_packet = Packet::initial(kind.clone()).to_bytes();

        self.send(&request_initial_packet)?;
        let packet = self.receive(app, protocol, PACKET_SIZE)?;

        if packet.kind != 0x41 { //'A'
            return Ok(packet.payload.clone());
        }

        let challenge = packet.payload;
        let challenge_packet = Packet::challenge(kind.clone(), challenge).to_bytes();

        self.send(&challenge_packet)?;
        Ok(self.receive(app, protocol, PACKET_SIZE)?.payload)
    }

    fn get_goldsrc_server_info(buf: &[u8]) -> GDResult<ServerInfo> {
        let mut pos = 0;

        buffer::get_u8(&buf, &mut pos)?; //get the header (useless info)
        buffer::get_string(&buf, &mut pos)?; //get the server address (useless info)
        let name = buffer::get_string(&buf, &mut pos)?;
        let map = buffer::get_string(&buf, &mut pos)?;
        let folder = buffer::get_string(&buf, &mut pos)?;
        let game = buffer::get_string(&buf, &mut pos)?;
        let players = buffer::get_u8(&buf, &mut pos)?;
        let max_players = buffer::get_u8(&buf, &mut pos)?;
        let protocol = buffer::get_u8(&buf, &mut pos)?;
        let server_type = match buffer::get_u8(&buf, &mut pos)? {
            68 => Server::Dedicated, //'D'
            76 => Server::NonDedicated, //'L'
            80 => Server::TV, //'P'
            _ => Err(GDError::UnknownEnumCast)?
        };
        let environment_type = match buffer::get_u8(&buf, &mut pos)? {
            76 => Environment::Linux, //'L'
            87 => Environment::Windows, //'W'
            _ => Err(GDError::UnknownEnumCast)?
        };
        let has_password = buffer::get_u8(&buf, &mut pos)? == 1;
        let is_mod = buffer::get_u8(&buf, &mut pos)? == 1;
        let mod_data = match is_mod {
            false => None,
            true => Some(ModData {
                link: buffer::get_string(&buf, &mut pos)?,
                download_link: buffer::get_string(&buf, &mut pos)?,
                version: buffer::get_u32_le(&buf, &mut pos)?,
                size: buffer::get_u32_le(&buf, &mut pos)?,
                multiplayer_only: buffer::get_u8(&buf, &mut pos)? == 1,
                has_own_dll: buffer::get_u8(&buf, &mut pos)? == 1
            })
        };
        let vac_secured = buffer::get_u8(&buf, &mut pos)? == 1;
        let bots = buffer::get_u8(&buf, &mut pos)?;

        Ok(ServerInfo {
            protocol,
            name,
            map,
            folder,
            game,
            appid: 0, //not present in the obsolete response
            players,
            max_players,
            bots,
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
    fn get_server_info(&self, app: &App) -> GDResult<ServerInfo> {
        let buf = self.get_request_data(&app, 0, Request::INFO)?;
        if let App::GoldSrc(force) = app {
            if *force {
                return ValveProtocol::get_goldsrc_server_info(&buf);
            }
        }

        let mut pos = 0;

        let protocol = buffer::get_u8(&buf, &mut pos)?;
        let name = buffer::get_string(&buf, &mut pos)?;
        let map = buffer::get_string(&buf, &mut pos)?;
        let folder = buffer::get_string(&buf, &mut pos)?;
        let game = buffer::get_string(&buf, &mut pos)?;
        let mut appid = buffer::get_u16_le(&buf, &mut pos)? as u32;
        let players = buffer::get_u8(&buf, &mut pos)?;
        let max_players = buffer::get_u8(&buf, &mut pos)?;
        let bots = buffer::get_u8(&buf, &mut pos)?;
        let server_type = match buffer::get_u8(&buf, &mut pos)? {
            100 => Server::Dedicated, //'d'
            108 => Server::NonDedicated, //'l'
            112 => Server::TV, //'p'
            _ => Err(GDError::UnknownEnumCast)?
        };
        let environment_type = match buffer::get_u8(&buf, &mut pos)? {
            108 => Environment::Linux, //'l'
            119 => Environment::Windows, //'w'
            109 | 111 => Environment::Mac, //'m' or 'o'
            _ => Err(GDError::UnknownEnumCast)?
        };
        let has_password = buffer::get_u8(&buf, &mut pos)? == 1;
        let vac_secured = buffer::get_u8(&buf, &mut pos)? == 1;
        let the_ship = match *app == SteamID::TS.as_app() {
            false => None,
            true => Some(TheShip {
                mode: buffer::get_u8(&buf, &mut pos)?,
                witnesses: buffer::get_u8(&buf, &mut pos)?,
                duration: buffer::get_u8(&buf, &mut pos)?
            })
        };
        let version = buffer::get_string(&buf, &mut pos)?;
        let extra_data = match buffer::get_u8(&buf, &mut pos) {
            Err(_) => None,
            Ok(value) => Some(ExtraData {
                port: match (value & 0x80) > 0 {
                    false => None,
                    true => Some(buffer::get_u16_le(&buf, &mut pos)?)
                },
                steam_id: match (value & 0x10) > 0 {
                    false => None,
                    true => Some(buffer::get_u64_le(&buf, &mut pos)?)
                },
                tv_port: match (value & 0x40) > 0 {
                    false => None,
                    true => Some(buffer::get_u16_le(&buf, &mut pos)?)
                },
                tv_name: match (value & 0x40) > 0 {
                    false => None,
                    true => Some(buffer::get_string(&buf, &mut pos)?)
                },
                keywords: match (value & 0x20) > 0 {
                    false => None,
                    true => Some(buffer::get_string(&buf, &mut pos)?)
                },
                game_id: match (value & 0x01) > 0 {
                    false => None,
                    true => {
                        let gid = buffer::get_u64_le(&buf, &mut pos)?;
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
            players,
            max_players,
            bots,
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
    fn get_server_players(&self, app: &App, protocol: u8) -> GDResult<Vec<ServerPlayer>> {
        let buf = self.get_request_data(&app, protocol, Request::PLAYERS)?;
        let mut pos = 0;

        let count = buffer::get_u8(&buf, &mut pos)?;
        let mut players: Vec<ServerPlayer> = Vec::new();

        for _ in 0..count {
            pos += 1; //skip the index byte
            players.push(ServerPlayer {
                name: buffer::get_string(&buf, &mut pos)?,
                score: buffer::get_u32_le(&buf, &mut pos)?,
                duration: buffer::get_f32_le(&buf, &mut pos)?,
                deaths: match *app == SteamID::TS.as_app() {
                    false => None,
                    true => Some(buffer::get_u32_le(&buf, &mut pos)?)
                },
                money: match *app == SteamID::TS.as_app() {
                    false => None,
                    true => Some(buffer::get_u32_le(&buf, &mut pos)?)
                }
            });
        }

        Ok(players)
    }

    /// Get the server rules's.
    fn get_server_rules(&self, app: &App, protocol: u8) -> GDResult<Option<Vec<ServerRule>>> {
        if *app == SteamID::CSGO.as_app() { //cause csgo wont respond to this since feb 21 2014 update
            return Ok(None);
        }

        let buf = self.get_request_data(&app, protocol, Request::RULES)?;
        let mut pos = 0;

        let count = buffer::get_u16_le(&buf, &mut pos)?;
        let mut rules: Vec<ServerRule> = Vec::new();

        for _ in 0..count {
            rules.push(ServerRule {
                name: buffer::get_string(&buf, &mut pos)?,
                value: buffer::get_string(&buf, &mut pos)?
            })
        }

        Ok(Some(rules))
    }
}

/// Query a server by providing the address, the port, the app, gather and timeout settings.
/// Providing None to the settings results in using the default values for them (GatherSettings::[default](GatheringSettings::default), TimeoutSettings::[default](TimeoutSettings::default)).
pub fn query(address: &str, port: u16, app: App, gather_settings: Option<GatheringSettings>, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let response_gather_settings = gather_settings.unwrap_or(GatheringSettings::default());
    let response_timeout_settings = timeout_settings.unwrap_or(TimeoutSettings::default());
    get_response(address, port, app, response_gather_settings, response_timeout_settings)
}

fn get_response(address: &str, port: u16, app: App, gather_settings: GatheringSettings, timeout_settings: TimeoutSettings) -> GDResult<Response> {
    let client = ValveProtocol::new(address, port, timeout_settings)?;

    let info = client.get_server_info(&app)?;
    let protocol = info.protocol;

    if let App::Source(x) = &app {
        if let Some(appid) = x {
            if *appid != info.appid {
                return Err(GDError::BadGame(format!("Expected {}, found {} instead!", *appid, info.appid)));
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
            true => client.get_server_rules(&app, protocol)?
        }
    })
}
