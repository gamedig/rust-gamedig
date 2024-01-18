use crate::buffer::{Buffer, Utf8Decoder};
use crate::protocols::gamespy::two::{Player, Response, Team};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::utils::retry_on_timeout;
use crate::GDErrorKind::{PacketBad, TypeParse};
use crate::{GDErrorKind, GDResult};
use byteorder::BigEndian;
use std::collections::HashMap;
use std::net::SocketAddr;

struct GameSpy2 {
    socket: UdpSocket,
    retry_count: usize,
}

macro_rules! table_extract {
    ($table:expr, $name:literal, $index:expr) => {
        $table
            .get($name)
            .ok_or(GDErrorKind::PacketBad)?
            .get($index)
            .ok_or(GDErrorKind::PacketBad)?
    };
}

macro_rules! table_extract_parse {
    ($table:expr, $name:literal, $index:expr) => {
        table_extract!($table, $name, $index)
            .parse()
            .map_err(|e| PacketBad.context(e))?
    };
}

fn data_as_table(data: &mut Buffer<BigEndian>) -> GDResult<(HashMap<String, Vec<String>>, usize)> {
    if data.read::<u8>()? != 0 {
        Err(GDErrorKind::PacketBad)?;
    }

    let rows = data.read::<u8>()? as usize;

    if rows == 0 {
        return Ok((HashMap::new(), 0));
    }

    let mut column_heads = Vec::new();

    let mut current_column = data.read_string::<Utf8Decoder>(None)?;
    while !current_column.is_empty() {
        column_heads.push(current_column);
        current_column = data.read_string::<Utf8Decoder>(None)?;
    }

    let columns = column_heads.len();
    let mut table = HashMap::with_capacity(columns);
    for head in &column_heads {
        // TODO: This doesn't look good nor it is performant, fix later
        // By using &column_heads in the for loop instead of cloning column_heads, you
        // avoid creating an unnecessary copy. However, column_heads is a
        // Vec<String> and head is a &String (a reference to a string). Hence, to use
        // head as a key to the HashMap, we still need to call clone(). This is because
        // HashMap takes ownership of its keys and we cannot give it a reference to a
        // local variable (head) that will be dropped at the end of the function.
        table.insert(head.clone(), Vec::new());
    }

    for _ in 0 .. rows {
        for column in &column_heads {
            let value = data.read_string::<Utf8Decoder>(None)?;
            table
                .get_mut(column)
                .ok_or(GDErrorKind::PacketBad)?
                .push(value);
        }
    }

    Ok((table, rows))
}

impl GameSpy2 {
    fn new(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address, &timeout_settings)?;
        let retry_count = TimeoutSettings::get_retries_or_default(&timeout_settings);

        Ok(Self {
            socket,
            retry_count,
        })
    }

    /// Send fetch request to server and store result in buffer.
    /// This function will retry fetch on timeouts.
    fn request_data(&mut self) -> GDResult<(Vec<u8>, usize)> {
        retry_on_timeout(self.retry_count, move || self.request_data_impl())
    }

    /// Send fetch request to server and store result in buffer (without retry
    /// logic).
    fn request_data_impl(&mut self) -> GDResult<(Vec<u8>, usize)> {
        self.socket
            .send(&[0xFE, 0xFD, 0x00, 0x00, 0x00, 0x00, 0x01, 0xFF, 0xFF, 0xFF])?;

        let received = self.socket.receive(None)?;

        let mut buf = Buffer::<BigEndian>::new(&received);
        if buf.read::<u8>()? != 0 || buf.read::<u32>()? != 1 {
            return Err(PacketBad.into());
        }

        let buf_index = buf.current_position();
        Ok((received, buf_index))
    }
}

fn get_server_vars(bufferer: &mut Buffer<BigEndian>) -> GDResult<HashMap<String, String>> {
    let mut values = HashMap::new();

    let mut done_processing_vars = false;
    while !done_processing_vars && bufferer.remaining_length() != 0 {
        let key = bufferer.read_string::<Utf8Decoder>(None)?;
        let value = bufferer.read_string::<Utf8Decoder>(None)?;

        if key.is_empty() {
            if value.is_empty() {
                bufferer.move_cursor(-1)?;
                done_processing_vars = true;
            }

            continue;
        }

        values.insert(key, value);
    }

    Ok(values)
}

fn get_teams(bufferer: &mut Buffer<BigEndian>) -> GDResult<Vec<Team>> {
    let mut teams = Vec::new();

    let (table, entries) = data_as_table(bufferer)?;

    for index in 0 .. entries {
        teams.push(Team {
            name: table_extract!(table, "team_t", index).clone(),
            score: table_extract_parse!(table, "score_t", index),
        });
    }

    Ok(teams)
}

fn get_players(bufferer: &mut Buffer<BigEndian>) -> GDResult<Vec<Player>> {
    let mut players = Vec::new();

    let (table, entries) = data_as_table(bufferer)?;

    for index in 0 .. entries {
        players.push(Player {
            name: table_extract!(table, "player_", index).clone(),
            score: table_extract_parse!(table, "score_", index),
            ping: table_extract_parse!(table, "ping_", index),
            team_index: table_extract_parse!(table, "team_", index),
        });
    }

    Ok(players)
}

pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let mut client = GameSpy2::new(address, timeout_settings)?;
    let (data, buf_index) = client.request_data()?;

    let mut buffer = Buffer::<BigEndian>::new(&data);
    buffer.move_cursor(buf_index as isize)?;

    let mut server_vars = get_server_vars(&mut buffer)?;
    let players = get_players(&mut buffer)?;

    let players_online = match server_vars.remove("numplayers") {
        None => players.len(),
        Some(v) => {
            let reported_players = v.parse().map_err(|e| TypeParse.context(e))?;
            match reported_players < players.len() {
                true => players.len(),
                false => reported_players,
            }
        }
    } as u32;
    let players_minimum = match server_vars.remove("minplayers") {
        None => None,
        Some(v) => Some(v.parse::<u32>().map_err(|e| TypeParse.context(e))?),
    };

    Ok(Response {
        name: server_vars.remove("hostname").ok_or(PacketBad)?,
        map: server_vars.remove("mapname").ok_or(PacketBad)?,
        has_password: server_vars.remove("password").ok_or(PacketBad)? == "1",
        teams: get_teams(&mut buffer)?,
        players_maximum: server_vars
            .remove("maxplayers")
            .ok_or(PacketBad)?
            .parse()
            .map_err(|e| TypeParse.context(e))?,
        players_online,
        players_minimum,
        players,
        unused_entries: server_vars,
    })
}
