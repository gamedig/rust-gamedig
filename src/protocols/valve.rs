use std::net::UdpSocket;
use crate::errors::GDError;
use crate::utils::{combine_two_u8, complete_address, concat_u8, find_first_string, get_u64_from_buf};

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
                Some(value) => concat_u8(&default, &value)
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
    pub(crate) fn query(address: &str, port: u16, has_the_ship: bool) -> Result<Response, GDError> {
        let client = ValveProtocol::new(address, port);

        client.do_request(Request::A2sInfo(None), None);
        let mut buf = client.receive(DEFAULT_PACKET_SIZE);
        let mut pos = 5;

        if buf[4] == 0x41 {
            client.do_request(Request::A2sInfo(Some([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]])), None);
            buf = client.receive(DEFAULT_PACKET_SIZE);
        }

        let protocol = buf[5]; pos = pos + 1;

        let name = find_first_string(&buf[pos..]); pos = pos + name.len() + 1;
        let map = find_first_string(&buf[pos..]); pos = pos + map.len() + 1;
        let folder = find_first_string(&buf[pos..]); pos = pos + folder.len() + 1;
        let game = find_first_string(&buf[pos..]); pos = pos + game.len() + 1;

        let id = combine_two_u8(buf[pos + 1], buf[pos]); pos = pos + 2;
        let players = buf[pos]; pos = pos + 1;
        let max_players = buf[pos]; pos = pos + 1;
        let bots = buf[pos]; pos = pos + 1;

        let server_type = match buf[pos] as char {
            'd' => Server::Dedicated,
            'l' => Server::NonDedicated,
            _ => Server::SourceTV
        }; pos = pos + 1;

        let environment_type = match buf[pos] as char {
            'l' => Environment::Linux,
            'w' => Environment::Windows,
            _ => Environment::Mac
        }; pos = pos + 1;

        let has_password = buf[pos] == 1; pos = pos + 1;
        let vac_secured = buf[pos] == 1; pos = pos + 1;

        let the_ship = match has_the_ship {
            false => None,
            true => {
                let ship = TheShip {
                    mode: buf[pos],
                    witnesses: buf[pos + 1],
                    duration: buf[pos + 2]
                }; pos = pos + 3;
                Some(ship)
            }
        };

        let version = find_first_string(&buf[pos..]); pos = pos + version.len() + 1;

        pos = pos + 1; //look ahead
        let extra_data = match buf.get(pos - 1) {
            None => None,
            Some(value) => {
                let edf_port = match (value & 0x80) > 0 {
                    false => None,
                    true => {
                        let p = combine_two_u8(buf[pos + 1], buf[pos]); pos = pos + 2;
                        Some(p)
                    }
                };

                let steam_id = match (value & 0x10) > 0 {
                    false => None,
                    true => {
                        let p = get_u64_from_buf(&buf[pos..]); pos = pos + 8;
                        Some(p)
                    }
                };

                let (tv_port, tv_name) = match (value & 0x40) > 0 {
                    false => (None, None),
                    true => {
                        let tv_port = combine_two_u8(buf[pos + 1], buf[pos]); pos = pos + 2;
                        let tv_name = find_first_string(&buf[pos..]); pos = pos + tv_name.len() + 1;
                        (Some(tv_port), Some(tv_name))
                    }
                };

                let keywords = match (value & 0x20) > 0 {
                    false => None,
                    true => {
                        let keywords = find_first_string(&buf[pos..]); pos = pos + keywords.len() + 1;
                        Some(keywords)
                    }
                };

                let game_id = match (value & 0x01) > 0 {
                    false => None,
                    true => {
                        let game_id = get_u64_from_buf(&buf[pos..]); pos = pos + 8;
                        Some(game_id)
                    }
                };

                Some(ExtraData {
                    port: edf_port,
                    steam_id,
                    tv_port,
                    tv_name,
                    keywords,
                    game_id
                })
            }
        };

        Ok(Response {
            protocol,
            name,
            map,
            folder,
            game,
            id,
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
}
