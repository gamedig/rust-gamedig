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
    "mc" => game!("Minecraft", 25565, Protocol::Minecraft(None)),
    "mc-java" => game!("Minecraft (java)", 25565, Protocol::Minecraft(Some(Server::Java))),
    "mc-bedrock" => game!("Minecraft (bedrock)", 19132, Protocol::Minecraft(Some(Server::Bedrock))),
    "mc-legacy-1.6" => game!("Minecraft (legacy v1.6)", 25565, Protocol::Minecraft(Some(Server::Legacy(LegacyGroup::V1_6)))),
    "mc-legacy-1.4" => game!("Minecraft (legacy v1.4-1.5)", 25565, Protocol::Minecraft(Some(Server::Legacy(LegacyGroup::V1_4)))),
    "mc-legacy-b1.8" => game!("Minecraft (legacy vB1.8-1.3)", 25565, Protocol::Minecraft(Some(Server::Legacy(LegacyGroup::VB1_8)))),
    "alienswarm" => game!("Alien Swarm", 27015, Protocol::Valve(SteamApp::ALIENS)),
    "ageofchivalry" => game!("Age of Chivalry", 27015, Protocol::Valve(SteamApp::AOC)),
    "arma2oa" => game!("ARMA 2: Operation Arrowhead", 2304, Protocol::Valve(SteamApp::ARMA2OA)),
    "arkse" => game!("ARK: Survival Evolved", 27015, Protocol::Valve(SteamApp::ASE)),
    "asrd" => game!("Alien Swarm: Reactive Drop", 2304, Protocol::Valve(SteamApp::ASRD)),
    "avorion" => game!("Avorion", 27020, Protocol::Valve(SteamApp::AVORION)),
    "bat1944" => game!("Battalion 1944", 7780, Protocol::Valve(SteamApp::BAT1944)),
    "brainbread2" => game!("BrainBread 2", 27015, Protocol::Valve(SteamApp::BB2)),
    "bf1942" => game!("Battlefield 1942", 23000, Protocol::Gamespy(GameSpyVersion::One)),
    "blackmesa" => game!("Black Mesa", 27015, Protocol::Valve(SteamApp::BM)),
    "ballisticoverkill" => game!("Ballistic Overkill", 27016, Protocol::Valve(SteamApp::BO)),
    "codenamecure" => game!("Codename CURE", 27015, Protocol::Valve(SteamApp::CCURE)),
    "colonysurvival" => game!("Colony Survival", 27004, Protocol::Valve(SteamApp::COSU)),
    "cs" => game!("Counter-Strike", 27015, Protocol::Valve(SteamApp::CS)),
    "cscz" => game!("Counter Strike: Condition Zero", 27015, Protocol::Valve(SteamApp::CSCZ)),
    "csgo" => game!("Counter-Strike: Global Offensive", 27015, Protocol::Valve(SteamApp::CSGO)),
    "css" => game!("Counter-Strike: Source", 27015, Protocol::Valve(SteamApp::CSS)),
    "creativerse" => game!("Creativerse", 26901, Protocol::Valve(SteamApp::CREATIVERSE)),
    "crysiswars" => game!("Crysis Wars", 64100, Protocol::Gamespy(GameSpyVersion::Three)),
    "dod" => game!("Day of Defeat", 27015, Protocol::Valve(SteamApp::DOD)),
    "dods" => game!("Day of Defeat: Source", 27015, Protocol::Valve(SteamApp::DODS)),
    "doi" => game!("Day of Infamy", 27015, Protocol::Valve(SteamApp::DOI)),
    "dst" => game!("Don't Starve Together", 27016, Protocol::Valve(SteamApp::DST)),
    "ffow" => game!("Frontlines: Fuel of War", 5478, Protocol::PROPRIETARY(ProprietaryProtocol::FFOW)),
    "garrysmod" => game!("Garry's Mod", 27016, Protocol::Valve(SteamApp::GM)),
    "hl2dm" => game!("Half-Life 2 Deathmatch", 27015, Protocol::Valve(SteamApp::HL2DM)),
    "haloce" => game!("Halo: Combat Evolved", 2302, Protocol::Gamespy(GameSpyVersion::Two)),
    "hldms" => game!("Half-Life Deathmatch: Source", 27015, Protocol::Valve(SteamApp::HLDMS)),
    "hll" => game!("Hell Let Loose", 26420, Protocol::Valve(SteamApp::HLL)),
    "insurgency" => game!("Insurgency", 27015, Protocol::Valve(SteamApp::INS)),
    "insurgencymic" => game!("Insurgency: Modern Infantry Combat", 27015, Protocol::Valve(SteamApp::INSMIC)),
    "insurgencysandstorm" => game!("Insurgency: Sandstorm", 27131, Protocol::Valve(SteamApp::INSS)),
    "left4dead" => game!("Left 4 Dead", 27015, Protocol::Valve(SteamApp::L4D)),
    "left4dead2" => game!("Left 4 Dead 2", 27015, Protocol::Valve(SteamApp::L4D2)),
    "ohd" => game!("Operation: Harsh Doorstop", 27005, Protocol::Valve(SteamApp::OHD)),
    "onset" => game!("Onset", 7776, Protocol::Valve(SteamApp::ONSET)),
    "przomboid" => game!("Project Zomboid", 16261, Protocol::Valve(SteamApp::PZ)),
    "quake1" => game!("Quake 1", 27500, Protocol::Quake(QuakeVersion::One)),
    "quake2" => game!("Quake 2", 27910, Protocol::Quake(QuakeVersion::Two)),
    "quake3" => game!("Quake 3: Arena", 27960, Protocol::Quake(QuakeVersion::Three)),
    "ror2" => game!("Risk of Rain 2", 27016, Protocol::Valve(SteamApp::ROR2)),
    "rust" => game!("Rust", 27015, Protocol::Valve(SteamApp::RUST)),
    "svencoop" => game!("Sven Co-op", 27015, Protocol::Valve(SteamApp::SC)),
    "7d2d" => game!("7 Days To Die", 26900, Protocol::Valve(SteamApp::SDTD)),
    "sof2" => game!("Soldier of Fortune 2", 20100, Protocol::Quake(QuakeVersion::Three)),
    "ss" => game!("Serious Sam", 25601, Protocol::Gamespy(GameSpyVersion::One)),
    "tf" => game!("The Forest", 27016, Protocol::Valve(SteamApp::TF)),
    "tf2" => game!("Team Fortress 2", 27015, Protocol::Valve(SteamApp::TF2)),
    "tfc" => game!("Team Fortress Classic", 27015, Protocol::Valve(SteamApp::TFC)),
    "ship" => game!("The Ship", 27015, Protocol::PROPRIETARY(ProprietaryProtocol::TheShip)),
    "unturned" => game!("Unturned", 27015, Protocol::Valve(SteamApp::UNTURNED)),
    "ut" => game!("Unreal Tournament", 7778, Protocol::Gamespy(GameSpyVersion::One)),
    "vrising" => game!("V Rising", 27016, Protocol::Valve(SteamApp::VR)),
    "jc2mp" => game!("Just Cause 2: Multiplayer", 7777, Protocol::PROPRIETARY(ProprietaryProtocol::JC2MP)),
    "warsow" => game!("Warsow", 44400, Protocol::Quake(QuakeVersion::Three)),
};
