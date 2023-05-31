use std::fs::{read_dir, OpenOptions};
use std::io::{Result, Write};

// https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("src/games/mod.rs")?;

    writeln!(output_file, "use std::collections::HashMap;")?;
    writeln!(output_file, "use crate::GameInfo;")?;

    writeln!(output_file, "// Import games")?;

    let game_names: Vec<String> = read_dir("src/games")
        ?
        // TODO: Check entry is file
        .filter_map(|entry| entry.ok().and_then(|entry| entry.file_name().into_string().ok()).filter(|name| *name != "mod.rs" && (name == "aliens.rs" || name == "tf2.rs")).and_then(|name| name.strip_suffix(".rs").map(|n| n.to_string()))).collect();

    for name in &game_names {
        writeln!(output_file, "pub mod {};", name)?;
    }

    writeln!(output_file, "\n// Generic interface")?;
    writeln!(
        output_file,
        "pub fn games() -> HashMap<String, &'static dyn GameInfo> {{
        let mut game_map = HashMap::new();"
    )?;

    for name in game_names {
        writeln!(
            output_file,
            "{{ let info: &'static dyn GameInfo = {name}::INFO; game_map.insert(String::from(\"{name}\"), info); }}"
        )?;
    }

    writeln!(
        output_file,
        "game_map
}}"
    )?;

    Ok(())
}
