#![cfg(test)]
#![cfg(feature = "integration_tests")]

use std::collections::HashSet;
use std::fs::OpenOptions;
use std::net::ToSocketAddrs;
use std::path::Path;

use gamedig::protocols::types::CommonResponseJson;
use gamedig::protocols::ExtraRequestSettings;
use gamedig::query_with_timeout_and_extra_settings;
use gamedig::GAMES;

use net_replay_test;

pub struct SelfImpl;
impl net_replay_test::implementations::QueryImplementation for SelfImpl {
    fn query_server(
        &self,
        options: &net_replay_test::QueryOptions,
    ) -> Result<net_replay_test::value::CommonValue, Box<dyn std::error::Error + 'static>> {
        let game = GAMES.get(&options.game).ok_or("Unknown game")?;

        let ip = format!("{}:0", options.address)
            .to_socket_addrs()?
            .next()
            .ok_or("Given hostname did not resolve to an IP")?
            .ip();

        let extra_settings = ExtraRequestSettings::default()
            .set_gather_rules(false)
            .set_gather_players(true)
            .set_hostname(options.address.clone());

        let output = query_with_timeout_and_extra_settings(game, &ip, options.port, None, Some(extra_settings))?;

        Ok(from_gamedig_to_net_replay(output.as_json()))
    }
}

fn from_gamedig_to_net_replay(value: CommonResponseJson<'_>) -> net_replay_test::value::CommonValue {
    net_replay_test::value::CommonValue {
        name: value.name.map(|v| v.to_string()),
        map: value.map.map(|v| v.to_string()),
        has_password: value.has_password,
        players_online: Some(value.players_online.into()),
        players_maximum: Some(value.players_maximum.into()),
        player_names: value
            .players
            .map(|players| {
                players
                    .into_iter()
                    .map(|player| player.name.to_string())
                    .collect()
            })
            .unwrap_or(HashSet::new()),
    }
}

fn get_impl() -> Box<dyn net_replay_test::implementations::QueryImplementation> { Box::new(SelfImpl) }

pub fn do_replay_test<P: AsRef<Path>>(replay_path: P) {
    let file = OpenOptions::new()
        .read(true)
        .open(replay_path)
        .expect("Replay file should exist");

    let query_replay = serde_json::from_reader(file).expect("Replay should be valid format");

    let result = net_replay_test::replay(get_impl(), query_replay).expect("Replay should succeed");

    assert!(result);

    println!("{:?}", result);
}

#[test]
fn integration_test_tf2() { do_replay_test("./tests/integration-data/tf2-success.json"); }
