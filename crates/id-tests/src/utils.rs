/// Split a str when characters swap between being digits and not digits.
pub fn split_on_switch_between_alpha_numeric(text: &str) -> Vec<String> {
    if text.is_empty() {
        return vec![];
    }

    let mut parts = Vec::with_capacity(text.len());
    let mut current = Vec::with_capacity(text.len());

    let mut iter = text.chars();
    let c = iter.next().unwrap();
    let mut last_was_numeric = c.is_ascii_digit();
    current.push(c);

    for c in iter {
        if c.is_ascii_digit() == last_was_numeric {
            current.push(c);
        } else {
            parts.push(current.iter().collect());
            current.clear();
            current.push(c);
            last_was_numeric = !last_was_numeric;
        }
    }

    parts.push(current.into_iter().collect());

    parts
}

#[test]
fn split_correctly() {
    assert_eq!(
        split_on_switch_between_alpha_numeric("2D45A"),
        &["2", "D", "45", "A"]
    );
}

#[test]
fn split_symbol_broken_numbers() {
    let game_name = super::extract_game_parts_from_name("Darkest Hour: Europe '44-'45");
    assert_eq!(game_name.words, &["Darkest", "Hour", "Europe", "4445"]);
}

/// Extract parts at end of string enclosed in brackets.
pub fn extract_bracketed_suffix(text: &str) -> (&str, Option<&str>) {
    if let Some(text) = text.strip_suffix(')') {
        if let Some((text, extra)) = text.rsplit_once('(') {
            return (text, Some(extra));
        }
    }

    (text, None)
}

#[test]
fn extract_brackets_correctly() {
    assert_eq!(
        extract_bracketed_suffix("no brackets here"),
        ("no brackets here", None)
    );
    assert_eq!(
        extract_bracketed_suffix("Game name (with protocol here)"),
        ("Game name ", Some("with protocol here"))
    );
}
