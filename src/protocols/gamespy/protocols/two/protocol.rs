use crate::bufferer::{Bufferer, Endianess};
use crate::protocols::gamespy::two::{Player, Response, Team};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::{GDError, GDResult};
use std::collections::HashMap;
use std::net::SocketAddr;

struct GameSpy2 {
    socket: UdpSocket,
}

macro_rules! table_extract {
    ($table:expr, $name:literal, $index:expr) => {
        $table
            .get($name)
            .ok_or(GDError::PacketBad)?
            .get($index)
            .ok_or(GDError::PacketBad)?
    };
}

macro_rules! table_extract_parse {
    ($table:expr, $name:literal, $index:expr) => {
        table_extract!($table, $name, $index)
            .parse()
            .map_err(|_| GDError::PacketBad)?
    };
}

fn data_as_table(data: &mut Bufferer) -> GDResult<(HashMap<String, Vec<String>>, usize)> {
    if data.get_u8()? != 0 {
        Err(GDError::PacketBad)?
    }

    let rows = data.get_u8()? as usize;

    if rows == 0 {
        return Ok((HashMap::new(), 0));
    }

    let mut column_heads = Vec::new();

    let mut current_column = data.get_string_utf8()?;
    while !current_column.is_empty() {
        column_heads.push(current_column.clone());
        current_column = data.get_string_utf8()?;
    }

    let columns = column_heads.len();
    let mut table = HashMap::with_capacity(columns);
    for head in &column_heads {
        table.insert(head.clone(), Vec::new());
    }

    for _ in 0 .. rows {
        for column_index in 0 .. columns {
            let value = data.get_string_utf8()?;
            table
                .get_mut(&*column_heads[column_index])
                .ok_or(GDError::PacketBad)?
                .push(value);
        }
    }

    Ok((table, rows))
}

impl GameSpy2 {
    fn new(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self { socket })
    }

    fn request_data(&mut self) -> GDResult<Bufferer> {
        self.socket
            .send(&[0xFE, 0xFD, 0x00, 0x00, 0x00, 0x00, 0x01, 0xFF, 0xFF, 0xFF])?;

        let received = self.socket.receive(None)?;
        let mut buf = Bufferer::new_with_data(Endianess::Big, &received);

        if buf.get_u8()? != 0 {
            return Err(GDError::PacketBad);
        }

        if buf.get_u32()? != 1 {
            return Err(GDError::PacketBad);
        }

        Ok(buf)
    }
}

fn get_server_vars(bufferer: &mut Bufferer) -> GDResult<HashMap<String, String>> {
    let mut values = HashMap::new();

    let mut done_processing_vars = false;
    while !done_processing_vars && bufferer.remaining_length() > 0 {
        let key = bufferer.get_string_utf8()?;
        let value = bufferer.get_string_utf8_optional()?;

        if key.is_empty() {
            if value.is_empty() {
                bufferer.move_position_backward(1);
                done_processing_vars = true;
            }

            continue;
        }

        values.insert(key, value);
    }

    Ok(values)
}

fn get_teams(bufferer: &mut Bufferer) -> GDResult<Vec<Team>> {
    let mut teams = Vec::new();

    let (table, entries) = data_as_table(bufferer)?;

    for index in 0 .. entries {
        teams.push(Team {
            name: table_extract!(table, "team_t", index).clone(),
            score: table_extract_parse!(table, "score_t", index),
        })
    }

    Ok(teams)
}

fn get_players(bufferer: &mut Bufferer) -> GDResult<Vec<Player>> {
    let mut players = Vec::new();

    let (table, entries) = data_as_table(bufferer)?;

    for index in 0 .. entries {
        players.push(Player {
            name: table_extract!(table, "player_", index).clone(),
            score: table_extract_parse!(table, "score_", index),
            ping: table_extract_parse!(table, "ping_", index),
            team_index: table_extract_parse!(table, "team_", index),
        })
    }

    Ok(players)
}

pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let mut client = GameSpy2::new(address, timeout_settings)?;
    let mut data = client.request_data()?;
    let mut server_vars = get_server_vars(&mut data)?;
    let players = get_players(&mut data)?;

    let players_online = match server_vars.remove("numplayers") {
        None => players.len(),
        Some(v) => {
            let reported_players = v.parse().map_err(|_| GDError::TypeParse)?;
            match reported_players < players.len() {
                true => players.len(),
                false => reported_players,
            }
        }
    };
    let players_minimum = match server_vars.remove("minplayers") {
        None => None,
        Some(v) => Some(v.parse::<u8>().map_err(|_| GDError::TypeParse)?),
    };

    Ok(Response {
        name: server_vars.remove("hostname").ok_or(GDError::PacketBad)?,
        map: server_vars.remove("mapname").ok_or(GDError::PacketBad)?,
        has_password: server_vars.remove("password").ok_or(GDError::PacketBad)? == "1",
        teams: get_teams(&mut data)?,
        players_maximum: server_vars
            .remove("maxplayers")
            .ok_or(GDError::PacketBad)?
            .parse()
            .map_err(|_| GDError::PacketBad)?,
        players_online,
        players_minimum,
        players,
        unused_entries: server_vars,
    })
}
