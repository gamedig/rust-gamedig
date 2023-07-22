use byteorder::LittleEndian;

use crate::buffer::{Buffer, Utf8Decoder};
use crate::protocols::quake::types::Response;
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::{GDError, GDResult};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::slice::Iter;

pub(crate) trait QuakeClient {
    type Player;

    fn get_send_header<'a>() -> &'a str;
    fn get_response_header<'a>() -> &'a str;
    fn parse_player_string(data: Iter<&str>) -> GDResult<Self::Player>;
}

fn get_data<Client: QuakeClient>(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Vec<u8>> {
    let mut socket = UdpSocket::new(address)?;
    socket.apply_timeout(timeout_settings)?;

    socket.send(
        &[
            &[0xFF, 0xFF, 0xFF, 0xFF],
            Client::get_send_header().as_bytes(),
            &[0x00],
        ]
        .concat(),
    )?;

    let data = socket.receive(None)?;
    let mut bufferer = Buffer::<LittleEndian>::new(&data);

    if bufferer.read::<u32>()? != 4294967295 {
        return Err(GDError::PacketBad);
    }

    let response_header = Client::get_response_header().as_bytes();
    if !bufferer.remaining_bytes().starts_with(response_header) {
        Err(GDError::PacketBad)?
    }

    bufferer.move_cursor(response_header.len() as isize)?;

    Ok(bufferer.remaining_bytes().to_vec()) //TODO: Maybe fix?
}

fn get_server_values(bufferer: &mut Buffer<LittleEndian>) -> GDResult<HashMap<String, String>> {
    let data = bufferer.read_string::<Utf8Decoder>(Some([0x0A]))?;
    let mut data_split = data.split('\\').collect::<Vec<&str>>();
    if let Some(first) = data_split.first() {
        if first == &"" {
            data_split.remove(0);
        }
    }

    let values = data_split.chunks(2);

    let mut vars: HashMap<String, String> = HashMap::new();
    for data in values {
        let key = data.first();
        let value = data.get(1);

        if let Some(k) = key {
            if let Some(v) = value {
                vars.insert(k.to_string(), v.to_string());
            }
        }
    }

    Ok(vars)
}

fn get_players<Client: QuakeClient>(bufferer: &mut Buffer<LittleEndian>) -> GDResult<Vec<Client::Player>> {
    let mut players: Vec<Client::Player> = Vec::new();

    // this needs to be looked at again as theres no way to check if the buffer has
    // a remaining null byte the original code was:
    // while !bufferer.is_remaining_empty() && bufferer.remaining_data() != [0x00]
    while !bufferer.remaining_length() == 0 {
        let data = bufferer.read_string::<Utf8Decoder>(Some([0x0A]))?;
        let data_split = data.split(' ').collect::<Vec<&str>>();
        let data_iter = data_split.iter();

        players.push(Client::parse_player_string(data_iter)?);
    }

    Ok(players)
}

pub(crate) fn client_query<Client: QuakeClient>(
    address: &SocketAddr,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response<Client::Player>> {
    let data = get_data::<Client>(address, timeout_settings)?;
    let mut bufferer = Buffer::<LittleEndian>::new(&data);

    let mut server_vars = get_server_values(&mut bufferer)?;
    let players = get_players::<Client>(&mut bufferer)?;

    Ok(Response {
        name: server_vars
            .remove("hostname")
            .or(server_vars.remove("sv_hostname"))
            .ok_or(GDError::PacketBad)?,
        map: server_vars
            .remove("mapname")
            .or(server_vars.remove("map"))
            .ok_or(GDError::PacketBad)?,
        players_online: players.len() as u8,
        players_maximum: server_vars
            .remove("maxclients")
            .or(server_vars.remove("sv_maxclients"))
            .ok_or(GDError::PacketBad)?
            .parse()
            .map_err(|_| GDError::TypeParse)?,
        players,
        version: server_vars
            .remove("version")
            .or(server_vars.remove("*version")),
        unused_entries: server_vars,
    })
}

pub(crate) fn remove_wrapping_quotes<'a>(string: &&'a str) -> &'a str {
    match string.starts_with('\"') && string.ends_with('\"') {
        false => string,
        true => &string[1 .. string.len() - 1],
    }
}
