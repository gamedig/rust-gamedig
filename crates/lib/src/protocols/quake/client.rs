use byteorder::LittleEndian;

use crate::buffer::{Buffer, Utf8Decoder};
use crate::protocols::quake::types::Response;
use crate::socket::{Socket, UdpSocket};
use crate::utils::retry_on_timeout;
use crate::GDErrorKind::{PacketBad, TypeParse};
use crate::{GDErrorKind, GDResult, TimeoutSettings};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::slice::Iter;

pub trait QuakeClient {
    type Player;

    fn get_send_header<'a>() -> &'a str;
    fn get_response_header<'a>() -> &'a str;
    fn parse_player_string(data: Iter<&str>) -> GDResult<Self::Player>;
}

/// Send request and return result buffer.
/// This function will retry fetch on timeouts.
fn get_data<Client: QuakeClient>(
    address: &SocketAddr,
    timeout_settings: &Option<TimeoutSettings>,
) -> GDResult<Vec<u8>> {
    let mut socket = UdpSocket::new(address)?;
    socket.apply_timeout(timeout_settings)?;
    retry_on_timeout(
        TimeoutSettings::get_retries_or_default(timeout_settings),
        move || get_data_impl::<Client>(&mut socket),
    )
}

/// Send request and return result buffer (without retry logic).
fn get_data_impl<Client: QuakeClient>(socket: &mut UdpSocket) -> GDResult<Vec<u8>> {
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

    if bufferer.read::<u32>()? != u32::MAX {
        return Err(PacketBad.context("Expected 4294967295"));
    }

    let response_header = Client::get_response_header().as_bytes();
    if !bufferer.remaining_bytes().starts_with(response_header) {
        Err(GDErrorKind::PacketBad)?;
    }

    bufferer.move_cursor(response_header.len() as isize)?;

    Ok(bufferer.remaining_bytes().to_vec())
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

pub fn client_query<Client: QuakeClient>(
    address: &SocketAddr,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Response<Client::Player>> {
    let data = get_data::<Client>(address, &timeout_settings)?;
    let mut bufferer = Buffer::<LittleEndian>::new(&data);

    let mut server_vars = get_server_values(&mut bufferer)?;
    let players = get_players::<Client>(&mut bufferer)?;

    Ok(Response {
        name: server_vars
            .remove("hostname")
            .or(server_vars.remove("sv_hostname"))
            .ok_or(GDErrorKind::PacketBad)?,
        map: server_vars
            .remove("mapname")
            .or(server_vars.remove("map"))
            .ok_or(GDErrorKind::PacketBad)?,
        players_online: players.len() as u8,
        players_maximum: server_vars
            .remove("maxclients")
            .or(server_vars.remove("sv_maxclients"))
            .ok_or(GDErrorKind::PacketBad)?
            .parse()
            .map_err(|e| TypeParse.context(e))?,
        players,
        game_version: server_vars
            .remove("version")
            .or(server_vars.remove("*version")),
        unused_entries: server_vars,
    })
}

pub fn remove_wrapping_quotes<'a>(string: &&'a str) -> &'a str {
    match string.starts_with('\"') && string.ends_with('\"') {
        false => string,
        true => &string[1 .. string.len() - 1],
    }
}
