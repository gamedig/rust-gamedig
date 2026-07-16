use std::net::{IpAddr, SocketAddr};

use crate::fivem::{DynamicInfo, FiveMRequestSettings, Info, Player, Response};
use crate::http::HttpClient;
use crate::protocols::types::GatherToggle;
use crate::{GDResult, TimeoutSettings};

/// The default FiveM server port.
pub const DEFAULT_PORT: u16 = 30120;
/// Maximum accepted size for each FiveM JSON response: 16 MiB.
const MAX_JSON_RESPONSE_LENGTH: usize = 16 * 1024 * 1024;

/// Query a FiveM server.
#[inline]
pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> { query_with_timeout(address, port, &None) }

/// Query a FiveM server.
#[inline]
pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: &Option<TimeoutSettings>,
) -> GDResult<Response> {
    query_with_timeout_and_extra_settings(address, port, timeout_settings, None)
}

/// Query a FiveM server with request-specific settings.
pub fn query_with_timeout_and_extra_settings(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: &Option<TimeoutSettings>,
    extra_settings: Option<FiveMRequestSettings>,
) -> GDResult<Response> {
    let address = SocketAddr::new(*address, port.unwrap_or(DEFAULT_PORT));
    let extra_settings = extra_settings.unwrap_or_default();
    let gather_players = extra_settings.gather_players();
    let players_token = extra_settings.players_token().map(str::to_owned);
    let mut client = HttpClient::new(&address, timeout_settings, extra_settings.into())?;

    let info = client.get_json_with_max_length::<Info>("/info.json", None, MAX_JSON_RESPONSE_LENGTH)?;
    let dynamic = client
        .get_json_with_max_length::<DynamicInfo>("/dynamic.json", None, MAX_JSON_RESPONSE_LENGTH)
        .ok();
    let players = match gather_players {
        GatherToggle::Skip => None,
        GatherToggle::Try => query_players(&mut client, players_token.as_deref()).ok(),
        GatherToggle::Enforce => Some(query_players(&mut client, players_token.as_deref())?),
    };

    Ok((info, dynamic, players).into())
}

fn query_players(client: &mut HttpClient, players_token: Option<&str>) -> GDResult<Vec<Player>> {
    if let Some(players_token) = players_token {
        let headers = [("X-Players-Token", players_token)];
        client.get_json_with_max_length("/players.json", Some(&headers), MAX_JSON_RESPONSE_LENGTH)
    } else {
        client.get_json_with_max_length("/players.json", None, MAX_JSON_RESPONSE_LENGTH)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{ErrorKind, Read, Write};
    use std::net::{Ipv4Addr, TcpListener};
    use std::thread;
    use std::time::{Duration, Instant};

    use super::*;
    use crate::protocols::types::CommonResponse;

    const INFO_RESPONSE: &str = r#"{
        "enhancedHostSupport": true,
        "requestSteamTicket": "unset",
        "resources": ["hardcap"],
        "server": "FXServer-test",
        "vars": {"sv_maxClients": "32"},
        "version": 12345
    }"#;
    const DYNAMIC_RESPONSE: &str = r#"{
        "hostname": "Mock FiveM",
        "gametype": "Freeroam",
        "mapname": "San Andreas",
        "clients": 2,
        "iv": "12345",
        "sv_maxclients": "32"
    }"#;
    const PLAYERS_RESPONSE: &str = r#"[
        {"endpoint":"127.0.0.1","id":1,"identifiers":["license:one"],"name":"One","ping":10},
        {"endpoint":"127.0.0.1","id":2,"identifiers":["license:two"],"name":"Two","ping":20}
    ]"#;

    fn spawn_server(
        players_succeed: bool,
        expected_players_token: Option<&'static str>,
    ) -> (IpAddr, u16, thread::JoinHandle<()>) {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("mock server must bind");
        listener
            .set_nonblocking(true)
            .expect("mock server must become nonblocking");
        let port = listener
            .local_addr()
            .expect("mock address must exist")
            .port();

        let handle = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(5);
            let mut requests_handled = 0;

            while requests_handled < 3 && Instant::now() < deadline {
                let (mut stream, _) = match listener.accept() {
                    Ok(connection) => connection,
                    Err(error) if error.kind() == ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(5));
                        continue;
                    }
                    Err(error) => panic!("mock server accept failed: {error}"),
                };

                let mut request = Vec::new();
                let mut buffer = [0_u8; 1024];
                while !request.windows(4).any(|window| window == b"\r\n\r\n") {
                    let length = stream
                        .read(&mut buffer)
                        .expect("mock request must be readable");
                    if length == 0 {
                        break;
                    }
                    request.extend_from_slice(&buffer[.. length]);
                }

                let request = String::from_utf8(request).expect("mock request must be UTF-8");
                let path = request
                    .lines()
                    .next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .expect("mock request path must exist");
                let (status, body) = match path {
                    "/info.json" => ("200 OK", INFO_RESPONSE),
                    "/dynamic.json" => ("200 OK", DYNAMIC_RESPONSE),
                    "/players.json" => {
                        let actual_token = request.lines().find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("X-Players-Token")
                                .then_some(value.trim())
                        });
                        assert_eq!(actual_token, expected_players_token);

                        if players_succeed {
                            ("200 OK", PLAYERS_RESPONSE)
                        } else {
                            ("403 Forbidden", "Nope.")
                        }
                    }
                    other => panic!("unexpected mock request path: {other}"),
                };
                let response = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: \
                     close\r\n\r\n{body}",
                    body.len()
                );
                stream
                    .write_all(response.as_bytes())
                    .expect("mock response must be writable");
                requests_handled += 1;
            }

            assert_eq!(
                requests_handled, 3,
                "mock server did not receive every request"
            );
        });

        (IpAddr::V4(Ipv4Addr::LOCALHOST), port, handle)
    }

    fn spawn_oversized_info_server() -> (IpAddr, u16, thread::JoinHandle<()>) {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("mock server must bind");
        let port = listener
            .local_addr()
            .expect("mock address must exist")
            .port();
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("mock server must accept");
            let mut request = [0_u8; 1024];
            let _ = stream
                .read(&mut request)
                .expect("mock request must be readable");
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                MAX_JSON_RESPONSE_LENGTH + 1
            );
            stream
                .write_all(response.as_bytes())
                .expect("mock response must be writable");
        });

        (IpAddr::V4(Ipv4Addr::LOCALHOST), port, handle)
    }

    #[test]
    fn queries_sparse_info_and_authenticates_player_request() {
        let (address, port, server) = spawn_server(true, Some("secret"));
        let settings = FiveMRequestSettings::default().set_players_token("secret".to_owned());

        let response = query_with_timeout_and_extra_settings(&address, Some(port), &None, Some(settings))
            .expect("mock query must succeed");
        server.join().expect("mock server must succeed");

        assert_eq!(response.info.icon, None);
        assert_eq!(response.info.vars.tags, None);
        assert_eq!(
            response.dynamic.as_ref().map(|dynamic| dynamic.clients),
            Some(2)
        );
        assert_eq!(response.players.as_ref().map(Vec::len), Some(2));
    }

    #[test]
    fn optional_player_failure_keeps_server_metadata() {
        let (address, port, server) = spawn_server(false, None);

        let response = query(&address, Some(port)).expect("optional player failure must not fail the query");
        server.join().expect("mock server must succeed");

        assert_eq!(response.players, None);
        assert_eq!(response.players_online(), 2);
        assert_eq!(response.players_maximum(), 32);
    }

    #[test]
    fn enforced_player_failure_is_propagated() {
        let (address, port, server) = spawn_server(false, None);
        let settings = FiveMRequestSettings::default().set_gather_players(GatherToggle::Enforce);

        let result = query_with_timeout_and_extra_settings(&address, Some(port), &None, Some(settings));
        server.join().expect("mock server must succeed");

        assert!(result.is_err());
    }

    #[test]
    fn oversized_json_response_is_rejected() {
        let (address, port, server) = spawn_oversized_info_server();

        let result = query(&address, Some(port));
        server.join().expect("mock server must succeed");

        assert_eq!(
            result.expect_err("oversized response must fail"),
            crate::GDErrorKind::PacketOverflow.into()
        );
    }

    #[cfg(feature = "game_defs")]
    #[test]
    fn generic_definition_dispatches_to_fivem() {
        let (address, port, server) = spawn_server(true, None);
        let game = crate::games::GAMES
            .get("fivem")
            .expect("FiveM definition must exist");

        let response =
            crate::games::query::query(game, &address, Some(port)).expect("generic FiveM query must succeed");
        server.join().expect("mock server must succeed");

        assert_eq!(game.default_port, DEFAULT_PORT);
        assert_eq!(response.players_online(), 2);
        assert_eq!(response.players_maximum(), 32);
    }
}
