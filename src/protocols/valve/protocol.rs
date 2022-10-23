use std::net::UdpSocket;
use bzip2_rs::decoder::Decoder;
use crate::{GDError, GDResult};
use crate::protocols::valve::types::{App, Environment, ExtraData, GatheringSettings, Request, Response, Server, ServerInfo, ServerPlayer, ServerRule, TheShip};
use crate::utils::{buffer, complete_address};

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
    fn new(_appid: u32, buf: &[u8]) -> GDResult<Self> {
        let mut pos = 0;

        let header = buffer::get_u32_le(&buf, &mut pos)?;
        let id = buffer::get_u32_le(&buf, &mut pos)?;
        let total = buffer::get_u8(&buf, &mut pos)?;
        let number = buffer::get_u8(&buf, &mut pos)?;
        let size = buffer::get_u16_le(&buf, &mut pos)?; //if game is CSS and if protocol is 7, queries with multi-packet responses will crash
        let compressed = ((id >> 31) & 1) == 1;
        let (decompressed_size, uncompressed_crc32) = match compressed {
            false => (None, None),
            true => (Some(buffer::get_u32_le(&buf, &mut pos)?), Some(buffer::get_u32_le(&buf, &mut pos)?))
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

    fn decompress(&self) -> GDResult<Vec<u8>> {
        if !self.compressed {
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
        } else { //already decompressed
            Ok(self.payload.clone())
        }
    }

    fn get_payload(&self) -> GDResult<Vec<u8>> {
        if self.compressed {
            Ok(self.decompress()?)
        } else {
            Ok(self.payload.clone())
        }
    }
}

struct ValveProtocol {
    socket: UdpSocket,
    complete_address: String
}

static DEFAULT_PACKET_SIZE: usize = 2048;

impl ValveProtocol {
    fn new(address: &str, port: u16) -> GDResult<Self> {
        Ok(Self {
            socket: UdpSocket::bind("0.0.0.0:0").unwrap(),
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

    fn receive(&self, appid: u32, buffer_size: usize) -> GDResult<Packet> {
        let mut buf = self.receive_raw(buffer_size)?;

        if buf[0] == 0xFE { //the packet is split
            let mut main_packet = SplitPacket::new(appid, &buf)?;

            for _ in 1..main_packet.total {
                buf = self.receive_raw(buffer_size)?;
                let chunk_packet = SplitPacket::new(appid, &buf)?;
                main_packet.payload.extend(chunk_packet.payload);
            }

            Ok(Packet::new(&main_packet.get_payload()?)?)
        }
        else {
            Packet::new(&buf)
        }
    }

    /// Ask for a specific request only.
    fn get_request_data(&self, appid: u32, kind: Request) -> GDResult<Vec<u8>> {
        let request_initial_packet = Packet::initial(kind.clone()).to_bytes();

        self.send(&request_initial_packet)?;
        let packet = self.receive(appid, DEFAULT_PACKET_SIZE)?;

        if packet.kind != 0x41 { //'A'
            return Ok(packet.payload.clone());
        }

        let challenge = packet.payload;
        let challenge_packet = Packet::challenge(kind.clone(), challenge).to_bytes();

        self.send(&challenge_packet)?;
        Ok(self.receive(appid, DEFAULT_PACKET_SIZE)?.payload)
    }

    /// Get the server information's.
    fn get_server_info(&self, initial_appid: u32) -> GDResult<ServerInfo> {
        let buf = self.get_request_data(initial_appid, Request::INFO)?;
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
            112 => Server::SourceTV, //'p'
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
        let the_ship = match appid == App::TS as u32 {
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
            extra_data
        })
    }

    /// Get the server player's.
    fn get_server_players(&self, appid: u32) -> GDResult<Vec<ServerPlayer>> {
        let buf = self.get_request_data(appid, Request::PLAYERS)?;
        let mut pos = 0;

        let count = buffer::get_u8(&buf, &mut pos)?;
        let mut players: Vec<ServerPlayer> = Vec::new();

        for _ in 0..count {
            pos += 1; //skip the index byte
            players.push(ServerPlayer {
                name: buffer::get_string(&buf, &mut pos)?,
                score: buffer::get_u32_le(&buf, &mut pos)?,
                duration: buffer::get_f32_le(&buf, &mut pos)?,
                deaths: match appid == App::TS as u32 {
                    false => None,
                    true => Some(buffer::get_u32_le(&buf, &mut pos)?)
                },
                money: match appid == App::TS as u32 {
                    false => None,
                    true => Some(buffer::get_u32_le(&buf, &mut pos)?)
                }
            });
        }

        Ok(players)
    }

    /// Get the server rules's.
    fn get_server_rules(&self, appid: u32) -> GDResult<Option<Vec<ServerRule>>> {
        if appid == App::CSGO as u32 { //cause csgo wont respond to this since feb 21 2014 update
            return Ok(None);
        }

        let buf = self.get_request_data(appid, Request::RULES)?;
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

/// Query a server, you need to provide the address, the port and optionally, the app and the
/// gather settings, the app being *None* means to anonymously query the server, and the gather
/// settings being *None* means to get the players and the rules.
pub fn query(address: &str, port: u16, app: Option<App>, gather_settings: Option<GatheringSettings>) -> Result<Response, GDError> {
    let client = ValveProtocol::new(address, port)?;

    let mut query_app_id = match app {
        None => 0,
        Some(app) => app as u32
    };

    let info = client.get_server_info(query_app_id)?;

    if query_app_id != 0 {
        if info.appid != query_app_id {
            return Err(GDError::BadGame(format!("Expected {}, found {} instead!", query_app_id, info.appid)));
        }
    } else {
        query_app_id = info.appid;
    }

    let (gather_players, gather_rules) = match gather_settings.is_some() {
        false => (true, true),
        true => {
            let settings = gather_settings.unwrap();
            (settings.players, settings.rules)
        }
    };

    Ok(Response {
        info,
        players: match gather_players {
            false => None,
            true => Some(client.get_server_players(query_app_id)?)
        },
        rules: match gather_rules {
            false => None,
            true => client.get_server_rules(query_app_id)?
        }
    })
}
