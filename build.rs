use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

use genco::fmt;
use genco::prelude::*;

use convert_case::{Case, Casing};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, serde::Deserialize)]
struct GameConfig {
    name: String,
    protocol: String,
}

#[derive(Debug)]
struct GameWithNames {
    config: GameConfig,
    struct_name: String,
    static_name: String,
}

fn generate_games_module() -> Result<()> {
    println!("cargo:rerun-if-changed=games.toml");

    let games: HashMap<String, GameConfig> = {
        let mut config_file = OpenOptions::new().read(true).open("games.toml")?;
        let mut config_string = String::new();
        config_file.read_to_string(&mut config_string)?;
        toml::from_str(&config_string)?
    };

    let games: HashMap<String, GameWithNames> = games
        .into_iter()
        .map(|(name, config)| {
            (
                name.clone(),
                GameWithNames {
                    config,
                    struct_name: format!("{}Info", name.to_case(Case::Pascal)).to_string(),
                    static_name: format!("INFO_{}", name.to_case(Case::UpperSnake)).to_string(),
                },
            )
        })
        .collect();

    eprintln!("GameConfig={:?}", games);

    let output_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_path.join("games_mod.rs"))?;

    let hash_map = rust::import("std::collections", "HashMap");
    let ip_addr = rust::import("std::net", "IpAddr");

    let game_info = rust::import("crate", "GameInfo");
    let gd_result = rust::import("crate", "GDResult");
    let generic_response = rust::import("crate", "GenericResponse");

    let import_tokens: Tokens<Rust> = quote! {
        $['\n']
        $(format!("// Game imports\n"))
        $(for name in games.keys() join($['\r']) => $(format!("#[path=\"{}/src/games/{}.rs\"]", env!("CARGO_MANIFEST_DIR"), name)) pub mod $name;)
    };

    let types_tokens: Tokens<Rust> = quote! {
        $(format!("// Game types\n"))
        $(for (name, info) in &games join ($['\n']) =>
            struct $(info.struct_name.clone());$['\r']
            impl $(game_info.clone()) for $(info.struct_name.clone()) {$['\r']
                fn name(&self) -> &'static str { $(quoted(info.config.name.clone())) }$['\r']
                fn protocol(&self) -> &'static str { $(quoted(info.config.protocol.clone())) }$['\r']
                fn query(&self, address: &$(ip_addr.clone()), port: Option<u16>) -> $(gd_result.clone())<Box<dyn $(generic_response.clone())>> {
                    Ok($name::query(address, port).map(Box::new)?)
                }$['\r']
            }$['\r']
            static $(info.static_name.clone()): &$(info.struct_name.clone()) = &$(info.struct_name.clone());$['\r']
        )
    };

    let accessor_tokens = quote! {
        $(format!("// Game array\n"))
        pub fn games() -> $(hash_map.clone())<String, &'static dyn $(game_info.clone())> {$['\r']
            $[' ']let mut r = $(hash_map.clone())::new();$['\r']

            $[' ']$(for (name, info) in &games join($['\r']) => {$['\r']
                $[' ']let info: &'static dyn $(game_info.clone()) = $(info.static_name.clone());$['\r']
                $[' ']r.insert(String::from($(quoted(name))), info);$['\r']
            })

            $[' ']return r;$['\r']
        }
    };

    let fmt = fmt::Config::from_lang::<Rust>().with_indentation(fmt::Indentation::Space(4));
    let config = rust::Config::default().with_default_import(rust::ImportMode::Qualified);

    let mut writer = fmt::IoWriter::new(output_file);
    import_tokens.format_file(&mut writer.as_formatter(&fmt), &config)?;
    types_tokens.format_file(&mut writer.as_formatter(&fmt), &config)?;
    accessor_tokens.format_file(&mut writer.as_formatter(&fmt), &config)?;

    Ok(())
}

// https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    generate_games_module()?;

    Ok(())
}
