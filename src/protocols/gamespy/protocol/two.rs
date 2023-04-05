use std::{
    collections::{HashMap, HashSet},
    net::Ipv4Addr,
};

use crate::{
    bufferer::{Bufferer, Endianess},
    protocols::gamespy::types::two::{
        player_info::{Player, PlayerInfo},
        server_info::{
            CameraSettings,
            FriendlyFireSettings,
            GameInfo,
            GameStartSettings,
            GameplaySettings,
            ServerConfig,
            ServerConnection,
            ServerInfo,
            TeamSettings,
        },
        team_info::TeamInfo,
        Flag,
        Response,
        REQUEST_PACKET_BYTES,
    },
    socket::{Socket, UdpSocket},
    GDResult,
};

pub struct GameSpy2 {
    address: String,
    port: u16,
}

impl GameSpy2 {
    pub fn new(address: Ipv4Addr, port: u16) -> Self {
        Self {
            address: address.to_string(),
            port,
        }
    }

    pub fn request(&self) -> GDResult<RawResponse> {
        let mut socket = UdpSocket::new(&self.address, self.port)?;

        socket.send(&REQUEST_PACKET_BYTES)?;

        let data = socket.receive(None)?;

        Ok(RawResponse { data })
    }
}

/// A raw response from a GameSpy2 server
pub struct RawResponse {
    /// The raw response data
    pub data: Vec<u8>,
}

impl RawResponse {
    fn parse_server_data(bufferer: &mut Bufferer) -> ServerInfo {
        let mut server_data = HashMap::new();
        while let Ok(key) = bufferer.get_string_utf8() {
            if !key.is_empty() {
                if let Ok(value) = bufferer.get_string_utf8() {
                    server_data.insert(key, value);
                }
            }
        }
        let connection = ServerConnection {
            hostname: server_data.remove("hostname").unwrap_or_default(),
            hostport: server_data
                .remove("hostport")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            password: Flag::from_str(server_data.remove("password").unwrap_or_default().as_str()),
        };

        let game_info = GameInfo {
            map_name: server_data.remove("mapname").unwrap_or_default(),
            game_type: server_data.remove("gametype").unwrap_or_default(),
            num_players: server_data
                .remove("numplayers")
                .and_then(|s| s.parse().ok())
                .unwrap_or_default(),
            max_players: server_data
                .remove("maxplayers")
                .and_then(|s| s.parse().ok())
                .unwrap_or_default(),

            game_mode: server_data.remove("gamemode").unwrap_or_default(),
            game_version: server_data.remove("gamever").unwrap_or_default(),
            status: server_data
                .remove("status")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            game_id: server_data.remove("game_id").unwrap_or_default(),
            map_id: server_data.remove("map_id").unwrap_or_default(),
        };

        let gameplay_settings = GameplaySettings {
            punkbuster: server_data
                .remove("sv_punkbuster")
                .map(|s| Flag::from_str(&s)),
            time_limit: server_data.remove("timelimit"),
            num_rounds: server_data
                .remove("number_of_rounds")
                .and_then(|s| s.parse().ok()),
            spawn_wave_time: server_data
                .remove("spawn_wave_time")
                .and_then(|s| s.parse().ok()),
            spawn_delay: server_data
                .remove("spawn_delay")
                .and_then(|s| s.parse().ok()),
        };

        let friendly_fire_settings = FriendlyFireSettings {
            soldier_friendly_fire: server_data
                .remove("soldier_friendly_fire")
                .map(|s| Flag::from_str(&s)),
            vehicle_friendly_fire: server_data
                .remove("vehicle_friendly_fire")
                .map(|s| Flag::from_str(&s)),
            soldier_friendly_fire_on_splash: server_data.remove("soldier_friendly_fire_on_splash"),
            vehicle_friendly_fire_on_splash: server_data.remove("vehicle_friendly_fire_on_splash"),
        };

        let team_settings = TeamSettings {
            us_team_ratio: server_data
                .remove("us_team_ratio")
                .and_then(|s| s.parse().ok()),
            nva_team_ratio: server_data
                .remove("nva_team_ratio")
                .and_then(|s| s.parse().ok()),
        };

        let camera_settings = CameraSettings {
            name_tag_distance: server_data
                .remove("name_tag_distance")
                .and_then(|s| s.parse().ok()),
            name_tag_distance_scope: server_data
                .remove("name_tag_distance_scope")
                .and_then(|s| s.parse().ok()),
            allow_nose_cam: server_data.remove("allow_nose_cam"),
            external_view: server_data.remove("external_view"),
            free_camera: server_data
                .remove("free_camera")
                .map(|s| Flag::from_str(&s)),
        };

        let game_start_settings = GameStartSettings {
            game_start_delay: server_data
                .remove("game_start_delay")
                .and_then(|s| s.parse().ok()),
            ticket_ratio: server_data.remove("ticket_ratio"),
            kickback: server_data.remove("kickback"),
            kickback_on_splash: server_data.remove("kickback_on_splash"),
            auto_balance: server_data
                .remove("auto_balance_teams")
                .map(|s| Flag::from_str(&s)),
        };

        let config = ServerConfig {
            dedicated: server_data.remove("dedicated").map(|s| Flag::from_str(&s)),
            cpu: server_data.remove("cpu").and_then(|s| s.parse().ok()),
            bot_skill: server_data.remove("bot_skill"),
            reserved_slots: server_data
                .remove("reservedslots")
                .and_then(|s| s.parse().ok()),
            active_mods: server_data.remove("active_mods"),
        };

        let server_info = ServerInfo {
            connection,
            game_info,
            gameplay_settings,
            friendly_fire_settings,
            team_settings,
            camera_settings,
            game_start_settings,
            config,
        };

        server_info
    }

    fn parse_score_set(bufferer: &mut Bufferer) -> HashSet<String> {
        let mut score_set = HashSet::new();
        while let Ok(score_name) = bufferer.get_string_utf8() {
            if !score_name.is_empty() {
                score_set.insert(score_name);
            }
        }
        score_set
    }

    fn parse_players(bufferer: &mut Bufferer, player_count: u8, score_set: &HashSet<String>) -> Vec<Player> {
        let mut players = Vec::new();
        for _ in 0 .. player_count {
            let mut player = HashMap::new();
            for score_name in score_set {
                if let Ok(score_value) = bufferer.get_string_utf8() {
                    player.insert(score_name.clone(), score_value);
                }
            }

            players.push(Player {
                name: player.remove("player_").unwrap_or_default(),
                score: player.remove("score_").and_then(|s| s.parse().ok()),
                deaths: player.remove("deaths_").and_then(|s| s.parse().ok()),
                ping: player.remove("ping_").and_then(|s| s.parse().ok()),
                team: player.remove("team_").and_then(|s| s.parse().ok()),
                kills: player.remove("kills_").and_then(|s| s.parse().ok()),
            });
        }
        players
    }

    /// Parses the raw response data and returns a `Response` object
    pub fn parse(&self) -> GDResult<Response> {
        let mut bufferer = Bufferer::new_with_data(Endianess::Little, &self.data);

        // Skip the header
        bufferer.move_position_ahead(5);

        // Parse the key/value strings pairs, ending with an empty key and value
        let server_info = Self::parse_server_data(&mut bufferer);

        // Skip empty key/value
        bufferer.move_position_ahead(2);

        // Parse the player count and score names
        let player_count = bufferer.get_u8()?;
        let score_set = Self::parse_score_set(&mut bufferer);

        // Skip empty byte
        bufferer.move_position_ahead(1);

        // Parse the players
        let players = Self::parse_players(&mut bufferer, player_count, &score_set);

        // Skip empty byte
        bufferer.move_position_ahead(1);

        // ! Team structure is unknown
        let teams = bufferer.get_string_utf8_unended()?;

        let team_info = TeamInfo { raw: teams };

        Ok(Response {
            server_info,
            player_info: PlayerInfo {
                players,
                player_count,
            },
            team_info,
        })
    }
}
