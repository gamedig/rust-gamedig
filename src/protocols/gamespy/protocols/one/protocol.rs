use crate::{
    bufferer::{Bufferer, Endianess},
    protocols::{
        gamespy::one::{Player, Response},
        types::TimeoutSettings,
    },
    socket::{Socket, UdpSocket},
    GDError,
    GDResult,
};

use crate::protocols::gamespy::common::has_password;
use std::collections::HashMap;

fn get_server_values(
    address: &str,
    port: u16,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<HashMap<String, String>> {
    let mut socket = UdpSocket::new(address, port)?;
    socket.apply_timeout(timeout_settings)?;

    socket.send("\\status\\xserverquery".as_bytes())?;

    let mut received_query_id: Option<usize> = None;
    let mut parts: Vec<usize> = Vec::new();
    let mut is_finished = false;

    let mut server_values = HashMap::new();

    while !is_finished {
        let data = socket.receive(None)?;
        let mut bufferer = Bufferer::new_with_data(Endianess::Little, &data);

        let mut as_string = bufferer.get_string_utf8_unended()?;
        as_string.remove(0);

        let splited: Vec<String> = as_string.split('\\').map(str::to_string).collect();

        for i in 0 .. splited.len() / 2 {
            let position = i * 2;
            let key = splited[position].clone();
            let value = match splited.get(position + 1) {
                None => "".to_string(),
                Some(v) => v.clone(),
            };

            server_values.insert(key, value);
        }

        is_finished = server_values.contains_key("final");
        server_values.remove("final");

        let query_data = server_values.get("queryid");

        let mut part = parts.len(); // if the part number isn't provided, it's value is the parts length
        let mut query_id = None;
        if let Some(qid) = query_data {
            let split: Vec<&str> = qid.split('.').collect();

            query_id = Some(split[0].parse().map_err(|_| GDError::TypeParse)?);
            match split.len() {
                1 => (),
                2 => part = split[1].parse().map_err(|_| GDError::TypeParse)?,
                _ => Err(GDError::PacketBad)?, /* the queryid can't be splitted in more than 2
                                                * elements */
            };
        }

        server_values.remove("queryid");

        if received_query_id.is_some() && received_query_id != query_id {
            return Err(GDError::PacketBad); // wrong query id!
        } else {
            received_query_id = query_id;
        }

        match parts.contains(&part) {
            true => Err(GDError::PacketBad)?,
            false => parts.push(part),
        }
    }

    Ok(server_values)
}

fn extract_players(server_vars: &mut HashMap<String, String>, players_maximum: usize) -> GDResult<Vec<Player>> {
    let mut players_data: Vec<HashMap<String, String>> = Vec::with_capacity(players_maximum);

    server_vars.retain(|key, value| {
        let split: Vec<&str> = key.split('_').collect();

        if split.len() != 2 {
            return true;
        }

        let kind = split[0];
        let id: usize = match split[1].parse() {
            Ok(v) => v,
            Err(_) => return true,
        };

        let early_return = match kind {
            "team" | "player" | "ping" | "face" | "skin" | "mesh" | "frags" | "ngsecret" | "deaths" | "health" => false,
            _x => true, // println!("UNKNOWN {id} {x} {value}");
        };

        if early_return {
            return true;
        }

        if id >= players_data.len() {
            let others = vec![HashMap::new(); id - players_data.len() + 1];
            players_data.extend_from_slice(&others);
        }
        players_data[id].insert(kind.to_string(), value.to_string());

        false
    });

    let mut players: Vec<Player> = Vec::with_capacity(players_data.len());

    for player_data in players_data {
        let new_player = Player {
            name: match player_data.get("player") {
                Some(v) => v.clone(),
                None => {
                    player_data
                        .get("playername")
                        .ok_or(GDError::PacketBad)?
                        .clone()
                }
            },
            team: player_data
                .get("team")
                .ok_or(GDError::PacketBad)?
                .trim()
                .parse()
                .map_err(|_| GDError::TypeParse)?,
            ping: player_data
                .get("ping")
                .ok_or(GDError::PacketBad)?
                .trim()
                .parse()
                .map_err(|_| GDError::TypeParse)?,
            face: player_data.get("face").ok_or(GDError::PacketBad)?.clone(),
            skin: player_data.get("skin").ok_or(GDError::PacketBad)?.clone(),
            mesh: player_data.get("mesh").ok_or(GDError::PacketBad)?.clone(),
            frags: player_data
                .get("frags")
                .ok_or(GDError::PacketBad)?
                .trim()
                .parse()
                .map_err(|_| GDError::TypeParse)?,
            deaths: match player_data.get("deaths") {
                Some(v) => Some(v.trim().parse().map_err(|_| GDError::TypeParse)?),
                None => None,
            },
            health: match player_data.get("health") {
                Some(v) => Some(v.trim().parse().map_err(|_| GDError::TypeParse)?),
                None => None,
            },
            secret: player_data
                .get("ngsecret")
                .ok_or(GDError::PacketBad)?
                .to_lowercase()
                .parse()
                .map_err(|_| GDError::TypeParse)?,
        };

        players.push(new_player);
    }

    Ok(players)
}

/// If there are parsing problems using the `query` function, you can directly
/// get the server's values using this function.
pub fn query_vars(
    address: &str,
    port: u16,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<HashMap<String, String>> {
    get_server_values(address, port, timeout_settings)
}

/// Query a server by providing the address, the port and timeout settings.
/// Providing None to the timeout settings results in using the default values.
/// (TimeoutSettings::[default](TimeoutSettings::default)).
pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let mut server_vars = query_vars(address, port, timeout_settings)?;

    let players_maximum = server_vars
        .remove("maxplayers")
        .ok_or(GDError::PacketBad)?
        .parse()
        .map_err(|_| GDError::TypeParse)?;
    let players_minimum = match server_vars.remove("minplayers") {
        None => None,
        Some(v) => Some(v.parse::<u8>().map_err(|_| GDError::TypeParse)?),
    };

    let players = extract_players(&mut server_vars, players_maximum)?;

    Ok(Response {
        name: server_vars.remove("hostname").ok_or(GDError::PacketBad)?,
        map: server_vars.remove("mapname").ok_or(GDError::PacketBad)?,
        map_title: server_vars.remove("maptitle"),
        admin_contact: server_vars.remove("AdminEMail"),
        admin_name: server_vars
            .remove("AdminName")
            .or_else(|| server_vars.remove("admin")),
        has_password: has_password(&mut server_vars)?,
        game_type: server_vars.remove("gametype").ok_or(GDError::PacketBad)?,
        game_version: server_vars.remove("gamever").ok_or(GDError::PacketBad)?,
        players_maximum,
        players_online: players.len(),
        players_minimum,
        players,
        tournament: server_vars
            .remove("tournament")
            .unwrap_or("true".to_string())
            .to_lowercase()
            .parse()
            .map_err(|_| GDError::TypeParse)?,
        unused_entries: server_vars,
    })
}
