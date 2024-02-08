use std::collections::HashMap;

mod utils;
use utils::{extract_bracketed_suffix, split_on_switch_between_alpha_numeric};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDRule {
    IDsMustBeLowerCase,
    NumbersAreTheirOwnWord,
    IfFirstWordNumberNoDigits,
    IfLastWordNumberMustBeAppended,
    ConvertRomanNumeralsToArabic,
    TwoWordsOrLessUseFullWords,
    MoreThanTwoWordsMakeAcronym,
    IfIDDuplicateSameGameAppendYearToNewer,
    IfIDDuplicateSameGameAppendProtocol,
    IfIDDuplicateNoAcronym,
    IfModForQueriesProcessOnlyModName,
    NoDuplicates,
}

#[derive(Clone, Debug)]
pub struct IDFail {
    pub game_id: String,
    pub game_name: String,
    pub expected_id: String,
    pub rule_stack: Vec<IDRule>,
}

impl IDFail {
    fn new(game_id: String, game_name: String, expected_id: String, rule_stack: Vec<IDRule>) -> Self {
        Self {
            game_id,
            game_name,
            expected_id,
            rule_stack,
        }
    }
}

/// Test a single game against the rules
pub fn test_game_name_rule(
    seen_ids: &mut HashMap<String, Vec<String>>,
    id: &str,
    mut game: GameNameParsed,
    is_mod_name: bool,
) -> Vec<IDFail> {
    let mut wrong_ids = Vec::new();

    let mut rule_stack = Vec::new();
    if is_mod_name {
        rule_stack.push(IDRule::IfModForQueriesProcessOnlyModName);
    }

    let mut suffix = String::new();

    // A game's identification is a lowercase alphanumeric string will and be forged
    // following these rules:
    if id.to_lowercase().ne(id) {
        wrong_ids.push(IDFail::new(
            id.to_owned(),
            game.name.to_owned(),
            id.to_lowercase(),
            vec![IDRule::IDsMustBeLowerCase],
        ));
    }

    // 5. Roman numbering will be converted to arabic numbering (XIV -> 14).
    game.words = {
        let mut is_first = true;
        game.words
            .into_iter()
            .map(|w| {
                // First word will never be a numeral
                if is_first {
                    is_first = false;
                    w
                } else if let Ok(number) = roman_numeral::RomanNumeral::from_string(&w) {
                    rule_stack.push(IDRule::ConvertRomanNumeralsToArabic);
                    number.get_u32().to_string()
                } else {
                    w
                }
            })
            .collect()
    };

    // 6. Unless numbers are at the end of a name, they will be considered words,
    //    but digits will always be used instead of the acronym (counter to #2)
    //    (Left 4 Dead -> l4d) unless they at the start position (7 Days to Die ->
    //    sdtd), if they are at the end (such as sequel number or the year), always
    //    append them (Team Fortress 2 -> teamfortress2, Unreal Tournament 2003 ->
    //    unrealtournament2003).
    game.words = game
        .words
        .into_iter()
        .flat_map(|w| {
            let n = split_on_switch_between_alpha_numeric(&w);
            if n.len() > 1 {
                rule_stack.push(IDRule::NumbersAreTheirOwnWord);
            }
            n
        })
        .collect();

    // If first word is number make text
    if !game.words.is_empty() && game.words[0].chars().next().unwrap().is_ascii_digit() {
        game.words[0] = number_to_words::number_to_words(game.words[0].parse::<f64>().unwrap(), false);
        rule_stack.push(IDRule::IfFirstWordNumberNoDigits);
    }

    // If last word is number append full number
    if let Some(last_word) = game.words.last() {
        if last_word.chars().all(|c| c.is_ascii_digit()) {
            suffix += &game.words.pop().unwrap();
            rule_stack.push(IDRule::IfLastWordNumberMustBeAppended);
        }
    }

    let main = if game.words.len() <= 2 {
        // 1. Names composed of a maximum of two words (unless #4 applies) will result
        //    in an id where the words are concatenated (Dead Cells -> deadcells),
        //    acronyms in the name count as a single word (S.T.A.L.K.E.R. -> stalker).

        rule_stack.push(IDRule::TwoWordsOrLessUseFullWords);

        game.words
            .iter()
            .map(|w| w.trim_matches('-').to_owned())
            .collect::<Vec<_>>()
            .join("")
    } else {
        // 2. Names of more than two words shall be made into an acronym made of the
        //    initial letters (The Binding of Isaac -> tboi), hypenation composed words
        //    don't count as a single word, but of how many parts they are made of (Dino
        //    D-Day, 3 words, so ddd).

        rule_stack.push(IDRule::MoreThanTwoWordsMakeAcronym);

        game.words
            .iter()
            .map(|w| w.chars().next().unwrap())
            .filter(|c| c.is_alphanumeric())
            .collect()
    };

    let mut expected_id = format!("{}{}", main, suffix).to_lowercase();

    if let Some(other_game_name_words) = seen_ids.get(&expected_id) {
        let mut game_names_same = other_game_name_words.len() == game.words.len();
        // Check all words in game name are the same
        if game_names_same {
            for (our_word, their_word) in game.words.iter().zip(other_game_name_words.iter()) {
                if our_word.to_lowercase() != their_word.to_lowercase() {
                    game_names_same = false;
                    break;
                }
            }
        }

        if game_names_same {
            if let Some(year) = game.year {
                // 3. If a game has the exact name as a previously existing id's game (Star Wars
                //    Battlefront 2, the 2005 and 2017 one), append the release year to the
                //    newer id (2005 would be swbf2 (suppose we already have this one supported)
                //    and 2017 would be swbf22017).

                rule_stack.push(IDRule::IfIDDuplicateSameGameAppendYearToNewer);
                expected_id = format!("{}{}", expected_id, year).to_lowercase();
            } else if let Some(protocol) = game.optional_parts.first() {
                // 7. If a game supports multiple protocols, multiple entries will be done for
                //    said game where the edition/protocol name (first disposable in this order)
                //    will be appended to the game name (Minecraft is divided by 2 editions,
                //    Java and Bedrock which will be minecraftjava and minecraftbedrock
                //    respectively) and one more entry can be added by the base name of the game
                //    which queries in a group said supported protocols to make generic queries
                //    easier and disposable.

                rule_stack.push(IDRule::IfIDDuplicateSameGameAppendProtocol);

                // Parse the protocol as a game name so we can remove all non-valid characters
                let protocol_parsed = extract_game_parts_from_name(protocol);

                expected_id = format!("{}{}", expected_id, protocol_parsed.words.concat(),);
            }
        }
    }

    // 4. If a new id (Day of Dragons -> dod) results in an id that already exists
    //    (Day of Defeat -> dod), then the new name should ignore rule #2 (Day of
    //    Dragons -> dayofdragons).
    if seen_ids.contains_key(&expected_id) {
        rule_stack.push(IDRule::IfIDDuplicateNoAcronym);

        let main = game
            .words
            .iter()
            .map(|w| w.trim_matches('-').to_owned())
            .collect::<Vec<_>>()
            .join("");

        expected_id = format!("{}{}", main, suffix).to_lowercase();
    }

    // 8. If its actually about a mod that adds the ability for queries to be
    //    performed, process only the mod name.
    if !is_mod_name && id != expected_id {
        if let Some((_, mod_game)) = game.name.split_once('-') {
            let mut result = test_game_name_rule(seen_ids, id, extract_game_parts_from_name(mod_game), true);

            if result.is_empty() {
                return result;
            } else {
                wrong_ids.append(&mut result);
            }
        }
    }

    let duplicate = if seen_ids.insert(expected_id.clone(), game.words).is_some() {
        rule_stack.push(IDRule::NoDuplicates);
        true
    } else {
        false
    };

    // Check ID matches
    if id != expected_id || duplicate {
        wrong_ids.push(IDFail::new(
            id.to_owned(),
            game.name.to_owned(),
            expected_id,
            rule_stack,
        ));
    }

    wrong_ids
}

#[derive(Clone, Debug)]
pub struct GameNameParsed<'a> {
    name: &'a str,
    words: Vec<String>,
    optional_parts: Vec<&'a str>,
    year: Option<u16>,
}

pub fn extract_game_parts_from_name(game: &str) -> GameNameParsed {
    // Separate game name into words
    // NOTE: we have to leave "-" in to prevent hyphenated prefixes being parsed as
    // numerals
    let mut optional_game_name_parts = Vec::new();

    let (game, paren) = extract_bracketed_suffix(game);

    if let Some(paren) = paren {
        optional_game_name_parts.push(paren);
    }

    let mut number_accumulator: Option<String> = None;

    // Filter map necessary to move out words
    #[allow(clippy::unnecessary_filter_map)]
    let game_name_words: Vec<_> = game
        // First split all text on space or dash
        .split_inclusive(&[' ', '-'])
        // Remove whitespace surrounding words (leave in dash because it is important information)
        .map(|w| w.trim())
        // If a word is entirely surrounded in brackets move it to optional parts
        .filter_map(|w| {
            if w.starts_with('(') && w.ends_with(')') {
                optional_game_name_parts.push(w);
                None
            } else {
                Some(w)
            }
        })
        // Remove all characters that aren't alphanumeric or dashses
        .map(|w| {
            w.replace(
                |c: char| !c.is_ascii_digit() && !c.is_alphabetic() && c != '-',
                "",
            )
        })
        // Remove words that are empty (discounting strings that are just dashes)
        .filter(|w| !w.trim_matches('-').is_empty())
        // Combine numbers that are seperated by dashes
        // e.g. 44-45 = 4445
        // Panics if there is text after number with trailing dash (44-text)
        .filter_map(|w| {
            if number_accumulator.is_some() {
                if let Some(maybe_number) = w.strip_suffix('-') {
                    if maybe_number.chars().all(|c| c.is_ascii_digit()) {
                        number_accumulator.as_mut().unwrap().push_str(maybe_number);
                        return None;
                    } else {
                        panic!("Text after number-");
                    }
                } else if w.chars().all(|c| c.is_ascii_digit()) {
                    let mut accumulator = number_accumulator.as_ref().unwrap().clone();
                    number_accumulator = None;
                    accumulator.push_str(&w);
                    return Some(accumulator);
                } else {
                    panic!("Text after number-");
                }
            } else if let Some(maybe_number) = w.strip_suffix('-') {
                if maybe_number.chars().all(|c| c.is_ascii_digit()) {
                    number_accumulator = Some(maybe_number.to_string());
                    return None;
                }
            }

            Some(w)
        })
        .collect();

    let mut game_year: Option<u16> = None;
    for optional_part in &optional_game_name_parts {
        if let Some(game_year_text) = optional_part
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
        {
            if let Ok(year) = game_year_text.parse() {
                game_year = Some(year);
                break;
            }
        } else if let Ok(year) = optional_part.parse() {
            game_year = Some(year);
            break;
        }
    }

    GameNameParsed {
        name: game,
        words: game_name_words,
        optional_parts: optional_game_name_parts,
        year: game_year,
    }
}

/// Iterate game entries and validate the id matches current rules
pub fn test_game_name_rules<'a, I: Iterator<Item = (&'a str, &'a str)>>(games: I) -> Vec<IDFail> {
    let mut wrong_ids = Vec::with_capacity(games.size_hint().0);

    let mut seen_ids: HashMap<String, Vec<String>> = HashMap::new();

    // We must sort games by year so that rule 3 is applied correctly
    let mut sorted_games: Vec<_> = games
        .map(|(id, game)| {
            let game = extract_game_parts_from_name(game);

            (id, game)
        })
        .collect();

    sorted_games.sort_by(|(_, a_game), (_, b_game)| {
        a_game
            .year
            .cmp(&b_game.year)
            .then(a_game.name.len().cmp(&b_game.name.len()))
    });

    let game_count = sorted_games.len();

    for (id, game) in sorted_games {
        wrong_ids.append(&mut test_game_name_rule(&mut seen_ids, id, game, false))
    }

    if !wrong_ids.is_empty() {
        for fail in &wrong_ids {
            println!("{:#?}", fail);
        }
        let percentage = (wrong_ids.len() * 100) / game_count;
        println!(
            "{} ({}%) IDs didn't match naming rules",
            wrong_ids.len(),
            percentage
        );
    }

    wrong_ids
}

pub fn test_single_game_rule(id: &str, name: &str) -> Vec<IDFail> { test_game_name_rules(std::iter::once((id, name))) }

#[cfg(test)]
mod id_tests {
    use super::{test_game_name_rules, test_single_game_rule};
    #[test]
    fn id_rule_one() {
        assert!(test_single_game_rule("testgame", "Test Game").is_empty());
        assert!(test_single_game_rule("testgame", "TestGame").is_empty());

        assert!(test_single_game_rule("deadcells", "Dead Cells").is_empty());
        assert!(test_single_game_rule("stalker", "S.T.A.L.K.E.R").is_empty());
    }

    #[test]
    fn id_rule_two() {
        assert!(test_single_game_rule("tgt", "Test Game Three").is_empty());
        assert!(test_single_game_rule("tgt", "Test Game-Three").is_empty());

        assert!(test_single_game_rule("tboi", "The Binding of Isaac").is_empty());
        assert!(test_single_game_rule("ddd", "Dino D-Day").is_empty());
    }

    #[test]
    fn id_rule_three() {
        let games = vec![
            ("swb22017", "Star Wars Battlefront 2 (2017)"),
            ("swb2", "Star Wars Battlefront 2 (2015)"),
        ];
        assert!(test_game_name_rules(games.into_iter()).is_empty());
    }

    #[test]
    fn id_rule_four() {
        let games = vec![("dod", "Day of Defeat"), ("dayofdragons", "Day of Dragons")];
        assert!(test_game_name_rules(games.into_iter()).is_empty());
    }

    #[test]
    fn id_rule_five() {
        assert!(test_single_game_rule("gta14", "Grand Theft Auto XIV").is_empty());
    }

    #[test]
    fn id_rule_six() {
        assert!(test_single_game_rule("l4d", "Left 4 Dead").is_empty());
        assert!(test_single_game_rule("sdtd", "7 Days to Die").is_empty());
        assert!(test_single_game_rule("teamfortress2", "Team Fortress 2").is_empty());
        assert!(test_single_game_rule("unrealtournament2003", "Unreal Tournament 2003").is_empty());
        assert!(test_single_game_rule("dhe4445", "Darkest Hour: Europe '44-'45").is_empty());
    }

    #[test]
    fn id_rule_seven() {
        let games = vec![
            ("minecraft", "Minecraft"),
            ("minecraftjava", "Minecraft (java)"),
            ("minecraftbedrock", "Minecraft (bedrock)"),
        ];
        assert!(test_game_name_rules(games.into_iter()).is_empty());
    }

    #[test]
    fn id_rule_eight() {
        assert!(test_single_game_rule("fivem", "Grand Theft Auto V - FiveM (2013)").is_empty());
        assert!(test_single_game_rule("jc3m", "Just Cause 3 - Multiplayer").is_empty());
    }
}
