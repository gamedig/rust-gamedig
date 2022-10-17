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

pub enum Request {
    INFO,
    PLAYER,
    RULES
}

#[derive(PartialEq)]
pub enum App {
    TF2 = 440,
    TheShip = 2400
}

pub struct ValveProtocol {
    socket: UdpSocket,
    complete_address: String
}

static DEFAULT_PACKET_SIZE: usize = 256;

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

    pub fn do_request(&self, kind: Request) -> Result<Vec<u8>, GDError> {
        let info_initial_packet = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x54, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x45, 0x6E, 0x67, 0x69, 0x6E, 0x65, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x00];
        let player_initial_packet = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x55];
        let rules_initial_packet = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x56];

        let no_challenge: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];

        let request_initial_packet = match kind {
            Request::INFO => info_initial_packet,
            Request::PLAYER => concat_u8_arrays(&player_initial_packet, &no_challenge),
            Request::RULES => concat_u8_arrays(&rules_initial_packet, &no_challenge)
        };

        self.send(&request_initial_packet)?;
        let buffer = self.receive(DEFAULT_PACKET_SIZE)?;

        if buffer.len() < 9 {
            return Err(GDError::PacketOverflow("Any Valve Protocol response can't be under 9 bytes long.".to_string()));
        }

        if buffer[4] != 41 { //'A'
            return Ok(buffer);
        }

        let challenge: [u8; 4] = [buffer[5], buffer[6], buffer[7], buffer[8]];
        self.send(&concat_u8_arrays(&request_initial_packet, &challenge))?;

        Ok(self.receive(DEFAULT_PACKET_SIZE)?)
    }
}

impl ValveProtocol {
    pub(crate) fn query(app: App, address: &str, port: u16, gather_players: bool, gather_rules: bool) -> Result<Response, GDError> {
        let client = ValveProtocol::new(address, port);

        let buf = client.do_request(Request::INFO)?;
        let mut pos = 4;

        Ok(Response {
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
            the_ship: match app == App::TheShip {
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
}
