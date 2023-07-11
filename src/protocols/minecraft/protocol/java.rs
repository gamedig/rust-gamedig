use crate::{
    buffer::Buffer,
    protocols::{
        minecraft::{as_varint, get_string, get_varint, JavaResponse, Player, Server},
        types::TimeoutSettings,
    },
    socket::{Socket, TcpSocket},
    GDError::{JsonParse, PacketBad},
    GDResult,
};

use std::net::SocketAddr;

use byteorder::LittleEndian;
use serde_json::Value;

#[rustfmt::skip]
const PAYLOAD: [u8; 17] = [
    //Packet ID (0)
    0x00,
    //Protocol Version (-1 to determine version)
    0xFF, 0xFF, 0xFF, 0xFF, 0x0F,
    //Server address (can be anything)
    0x07, 0x47, 0x61, 0x6D, 0x65, 0x44, 0x69, 0x67,
    //Server port (can be anything)
    0x00, 0x00,
    //Next state (1 for status)
    0x01
];

pub struct Java {
    socket: TcpSocket,
}

impl Java {
    fn new(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = TcpSocket::new(address)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self { socket })
    }

    fn send(&mut self, data: Vec<u8>) -> GDResult<()> {
        self.socket
            .send(&[as_varint(data.len() as i32), data].concat())
    }

    fn receive(&mut self) -> GDResult<Vec<u8>> { self.socket.receive(None) }

    fn send_handshake(&mut self) -> GDResult<()> {
        self.send(PAYLOAD.to_vec())?;

        Ok(())
    }

    fn send_status_request(&mut self) -> GDResult<()> {
        self.send(
            [0x00] // Packet ID (0)
                .to_vec(),
        )?;

        Ok(())
    }

    fn send_ping_request(&mut self) -> GDResult<()> {
        self.send(
            [0x01] // Packet ID (1)
                .to_vec(),
        )?;

        Ok(())
    }

    fn get_info(&mut self) -> GDResult<JavaResponse> {
        self.send_handshake()?;
        self.send_status_request()?;
        self.send_ping_request()?;

        let socket_data = self.receive()?;
        let mut buffer = Buffer::<LittleEndian>::new(&socket_data);

        if get_varint(&mut buffer)? != 0 {
            // first var int is the packet id
            return Err(PacketBad);
        }

        let json_response = get_string(&mut buffer)?;
        let value_response: Value = serde_json::from_str(&json_response).map_err(|_| JsonParse)?;

        let version_name = value_response["version"]["name"]
            .as_str()
            .ok_or(PacketBad)?
            .to_string();
        let version_protocol = value_response["version"]["protocol"]
            .as_i64()
            .ok_or(PacketBad)? as i32;

        let max_players = value_response["players"]["max"].as_u64().ok_or(PacketBad)? as u32;
        let online_players = value_response["players"]["online"]
            .as_u64()
            .ok_or(PacketBad)? as u32;
        let sample_players: Option<Vec<Player>> = match value_response["players"]["sample"].is_null() {
            true => None,
            false => {
                Some({
                    let players_values = value_response["players"]["sample"]
                        .as_array()
                        .ok_or(PacketBad)?;

                    let mut players = Vec::with_capacity(players_values.len());
                    for player in players_values {
                        players.push(Player {
                            name: player["name"].as_str().ok_or(PacketBad)?.to_string(),
                            id: player["id"].as_str().ok_or(PacketBad)?.to_string(),
                        })
                    }

                    players
                })
            }
        };

        Ok(JavaResponse {
            version_name,
            version_protocol,
            players_maximum: max_players,
            players_online: online_players,
            players_sample: sample_players,
            description: value_response["description"].to_string(),
            favicon: value_response["favicon"].as_str().map(str::to_string),
            previews_chat: value_response["previewsChat"].as_bool(),
            enforces_secure_chat: value_response["enforcesSecureChat"].as_bool(),
            server_type: Server::Java,
        })
    }

    pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<JavaResponse> {
        Java::new(address, timeout_settings)?.get_info()
    }
}
