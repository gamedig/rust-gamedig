use std::net::UdpSocket;
use crate::{GDError, GDResult};
use crate::utils::{buffer, complete_address};

/// The type of the server.
#[derive(Debug)]
pub enum Server {
    Dedicated,
    NonDedicated,
    SourceTV
}

/// The Operating System that the server is on.
#[derive(Debug)]
pub enum Environment {
    Linux,
    Windows,
    Mac
}

/// A query response.
#[derive(Debug)]
pub struct Response {
    pub info: ServerInfo,
    pub players: Option<Vec<ServerPlayer>>,
    pub rules: Option<Vec<ServerRule>>
}

/// General server information's.
#[derive(Debug)]
pub struct ServerInfo {
    /// Protocol used by the server.
    pub protocol: u8,
    /// Name of the server.
    pub name: String,
    /// Map name.
    pub map: String,
    /// Name of the folder containing the game files.
    pub folder: String,
    /// Full name of the game.
    pub game: String,
    /// [Steam Application ID](https://developer.valvesoftware.com/wiki/Steam_Application_ID) of game.
    pub id: u16,
    /// Number of players on the server.
    pub players: u8,
    /// Maximum number of players the server reports it can hold.
    pub max_players: u8,
    /// Number of bots on the server.
    pub bots: u8,
    /// Dedicated, NonDedicated or SourceTV
    pub server_type: Server,
    /// The Operating System that the server is on.
    pub environment_type: Environment,
    /// Indicated whether the server requires a password.
    pub has_password: bool,
    /// Indicated whether the server uses VAC.
    pub vac_secured: bool,
    /// [The ship](https://developer.valvesoftware.com/wiki/The_Ship) extra data
    pub the_ship: Option<TheShip>,
    /// Version of the game installed on the server.
    pub version: String,
    /// Some extra data that the server might provide or not.
    pub extra_data: Option<ExtraData>
}

/// A server player.
#[derive(Debug)]
pub struct ServerPlayer {
    /// Player's name.
    pub name: String,
    /// General score.
    pub score: u32,
    /// How long they've been on the server for.
    pub duration: f32,
    /// Only for [the ship](https://developer.valvesoftware.com/wiki/The_Ship): deaths count
    pub deaths: Option<u32>, //the_ship
    /// Only for [the ship](https://developer.valvesoftware.com/wiki/The_Ship): money amount
    pub money: Option<u32>, //the_ship
}

/// A server rule.
#[derive(Debug)]
pub struct ServerRule {
    pub name: String,
    pub value: String
}

/// Only present for [the ship](https://developer.valvesoftware.com/wiki/The_Ship).
#[derive(Debug)]
pub struct TheShip {
    pub mode: u8,
    pub witnesses: u8,
    pub duration: u8
}

/// Some extra data that the server might provide or not.
#[derive(Debug)]
pub struct ExtraData {
    /// The server's game port number.
    pub port: Option<u16>,
    /// Server's SteamID.
    pub steam_id: Option<u64>,
    /// Spectator port number for SourceTV.
    pub tv_port: Option<u16>,
    /// Name of the spectator server for SourceTV.
    pub tv_name: Option<String>,
    /// Tags that describe the game according to the server.
    pub keywords: Option<String>,
    /// The server's 64-bit GameID.
    pub game_id: Option<u64>
}

/// The type of the request, see the [protocol](https://developer.valvesoftware.com/wiki/Server_queries).
#[derive(PartialEq, Clone)]
#[repr(u8)]
pub enum Request {
    /// Known as `A2S_INFO`
    INFO = 0x54,
    /// Known as `A2S_PLAYERS`
    PLAYERS = 0x55,
    /// Known as `A2S_RULES`
    RULES = 0x56
}

/// Supported app id's
#[derive(PartialEq)]
pub enum App {
    /// Counter-Strike: Source
    CSS = 240,
    /// Day of Defeat: Sourcec
    DODS = 300,
    /// Half-Life 2 Deathmatch
    HL2DM = 320,
    /// Team Fortress 2
    TF2 = 440,
    /// Left 4 Dead
    L4D = 500,
    /// Left 4 Dead
    L4D2 = 550,
    /// Counter-Strike: Global Offensive
    CSGO = 730,
    /// The Ship
    TS = 2400,
    /// Garry's Mod
    GM = 4000,
}

impl TryFrom<u16> for App {
    type Error = GDError;

    fn try_from(value: u16) -> GDResult<Self> {
        match value {
            x if x == App::CSS as u16 => Ok(App::CSS),
            x if x == App::HL2DM as u16 => Ok(App::HL2DM),
            x if x == App::DODS as u16 => Ok(App::DODS),
            x if x == App::TF2 as u16 => Ok(App::TF2),
            x if x == App::L4D as u16 => Ok(App::L4D),
            x if x == App::L4D2 as u16 => Ok(App::L4D2),
            x if x == App::CSGO as u16 => Ok(App::CSGO),
            x if x == App::TS as u16 => Ok(App::TS),
            x if x == App::GM as u16 => Ok(App::GM),
            _ => Err(GDError::UnknownEnumCast),
        }
    }
}

/// What data to gather, purely used only with the query function.
pub struct GatheringSettings {
    pub players: bool,
    pub rules: bool
}

pub struct ValveProtocol {
    socket: UdpSocket,
    complete_address: String
}

static DEFAULT_PACKET_SIZE: usize = 2048;

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
                Request::INFO => String::from("Source Engine Query.\0").into_bytes(),
                _ => vec![0xFF, 0xFF, 0xFF, 0xFF]
            }
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend(&self.header.to_be_bytes());
        buf.push(self.kind);
        buf.extend(&self.payload);

        println!("{:x?}", buf);

        buf
    }
}

#[derive(Debug)]
struct SplitPacketInfo {
    pub header: u32,
    pub id: u32,
    pub total: u8,
    pub number: u8,
    pub size: Option<u16>,
    pub packet: Packet
}

impl SplitPacketInfo {
    fn new(app: &App, buf: &[u8]) -> GDResult<Self> {
        let mut pos = 0;

        let header = buffer::get_u32_le(&buf, &mut pos)?;
        let id = buffer::get_u32_le(&buf, &mut pos)?;
        let total = buffer::get_u8(&buf, &mut pos)?;
        let number = buffer::get_u8(&buf, &mut pos)?;
        let size = match *app {
            App::CSS | App::TF2 => Some(buffer::get_u16_le(&buf, &mut pos)?),
            _ => None
        };

        let packet = match ((id >> 31) & 1) == 1 {
            false => Packet::new(&buf[pos..])?,
            true => {
                let compressed_size = buffer::get_u32_le(&buf, &mut pos)?;
                let compressed_crc32 = buffer::get_u32_le(&buf, &mut pos)?;

                //decompress...
                Packet::new(&buf[pos..])?
            }
        };

        Ok(Self {
            header,
            id,
            total,
            number,
            size,
            packet
        })
    }
}

impl ValveProtocol {
    fn new(address: &str, port: u16) -> Self {
        Self {
            socket: UdpSocket::bind("0.0.0.0:0").unwrap(),
            complete_address: complete_address(address, port)
        }
    }

    fn send(&self, data: &[u8]) -> GDResult<()> {
        self.socket.send_to(&data, &self.complete_address).map_err(|e| GDError::PacketSend(e.to_string()))?;
        Ok(())
    }

    fn receive_raw(&self, buffer_size: usize) -> GDResult<Vec<u8>> {
        let mut buf: Vec<u8> = vec![0; buffer_size];
        let (amt, _) = self.socket.recv_from(&mut buf.as_mut_slice()).map_err(|e| GDError::PacketReceive(e.to_string()))?;

        if amt < 9 {
            return Err(GDError::PacketUnderflow("Any Valve Protocol response can't be under 9 bytes long.".to_string()));
        }

        Ok(buf[..amt].to_vec())
    }

    fn receive(&self, app: &App, buffer_size: usize) -> GDResult<Packet> {
        let mut buf = self.receive_raw(buffer_size)?;

        if buf[0] == 0xFE { //the packet is split
            let initial_split_packet_info = SplitPacketInfo::new(app, &buf)?;
            let mut final_packet = initial_split_packet_info.packet.clone();

            for _ in 1..initial_split_packet_info.total {
                buf = self.receive_raw(buffer_size)?;
                let split_packet_info = SplitPacketInfo::new(app, &buf)?;
                final_packet.payload.extend(&split_packet_info.packet.payload);
            }

            Ok(final_packet)
        }
        else {
            Packet::new(&buf)
        }
    }

    /// Ask for a specific request only.
    pub fn get_request_data(&self, app: &App, kind: Request) -> GDResult<Vec<u8>> {
        let request_initial_packet = Packet::initial(kind.clone()).to_bytes();

        self.send(&request_initial_packet)?;
        let packet = self.receive(app, DEFAULT_PACKET_SIZE)?;

        if packet.kind != 0x41 { //'A'
            return Ok(packet.payload.clone());
        }

        let challenge = packet.payload;
        let challenge_packet = Packet::challenge(kind.clone(), challenge).to_bytes();

        self.send(&challenge_packet)?;
        Ok(self.receive(app, DEFAULT_PACKET_SIZE)?.payload)
    }

    /// Get the server information's.
    pub fn get_server_info(&self, app: &App) -> GDResult<ServerInfo> {
        let buf = self.get_request_data(app, Request::INFO)?;
        let mut pos = 0;

        Ok(ServerInfo {
            protocol: buffer::get_u8(&buf, &mut pos)?,
            name: buffer::get_string(&buf, &mut pos)?,
            map: buffer::get_string(&buf, &mut pos)?,
            folder: buffer::get_string(&buf, &mut pos)?,
            game: buffer::get_string(&buf, &mut pos)?,
            id: buffer::get_u16_le(&buf, &mut pos)?,
            players: buffer::get_u8(&buf, &mut pos)?,
            max_players: buffer::get_u8(&buf, &mut pos)?,
            bots: buffer::get_u8(&buf, &mut pos)?,
            server_type: match buffer::get_u8(&buf, &mut pos)? {
                100 => Server::Dedicated, //'d'
                108 => Server::NonDedicated, //'l'
                112 => Server::SourceTV, //'p'
                _ => Err(GDError::UnknownEnumCast)?
            },
            environment_type: match buffer::get_u8(&buf, &mut pos)? {
                108 => Environment::Linux, //'l'
                119 => Environment::Windows, //'w'
                109 | 111 => Environment::Mac, //'m' or 'o'
                _ => Err(GDError::UnknownEnumCast)?
            },
            has_password: buffer::get_u8(&buf, &mut pos)? == 1,
            vac_secured: buffer::get_u8(&buf, &mut pos)? == 1,
            the_ship: match *app == App::TS {
                false => None,
                true => Some(TheShip {
                    mode: buffer::get_u8(&buf, &mut pos)?,
                    witnesses: buffer::get_u8(&buf, &mut pos)?,
                    duration: buffer::get_u8(&buf, &mut pos)?
                })
            },
            version: buffer::get_string(&buf, &mut pos)?,
            extra_data: match buffer::get_u8(&buf, &mut pos) {
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
                        true => Some(buffer::get_u64_le(&buf, &mut pos)?)
                    }
                })
            }
        })
    }

    /// Get the server player's.
    pub fn get_server_players(&self, app: &App) -> GDResult<Vec<ServerPlayer>> {
        let buf = self.get_request_data(app, Request::PLAYERS)?;
        let mut pos = 0;

        let count = buffer::get_u8(&buf, &mut pos)?;
        let mut players: Vec<ServerPlayer> = Vec::new();

        for _ in 0..count {
            pos += 1; //skip the index byte
            players.push(ServerPlayer {
                name: buffer::get_string(&buf, &mut pos)?,
                score: buffer::get_u32_le(&buf, &mut pos)?,
                duration: buffer::get_f32_le(&buf, &mut pos)?,
                deaths: match *app == App::TS {
                    false => None,
                    true => Some(buffer::get_u32_le(&buf, &mut pos)?)
                },
                money: match *app == App::TS {
                    false => None,
                    true => Some(buffer::get_u32_le(&buf, &mut pos)?)
                }
            });
        }

        Ok(players)
    }

    /// Get the server rules's.
    pub fn get_server_rules(&self, app: &App) -> GDResult<Option<Vec<ServerRule>>> {
        if *app == App::CSGO { //cause csgo response here is broken after feb 21 2014
            return Ok(None);
        }

        let buf = self.get_request_data(app, Request::RULES)?;
        println!("{:02X?}", &buf);
        let mut pos = 0;

        let count = buffer::get_u16_le(&buf, &mut pos)?;
        let mut rules: Vec<ServerRule> = Vec::new();

        for i in 0..count - 1 {
            println!("{i}/{count}");
            rules.push(ServerRule {
                name: buffer::get_string(&buf, &mut pos)?,
                value: buffer::get_string(&buf, &mut pos)?
            })
        }

        Ok(Some(rules))
    }

    pub(crate) fn query(app: App, address: &str, port: u16, gather: GatheringSettings) -> Result<Response, GDError> {
        let client = ValveProtocol::new(address, port);

        let info = client.get_server_info(&app)?;

        App::try_from(info.id).map_err(|_| GDError::BadGame(format!("Found {} instead!", info.id)))?;

        Ok(Response {
            info,
            players: match gather.players {
                false => None,
                true => Some(client.get_server_players(&app)?)
            },
            rules: match gather.rules {
                false => None,
                true => client.get_server_rules(&app)?
            }
        })
    }
}
