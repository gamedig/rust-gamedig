use crate::{
    protocols::valve::{self, game, SteamApp},
    GDError::TypeParse,
    GDResult,
};
use std::net::{IpAddr, SocketAddr};

pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<game::Response> {
    let mut valve_response = valve::query(
        &SocketAddr::new(*address, port.unwrap_or(7780)),
        SteamApp::BAT1944.as_engine(),
        None,
        None,
    )?;

    if let Some(rules) = &mut valve_response.rules {
        if let Some(bat_max_players) = rules.get("bat_max_players_i") {
            valve_response.info.players_maximum = bat_max_players.parse().map_err(|_| TypeParse)?;
            rules.remove("bat_max_players_i");
        }

        if let Some(bat_player_count) = rules.get("bat_player_count_s") {
            valve_response.info.players_online = bat_player_count.parse().map_err(|_| TypeParse)?;
            rules.remove("bat_player_count_s");
        }

        if let Some(bat_has_password) = rules.get("bat_has_password_s") {
            valve_response.info.has_password = bat_has_password == "Y";
            rules.remove("bat_has_password_s");
        }

        if let Some(bat_name) = rules.get("bat_name_s") {
            valve_response.info.name = bat_name.clone();
            rules.remove("bat_name_s");
        }

        if let Some(bat_gamemode) = rules.get("bat_gamemode_s") {
            valve_response.info.game = bat_gamemode.clone();
            rules.remove("bat_gamemode_s");
        }

        rules.remove("bat_map_s");
    }

    Ok(game::Response::new_from_valve_response(valve_response))
}
