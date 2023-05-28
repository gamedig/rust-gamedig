use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::slice::Iter;
use crate::bufferer::{Bufferer, Endianess};
use crate::{GDError, GDResult};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};

#[derive(Debug)]
pub struct Response<P> {
    pub name: String,
    pub map: String,
    pub players: Vec<P>,
    /// Number of players on the server.
    pub players_online: u8,
    /// Maximum number of players the server reports it can hold.
    pub players_maximum: u8,
    /// Indicates whether the server requires a password.
    pub has_password: bool,
    /// Indicates whether the server has cheats enabled.
    pub cheats_enabled: bool,
    pub frag_limit: u8,
    pub time_limit: u8,
    pub version: String,
    pub unused_entries: HashMap<String, String>,
}

pub(crate) trait Client {
    type Player;

    fn get_send_header(&self) -> String;
    fn validate_received_data(&self, bufferer: &mut Bufferer) -> GDResult<()>;
    fn parse_player_string(&self, data: Iter<&str>) -> GDResult<Self::Player>;
}

fn get_data(client: &impl Client, address: &Ipv4Addr, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Bufferer> {
    let mut socket = UdpSocket::new(address, port)?;
    socket.apply_timeout(timeout_settings)?;

    socket.send(&[&[0xFF, 0xFF, 0xFF, 0xFF], client.get_send_header().into_bytes().as_slice(), &[0x00]].concat())?;

    let data = socket.receive(None)?;
    let mut bufferer = Bufferer::new_with_data(Endianess::Little, &data);

    if bufferer.get_u32()? != 4294967295 {
        return Err(GDError::PacketBad);
    }

    client.validate_received_data(&mut bufferer)?;

    Ok(bufferer)
}

fn get_server_values(bufferer: &mut Bufferer) -> GDResult<HashMap<String, String>> {
    let data = bufferer.get_string_utf8_newline()?;
    let mut data_split = data.split("\\").collect::<Vec<&str>>();
    if let Some(first) = data_split.first() {
        if first == &"" {
            data_split.remove(0);
        }
    }

    let values = data_split.chunks(2);

    let mut vars: HashMap<String, String> = HashMap::new();
    for data in values {
        let key = data.get(0);
        let value = data.get(1);

        if let Some(k) = key {
            if let Some(v) = value {
                vars.insert(k.to_string(), v.to_string());
            }
        }
    }

    Ok(vars)
}

fn get_players<C: Client>(client: &C, bufferer: &mut Bufferer) -> GDResult<Vec<C::Player>> {
    let mut players: Vec<C::Player> = Vec::new();

    while !bufferer.is_remaining_empty() {
        let data = bufferer.get_string_utf8_newline()?;
        let data_split = data.split(" ").collect::<Vec<&str>>();
        let data_iter = data_split.iter();

        players.push(client.parse_player_string(data_iter)?)
    }

    Ok(players)
}

pub(crate) fn client_query<C: Client>(client: C, address: &Ipv4Addr, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response<C::Player>> {
    let mut bufferer = get_data(&client, address, port, timeout_settings)?;

    let mut server_vars = get_server_values(&mut bufferer)?;
    let players = get_players(&client, &mut bufferer)?;

    Ok(Response {
        name: server_vars.remove("hostname")
            .ok_or(GDError::PacketBad)?,
        map: server_vars.remove("mapname")
            .ok_or(GDError::PacketBad)?,
        players_online: players.len() as u8,
        players_maximum: server_vars.remove("maxclients")
            .ok_or(GDError::PacketBad)?
            .parse()
            .map_err(|_| GDError::TypeParse)?,
        has_password: server_vars.remove("needpass")
            .ok_or(GDError::PacketBad)? == "1",
        players,
        frag_limit: server_vars.remove("fraglimit")
            .ok_or(GDError::PacketBad)?
            .parse()
            .map_err(|_| GDError::TypeParse)?,
        time_limit: server_vars.remove("timelimit")
            .ok_or(GDError::PacketBad)?
            .parse()
            .map_err(|_| GDError::TypeParse)?,
        version: server_vars.remove("version")
            .ok_or(GDError::PacketBad)?,
        cheats_enabled: server_vars.remove("cheats")
            .ok_or(GDError::PacketBad)? == "1",
        unused_entries: server_vars,
    })
}
