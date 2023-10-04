//! Static definitions of currently supported games

use crate::protocols::{
    gamespy::GameSpyVersion,
    minecraft::{LegacyGroup, Server},
    quake::QuakeVersion,
    valve::SteamApp,
    Protocol,
};
use crate::Game;

use crate::protocols::types::ProprietaryProtocol;
use phf::{phf_map, Map};

macro_rules! game {
    ($name: literal, $default_port: literal, $protocol: expr) => {
        Game {
            name: $name,
            default_port: $default_port,
            protocol: $protocol,
        }
    };
}

/// Map of all currently supported games
pub static GAMES: Map<&'static str, Game> = phf_map! {
    // Query with all minecraft protocols node-gamedig: minecraft,minecraftping
    "minecraft" => game!("Minecraft", 25565, Protocol::Minecraft(None)),
    "minecraftping" => game!("Minecraft", 25565, Protocol::Minecraft(None)),
    // Query with specific minecraft protocols
    "minecraftbe" => game!("Minecraft (bedrock)", 19132, Protocol::Minecraft(Some(Server::Bedrock))),
    "minecraftpe" => game!("Minecraft (bedrock/pocket edition)", 19132, Protocol::Minecraft(Some(Server::Bedrock))),
    "minecraftjava" => game!("Minecraft (java)", 25565, Protocol::Minecraft(Some(Server::Java))),
    "minecraft-legacy-1.6" => game!("Minecraft (legacy v1.6)", 25565, Protocol::Minecraft(Some(Server::Legacy(LegacyGroup::V1_6)))),
    "minecraft-legacy-1.4" => game!("Minecraft (legacy v1.4-1.5)", 25565, Protocol::Minecraft(Some(Server::Legacy(LegacyGroup::V1_4)))),
    "minecraft-legacy-b1.8" => game!("Minecraft (legacy vB1.8-1.3)", 25565, Protocol::Minecraft(Some(Server::Legacy(LegacyGroup::VB1_8)))),
    "alienswarm" => game!("Alien Swarm", 27015, Protocol::Valve(SteamApp::ALIENSWARM)),
    "ageofchivalry" => game!("Age of Chivalry", 27015, Protocol::Valve(SteamApp::AGEOFCHIVALRY)),
    "arma2oa" => game!("ARMA 2: Operation Arrowhead", 2304, Protocol::Valve(SteamApp::ARMA2OA)),
    "arkse" => game!("ARK: Survival Evolved", 27015, Protocol::Valve(SteamApp::ARKSE)),
    "asrd" => game!("Alien Swarm: Reactive Drop", 2304, Protocol::Valve(SteamApp::ASRD)),
    "avorion" => game!("Avorion", 27020, Protocol::Valve(SteamApp::AVORION)),
    "bat1944" => game!("Battalion 1944", 7780, Protocol::Valve(SteamApp::BAT1944)),
    "brainbread2" => game!("BrainBread 2", 27015, Protocol::Valve(SteamApp::BRAINBREAD2)),
    "bf1942" => game!("Battlefield 1942", 23000, Protocol::Gamespy(GameSpyVersion::One)),
    "blackmesa" => game!("Black Mesa", 27015, Protocol::Valve(SteamApp::BLACKMESA)),
    "ballisticoverkill" => game!("Ballistic Overkill", 27016, Protocol::Valve(SteamApp::BALLISTICOVERKILL)),
    "codenamecure" => game!("Codename CURE", 27015, Protocol::Valve(SteamApp::CODENAMECURE)),
    "colonysurvival" => game!("Colony Survival", 27004, Protocol::Valve(SteamApp::COLONYSURVIVAL)),
    "cs" => game!("Counter-Strike", 27015, Protocol::Valve(SteamApp::CS)),
    "cscz" => game!("Counter Strike: Condition Zero", 27015, Protocol::Valve(SteamApp::CSCZ)),
    "csgo" => game!("Counter-Strike: Global Offensive", 27015, Protocol::Valve(SteamApp::CSGO)),
};

#[cfg(test)]
mod test {
    use super::GAMES;
    use std::fs;

    #[test]
    fn check_game_files_match_defs() {
        let ignore = [
            "mod",         // Module file
            "definitions", // This file
            "mc",          // Has various defs
            "sd2d",        // Module names cannot start with numbers
        ];

        for file in fs::read_dir("./src/games/").unwrap() {
            let file = file.unwrap();
            let metadata = file.metadata().unwrap();
            if metadata.is_file() {
                if let Some(file_name) = file.file_name().into_string().unwrap().strip_suffix(".rs") {
                    if !ignore.contains(&file_name) && !GAMES.contains_key(file_name) {
                        panic!("Expected GAMES to contain a definition to match {file_name}");
                    }
                }
            }
        }
    }
}
