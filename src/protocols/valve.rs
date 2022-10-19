use std::collections::HashMap;
use std::net::UdpSocket;
use crate::errors::GDError;
use crate::utils::{buffer, complete_address, concat_u8_arrays};

#[derive(Debug)]
pub enum Server {
    Dedicated,
    NonDedicated,
    SourceTV
}

#[derive(Debug)]
pub enum Environment {
    Linux,
    Windows,
    Mac
}

#[derive(Debug)]
pub struct Response {
    pub info: ServerInfo,
    pub players: Option<ServerPlayers>,
    pub rules: Option<ServerRules>
}

#[derive(Debug)]
pub struct ServerInfo {
    pub protocol: u8,
    pub map: String,
    pub name: String,
    pub folder: String,
    pub game: String,
    pub id: u16,
    pub players: u8,
    pub max_players: u8,
    pub bots: u8,
    pub server_type: Server,
    pub environment_type: Environment,
    pub has_password: bool,
    pub vac_secured: bool,
    pub the_ship: Option<TheShip>,
    pub version: String,
    pub extra_data: Option<ExtraData>
}

#[derive(Debug)]
pub struct ServerPlayers {
    pub count: u8,
    pub players: Vec<Player>
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub duration: f32,
    pub deaths: Option<u32>, //the_ship
    pub money: Option<u32>, //the_ship
}

#[derive(Debug)]
pub struct ServerRules {
    pub count: u16,
    pub map: HashMap<String, String>
}

#[derive(Debug)]
pub struct TheShip {
    pub mode: u8,
    pub witnesses: u8,
    pub duration: u8
}

#[derive(Debug)]
pub struct ExtraData {
    pub port: Option<u16>,
    pub steam_id: Option<u64>,
    pub tv_port: Option<u16>,
    pub tv_name: Option<String>,
    pub keywords: Option<String>,
    pub game_id: Option<u64>
}

#[derive(PartialEq)]
pub enum Request {
    INFO,
    PLAYERS,
    RULES
}

#[derive(PartialEq)]
pub enum App {
    TF2 = 440,
    CSGO = 730,
    TheShip = 2400
}

impl TryFrom<u16> for App {
    type Error = GDError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == App::TF2 as u16 => Ok(App::TF2),
            x if x == App::CSGO as u16 => Ok(App::CSGO),
            x if x == App::TheShip as u16 => Ok(App::TheShip),
            _ => Err(GDError::UnknownEnumCast),
        }
    }
}

pub struct GatheringSettings {
    pub players: bool,
    pub rules: bool
}

pub struct ValveProtocol {
    socket: UdpSocket,
    complete_address: String
}

static DEFAULT_PACKET_SIZE: usize = 2048;

impl ValveProtocol {
    fn new(address: &str, port: u16) -> Self {
        Self {
            socket: UdpSocket::bind("0.0.0.0:0").unwrap(),
            complete_address: complete_address(address, port)
        }
    }

    fn send(&self, data: &[u8]) -> Result<(), GDError> {
        self.socket.send_to(&data, &self.complete_address).map_err(|e| GDError::PacketSend(e.to_string()))?;
        Ok(())
    }

    fn receive(&self, buffer_size: usize) -> Result<Vec<u8>, GDError> {
        let mut buffer: Vec<u8> = vec![0; buffer_size];
        let (amt, _) = self.socket.recv_from(&mut buffer.as_mut_slice()).map_err(|e| GDError::PacketReceive(e.to_string()))?;
        Ok(buffer[..amt].to_vec())
    }

    fn receive_truncated(&self, initial_packet: &[u8]) -> Result<Vec<u8>, GDError> {
        let count = initial_packet[8] - 1;
        let mut final_packet: Vec<u8> = initial_packet.to_vec().drain(17..).collect::<Vec<u8>>();

        for _ in 0..count {
            let mut packet = self.receive(DEFAULT_PACKET_SIZE)?;
            final_packet.append(&mut packet.drain(13..).collect::<Vec<u8>>());
        }

        Ok(final_packet)
    }

    pub fn get_request_data(&self, kind: Request) -> Result<Vec<u8>, GDError> {
        let info_initial_packet = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x54, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x45, 0x6E, 0x67, 0x69, 0x6E, 0x65, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x00];
        let players_initial_packet = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x55, 0xFF, 0xFF, 0xFF, 0xFF];
        let rules_initial_packet = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x56, 0xFF, 0xFF, 0xFF, 0xFF];

        let request_initial_packet = match kind {
            Request::INFO => info_initial_packet,
            Request::PLAYERS => players_initial_packet,
            Request::RULES => rules_initial_packet
        };

        self.send(&request_initial_packet)?;
        let mut initial_receive = self.receive(DEFAULT_PACKET_SIZE)?;

        if initial_receive.len() < 9 {
            return Err(GDError::PacketOverflow("Any Valve Protocol response can't be under 9 bytes long.".to_string()));
        }

        if initial_receive[4] != 0x41 { //'A'
            return Ok(initial_receive.drain(5..).collect());
        }

        let challenge: [u8; 4] = [initial_receive[5], initial_receive[6], initial_receive[7], initial_receive[8]];
        let challenge_packet = match kind {
            Request::INFO => concat_u8_arrays(&request_initial_packet, &challenge),
            Request::PLAYERS => vec![0xFF, 0xFF, 0xFF, 0xFF, 0x55, challenge[0], challenge[1], challenge[2], challenge[3]],
            Request::RULES => vec![0xFF, 0xFF, 0xFF, 0xFF, 0x56, challenge[0], challenge[1], challenge[2], challenge[3]]
        };

        self.send(&challenge_packet)?;

        let mut packet = self.receive(DEFAULT_PACKET_SIZE)?;
        if packet[0] == 0xFE || (packet[0] == 0xFF && packet[4] == 0x45) { //'E'
            self.receive_truncated(&packet)
        } else {
            Ok(packet.drain(5..).collect::<Vec<u8>>())
        }
    }

    fn get_server_info(&self, app: &App) -> Result<ServerInfo, GDError> {
        let buf = self.get_request_data(Request::INFO)?;
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
                _ => Server::SourceTV //'p'
            },
            environment_type: match buffer::get_u8(&buf, &mut pos)? {
                100 => Environment::Linux, //'l'
                119 => Environment::Windows, //'w'
                _ => Environment::Mac //'m' or 'o'
            },
            has_password: buffer::get_u8(&buf, &mut pos)? == 1,
            vac_secured: buffer::get_u8(&buf, &mut pos)? == 1,
            the_ship: match *app == App::TheShip {
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

    fn get_server_players(&self, app: &App) -> Result<ServerPlayers, GDError> {
        let buf = self.get_request_data(Request::PLAYERS)?;
        let mut pos = 0;

        let count = buffer::get_u8(&buf, &mut pos)?;
        let mut players: Vec<Player> = Vec::new();

        for _ in 0..count {
            pos += 1; //skip the index byte
            players.push(Player {
                name: buffer::get_string(&buf, &mut pos)?,
                score: buffer::get_u32_le(&buf, &mut pos)?,
                duration: buffer::get_f32_le(&buf, &mut pos)?,
                deaths: match *app == App::TheShip {
                    false => None,
                    true => Some(buffer::get_u32_le(&buf, &mut pos)?)
                },
                money: match *app == App::TheShip {
                    false => None,
                    true => Some(buffer::get_u32_le(&buf, &mut pos)?)
                }
            });
        }

        Ok(ServerPlayers {
            count,
            players
        })
    }

    fn get_server_rules(&self) -> Result<ServerRules, GDError> {
        let buf = self.get_request_data(Request::RULES)?;
        let mut pos = 0;

        let count = buffer::get_u16_le(&buf, &mut pos)?;
        let mut rules: HashMap<String, String> = HashMap::new();

        for _ in 0..count {
            rules.insert(buffer::get_string(&buf, &mut pos)?,   //name
                         buffer::get_string(&buf, &mut pos)?);  //value
        }

        Ok(ServerRules {
            count,
            map: rules
        })
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
                true => Some(client.get_server_rules()?)
            }
        })
    }
}
