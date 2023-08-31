use crate::{
    buffer::Buffer,
    protocols::{
        minecraft::{as_varint, get_string, get_varint, JavaResponse, Player, Server},
        types::TimeoutSettings,
    },
    socket::{Socket, TcpSocket},
    GDErrorKind::{JsonParse, PacketBad},
    GDResult,
};

use std::net::SocketAddr;

use crate::protocols::minecraft::as_string;
use byteorder::LittleEndian;
use serde_json::Value;

pub struct Java {
    socket: TcpSocket,
    request_settings: RequestSettings,
}

impl Java {
    fn new(
        address: &SocketAddr,
        timeout_settings: Option<TimeoutSettings>,
        request_settings: Option<RequestSettings>,
    ) -> GDResult<Self> {
        let socket = TcpSocket::new(address)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self {
            socket,
            request_settings: request_settings.unwrap_or_default(),
        })
    }

    fn send(&mut self, data: Vec<u8>) -> GDResult<()> {
        self.socket
            .send(&[as_varint(data.len() as i32), data].concat())
    }

    fn receive(&mut self) -> GDResult<Vec<u8>> {
        let data = &self.socket.receive(None)?;
        let mut buffer = Buffer::<LittleEndian>::new(data);

        let _packet_length = get_varint(&mut buffer)? as usize;
        // this declared 'packet length' from within the packet might be wrong (?), not
        // checking with it...

        Ok(buffer.remaining_bytes().to_vec())
    }

    fn send_handshake(&mut self) -> GDResult<()> {
        let handshake_payload = [
            &[
                // Packet ID (0)
                0x00,
            ], // Protocol Version (-1 to determine version)
            as_varint(self.request_settings.protocol_version).as_slice(),
            // Server address (can be anything)
            as_string(&self.request_settings.query_address).as_slice(),
            // Server port (can be anything)
            &[
                0x00, 0x00, // Next state (1 for status)
                0x01,
            ],
        ]
        .concat();

        self.send(handshake_payload)?;

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
            return Err(PacketBad.context("Expected 0"));
        }

        let json_response = get_string(&mut buffer)?;
        let value_response: Value = serde_json::from_str(&json_response).map_err(|e| JsonParse.context(e))?;

        let game_version = value_response["version"]["name"]
            .as_str()
            .ok_or(PacketBad)?
            .to_string();
        let protocol_version = value_response["version"]["protocol"]
            .as_i64()
            .ok_or(PacketBad)? as i32;

        let max_players = value_response["players"]["max"].as_u64().ok_or(PacketBad)? as u32;
        let online_players = value_response["players"]["online"]
            .as_u64()
            .ok_or(PacketBad)? as u32;
        let players: Option<Vec<Player>> = match value_response["players"]["sample"].is_null() {
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
                        });
                    }

                    players
                })
            }
        };

        Ok(JavaResponse {
            game_version,
            protocol_version,
            players_maximum: max_players,
            players_online: online_players,
            players,
            description: value_response["description"].to_string(),
            favicon: value_response["favicon"].as_str().map(str::to_string),
            previews_chat: value_response["previewsChat"].as_bool(),
            enforces_secure_chat: value_response["enforcesSecureChat"].as_bool(),
            server_type: Server::Java,
        })
    }

    pub fn query(
        address: &SocketAddr,
        timeout_settings: Option<TimeoutSettings>,
        request_settings: Option<RequestSettings>,
    ) -> GDResult<JavaResponse> {
        Self::new(address, timeout_settings, request_settings)?.get_info()
    }
}

struct RequestSettings {
    pub query_address: String,
    pub protocol_version: i32,
}

impl Default for RequestSettings {
    fn default() -> Self {
        Self {
            query_address: "gamedig".to_string(),
            protocol_version: -1,
        }
    }
}
