use std::net::UdpSocket;
use crate::errors::GDError;
use crate::protocol::Protocol;
use crate::utils::{combine_two_u8, complete_address, concat_u8, find_null_in_array};

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
    pub vac_secured: bool
}

pub enum Request {
    A2sInfo(Option<[u8; 4]>)
}

pub struct ValveProtocol {
    socket: UdpSocket,
    complete_address: String
}

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

impl Protocol for ValveProtocol {
    type Response = Response;

    fn query(address: &str, port: u16) -> Result<Response, GDError> {
        let client = ValveProtocol::new(address, port);

        client.do_request(Request::A2sInfo(None), None);
        let mut buf = client.receive();

        if buf[4] == 0x41 {
            client.do_request(Request::A2sInfo(Some([buf[5], buf[6], buf[7], buf[8]])), None);
        }

        buf = client.receive_with_size(256);
        println!("{:x?}", &buf);

        let name_nul_pos = find_null_in_array(&mut buf);
        let map_nul_pos = name_nul_pos + 1 + find_null_in_array(&mut buf[name_nul_pos + 1..]);
        let folder_nul_pos = map_nul_pos + 1 + find_null_in_array(&mut buf[map_nul_pos + 1..]);
        let game_nul_pos = folder_nul_pos + 1 + find_null_in_array(&mut buf[folder_nul_pos + 1..]);

        let server_type = match buf[game_nul_pos + 6] as char {
            'd' => Server::Dedicated,
            'l' => Server::NonDedicated,
            _ => Server::SourceTV
        };

        let environment_type = match buf[game_nul_pos + 7] as char {
            'l' => Environment::Linux,
            'w' => Environment::Windows,
            _ => Environment::Mac
        };

        Ok(Response {
            protocol: buf[5],
            name: String::from_utf8(Vec::from(&mut buf[6..name_nul_pos])).expect("cacat"),
            map: String::from_utf8(Vec::from(&mut buf[name_nul_pos + 1..map_nul_pos])).expect("cacat"),
            folder: String::from_utf8(Vec::from(&mut buf[map_nul_pos + 1..folder_nul_pos])).expect("cacat"),
            game: String::from_utf8(Vec::from(&mut buf[folder_nul_pos + 1..game_nul_pos])).expect("cacat"),
            id: combine_two_u8(buf[game_nul_pos + 2], buf[game_nul_pos + 1]),
            players: buf[game_nul_pos + 3],
            max_players: buf[game_nul_pos + 4],
            bots: buf[game_nul_pos + 5],
            server_type,
            environment_type,
            has_password: buf[game_nul_pos + 8] == 1,
            vac_secured: buf[game_nul_pos + 9] != 0
        })
    }
}
