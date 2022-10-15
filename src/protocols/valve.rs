use std::net::UdpSocket;
use crate::errors::GDError;
use crate::utils::{combine_two_u8, complete_address, concat_u8, find_null_in_array, get_u64_from_buf};

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

static default_packet_size: usize = 256;

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

    pub fn receive(&self) -> Vec<u8> {
        self.receive_with_size(64)
    }

    pub fn receive_with_size(&self, buffer_size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; buffer_size];
        let (amt, _) = self.socket.recv_from(&mut buffer.as_mut_slice()).unwrap();
        buffer[..amt].to_vec()
    }
}

impl ValveProtocol {
    pub(crate) fn query(address: &str, port: u16, has_the_ship: bool) -> Result<Response, GDError> {
        let client = ValveProtocol::new(address, port);

        client.do_request(Request::A2sInfo(None), None);
        let mut buf = client.receive_with_size(default_packet_size);

        if buf[4] == 0x41 {
            client.do_request(Request::A2sInfo(Some([buf[5], buf[6], buf[7], buf[8]])), None);
            buf = client.receive_with_size(default_packet_size);
        }

        println!("{:x?}", &buf);

        let name_null_pos = find_null_in_array(&mut buf);
        let map_null_pos = name_null_pos + 1 + find_null_in_array(&mut buf[name_null_pos + 1..]);
        let folder_null_pos = map_null_pos + 1 + find_null_in_array(&mut buf[map_null_pos + 1..]);
        let game_null_pos = folder_null_pos + 1 + find_null_in_array(&mut buf[folder_null_pos + 1..]);

        let server_type = match buf[game_null_pos + 6] as char {
            'd' => Server::Dedicated,
            'l' => Server::NonDedicated,
            _ => Server::SourceTV
        };

        let environment_type = match buf[game_null_pos + 7] as char {
            'l' => Environment::Linux,
            'w' => Environment::Windows,
            _ => Environment::Mac
        };

        let mut the_ship_index = game_null_pos + 10;
        let the_ship = match has_the_ship {
            false => None,
            true => {
                let ship = TheShip {
                    mode: buf[the_ship_index],
                    witnesses: buf[the_ship_index + 1],
                    duration: buf[the_ship_index + 2]
                };
                the_ship_index = the_ship_index + 3;
                Some(ship)
            }
        };

        let version_null_pos = the_ship_index + find_null_in_array(&mut buf[the_ship_index..]);
        let extra_data = match buf.get(version_null_pos + 1) {
            None => None,
            Some(value) => {
                let mut last_edf_position = version_null_pos + 2;
                let edf_port = match (value & 0x80) > 0 {
                    false => None,
                    true => {
                        let p = combine_two_u8(buf[last_edf_position + 1], buf[last_edf_position]);
                        last_edf_position = last_edf_position + 2;
                        Some(p)
                    }
                };

                let steam_id = match (value & 0x10) > 0 { //doesnt work?
                    false => None,
                    true => {
                        let p = get_u64_from_buf(&buf[last_edf_position..]);
                        last_edf_position = last_edf_position + 8;
                        Some(p)
                    }
                };

                let (tv_port, tv_name) = match (value & 0x40) > 0 {
                    false => (None, None),
                    true => {
                        let port = combine_two_u8(buf[last_edf_position + 1], buf[last_edf_position]);
                        last_edf_position = last_edf_position + 2;
                        let tv_name_null_pos = last_edf_position + find_null_in_array(&buf[last_edf_position..]);
                        let tv_name = String::from_utf8(Vec::from(&buf[last_edf_position..tv_name_null_pos])).expect("cacat");
                        last_edf_position = tv_name_null_pos + 1;
                        (Some(port), Some(tv_name))
                    }
                };

                let keywords = match (value & 0x20) > 0 {
                    false => None,
                    true => {
                        let kws_null_pos = last_edf_position + find_null_in_array(&buf[last_edf_position..]);
                        let kws = String::from_utf8(Vec::from(&buf[last_edf_position..kws_null_pos])).expect("cacat");
                        last_edf_position = kws_null_pos + 1;
                        Some(kws)
                    }
                };

                let game_id = match (value & 0x01) > 0 {
                    false => None,
                    true => Some(get_u64_from_buf(&buf[last_edf_position..]))
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
            protocol: buf[5],
            name: String::from_utf8(Vec::from(&mut buf[6..name_null_pos])).expect("cacat"),
            map: String::from_utf8(Vec::from(&mut buf[name_null_pos + 1..map_null_pos])).expect("cacat"),
            folder: String::from_utf8(Vec::from(&mut buf[map_null_pos + 1..folder_null_pos])).expect("cacat"),
            game: String::from_utf8(Vec::from(&mut buf[folder_null_pos + 1..game_null_pos])).expect("cacat"),
            id: combine_two_u8(buf[game_null_pos + 2], buf[game_null_pos + 1]),
            players: buf[game_null_pos + 3],
            max_players: buf[game_null_pos + 4],
            bots: buf[game_null_pos + 5],
            server_type,
            environment_type,
            has_password: buf[game_null_pos + 8] == 1,
            vac_secured: buf[game_null_pos + 9] != 0,
            the_ship,
            version: String::from_utf8(Vec::from(&mut buf[the_ship_index..version_null_pos])).expect("cacat"),
            extra_data
        })
    }
}
