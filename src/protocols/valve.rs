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
    A2sInfo(Option<[u8; 4]>)
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

    pub fn do_request(&self, kind: Request, data_packet: Option<&[u8]>) -> bool {
        let default: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x54, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20,
            0x45, 0x6E, 0x67, 0x69, 0x6E, 0x65, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x00];

        let request_kind_packet = match kind {
            Request::A2sInfo(challenge) => match challenge {
                None => default,
                Some(value) => concat_u8_arrays(&default, &value)
            }
        };

        let mut packet = request_kind_packet;
        match data_packet {
            None => (),
            Some(data) => packet.extend_from_slice(data)
        }

        match self.socket.send_to(&packet, &self.complete_address) {
            Err(_) => false,
            Ok(_) => true
        }
    }

    pub fn receive(&self, buffer_size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; buffer_size];
        let (amt, _) = self.socket.recv_from(&mut buffer.as_mut_slice()).unwrap();
        buffer[..amt].to_vec()
    }
}

impl ValveProtocol {
    pub(crate) fn query(app: App, address: &str, port: u16) -> Result<Response, GDError> {
        let client = ValveProtocol::new(address, port);

        client.do_request(Request::A2sInfo(None), None);
        let mut buf = client.receive(DEFAULT_PACKET_SIZE);
        let mut pos = 4;

        if buffer::get_u8(&buf, &mut pos)? == 0x41 {
            client.do_request(Request::A2sInfo(Some([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]])), None);
            buf = client.receive(DEFAULT_PACKET_SIZE);
        }

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
            server_type: match buffer::get_u8(&buf, &mut pos)? as char {
                'd' => Server::Dedicated,
                'l' => Server::NonDedicated,
                _ => Server::SourceTV
            },
            environment_type: match buffer::get_u8(&buf, &mut pos)? as char {
                'l' => Environment::Linux,
                'w' => Environment::Windows,
                _ => Environment::Mac
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
