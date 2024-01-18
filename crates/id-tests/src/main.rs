#![cfg(feature = "cli")]

use std::collections::HashMap;

/// Format for input games (the same as used in node-gamedig/lib/games.js).
type GamesInput = HashMap<String, Game>;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
struct Game {
    name: String,
}

use gamedig_id_tests::test_game_name_rules;

fn main() {
    let games: GamesInput = if let Some(file) = std::env::args_os().skip(1).next() {
        let file = std::fs::OpenOptions::new().read(true).open(file).unwrap();

        serde_json::from_reader(file).unwrap()
    } else {
        serde_json::from_reader(std::io::stdin().lock()).unwrap()
    };

    let failed = test_game_name_rules(
        games
            .iter()
            .map(|(key, game)| (key.as_str(), game.name.as_str())),
    );

    assert!(failed.is_empty());
}
