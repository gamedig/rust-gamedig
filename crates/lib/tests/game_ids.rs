#![cfg(all(test, feature = "game_defs"))]

use gamedig::GAMES;

use gamedig_id_tests::test_game_name_rules;

#[test]
fn check_definitions_match_name_rules() {
    let wrong = test_game_name_rules(GAMES.entries().map(|(id, game)| (id.to_owned(), game.name)));
    assert!(wrong.is_empty());
}
