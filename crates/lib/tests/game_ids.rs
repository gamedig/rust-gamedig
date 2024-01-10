#![cfg(all(test, feature = "game_defs"))]

use std::{fs, io::Read};

use gamedig::GAMES;

use gamedig_id_tests::test_game_name_rules;

#[test]
fn check_definitions_match_name_rules() {
    let wrong = test_game_name_rules(GAMES.entries().map(|(id, game)| (id.to_owned(), game.name)));
    assert!(wrong.is_empty());
}

#[test]
#[ignore = "Don't test node by default"]
fn check_node_definitions_match_name_rules() {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open("./node-gamedig/games.txt")
        .unwrap();

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let games = text
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<_> = line.splitn(3, '|').collect();
            if parts.len() > 1 {
                Some((parts[0].split(',').next().unwrap(), parts[1]))
            } else {
                None
            }
        });

    let wrong = test_game_name_rules(games);
    assert!(wrong.is_empty());
}
