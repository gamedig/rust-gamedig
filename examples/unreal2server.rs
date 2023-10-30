/// ! Pretend to be an unreal 2 server by using the ToPacket/FromPacket trait.
use std::fmt::Debug;
use std::net::{SocketAddr, UdpSocket};

use gamedig::protocols::types::{FromPacket, ToPacket};
use gamedig::protocols::unreal2::{
    MutatorsAndRules,
    PacketKind,
    PacketRequest,
    PacketResponse,
    Player,
    Players,
    ServerInfo,
};

type GResult<T> = Result<T, Box<dyn std::error::Error>>;

fn send_response<T: Into<PacketResponse<I>>, I: ToPacket + Debug>(
    socket: &UdpSocket,
    client: SocketAddr,
    response: T,
) -> GResult<()> {
    let response = response.into();

    println!("Sending: {:?}", response);

    socket.send_to(&response.as_packet()?, client)?;

    Ok(())
}

fn main() -> GResult<()> {
    let socket = UdpSocket::bind("0.0.0.0:7777")?;

    let mut buf = [0; 100];

    loop {
        let (size, client) = socket.recv_from(&mut buf)?;
        let request = PacketRequest::from_packet(&buf[.. size])?;
        println!("Recieved {:?}", request);

        match request.packet_type {
            PacketKind::ServerInfo => {
                send_response(
                    &socket,
                    client,
                    ServerInfo {
                        server_id: 69,
                        ip: String::from("test.server"),
                        game_port: 7776,
                        query_port: 7777,
                        name: String::from("Test server"),
                        map: String::from("No map"),
                        game_type: String::from("None"),
                        num_players: 1,
                        max_players: 5,
                    },
                )
            }
            PacketKind::MutatorsAndRules => send_response(&socket, client, MutatorsAndRules::default()),
            PacketKind::Players => {
                send_response(
                    &socket,
                    client,
                    Players {
                        players: vec![Player {
                            id: 6,
                            name: String::from("Tom"),
                            ping: u32::MAX,
                            score: -1,
                            stats_id: 1,
                        }],
                        bots: vec![],
                    },
                )
            }
        }?;
    }
}
