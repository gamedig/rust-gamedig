//! Static definitions of currently supported games

use crate::games::minecraft::types::{LegacyGroup, Server};
use crate::protocols::{gamespy::GameSpyVersion, quake::QuakeVersion, valve::Engine, Protocol};
use crate::Game;

use crate::protocols::types::{GatherToggle, ProprietaryProtocol};
use crate::protocols::valve::GatheringSettings;
use phf::{phf_map, Map};

macro_rules! game {
    ($name: literal, $default_port: expr, $protocol: expr) => {
        game!(
            $name,
            $default_port,
            $protocol,
            GatheringSettings::default().into_extra()
        )
    };

    ($name: literal, $default_port: expr, $protocol: expr, $extra_request_settings: expr) => {
        Game {
            name: $name,
            default_port: $default_port,
            protocol: $protocol,
            request_settings: $extra_request_settings,
        }
    };
}

/// Map of all currently supported games
pub static GAMES: Map<&'static str, Game> = phf_map! {
    // Query with all minecraft protocols
    "minecraft" => game!("Minecraft", 25565, Protocol::PROPRIETARY(ProprietaryProtocol::Minecraft(None))),
    // Query with specific minecraft protocols
    "minecraftbedrock" => game!("Minecraft (bedrock)", 19132, Protocol::PROPRIETARY(ProprietaryProtocol::Minecraft(Some(Server::Bedrock)))),
    "minecraftpocket" => game!("Minecraft (pocket)", 19132, Protocol::PROPRIETARY(ProprietaryProtocol::Minecraft(Some(Server::Bedrock)))),
    "minecraftjava" => game!("Minecraft (java)", 25565, Protocol::PROPRIETARY(ProprietaryProtocol::Minecraft(Some(Server::Java)))),
    "minecraftlegacy16" => game!("Minecraft (legacy 1.6)", 25565, Protocol::PROPRIETARY(ProprietaryProtocol::Minecraft(Some(Server::Legacy(LegacyGroup::V1_6))))),
    "minecraftlegacy14" => game!("Minecraft (legacy 1.4)", 25565, Protocol::PROPRIETARY(ProprietaryProtocol::Minecraft(Some(Server::Legacy(LegacyGroup::V1_4))))),
    "minecraftlegacyb18" => game!("Minecraft (legacy b1.8)", 25565, Protocol::PROPRIETARY(ProprietaryProtocol::Minecraft(Some(Server::Legacy(LegacyGroup::VB1_8))))),
    "aapg" => game!("America's Army: Proving Grounds", 27020, Protocol::Valve(Engine::new(203_290)), GatheringSettings {
        players: GatherToggle::Enforce,
        rules: GatherToggle::Skip,
        check_app_id: true,
    }.into_extra()),
    "alienswarm" => game!("Alien Swarm", 27015, Protocol::Valve(Engine::new(630))),
    "aoc" => game!("Age of Chivalry", 27015, Protocol::Valve(Engine::new(17510))),
    "a2oa" => game!("ARMA 2: Operation Arrowhead", 2304, Protocol::Valve(Engine::new(33930))),
    "ase" => game!("ARK: Survival Evolved", 27015, Protocol::Valve(Engine::new(346_110))),
    "asrd" => game!("Alien Swarm: Reactive Drop", 2304, Protocol::Valve(Engine::new(563_560))),
    "atlas" => game!("ATLAS", 57561, Protocol::Valve(Engine::new(834_910))),
    "avorion" => game!("Avorion", 27020, Protocol::Valve(Engine::new(445_220))),
    "barotrauma" => game!("Barotrauma", 27016, Protocol::Valve(Engine::new(602_960))),
    "basedefense" => game!("Base Defense", 27015, Protocol::Valve(Engine::new(632_730)), GatheringSettings {
        players: GatherToggle::Enforce,
        rules: GatherToggle::Skip,
        check_app_id: true,
    }.into_extra()),
    "battalion1944" => game!("Battalion 1944", 7780, Protocol::Valve(Engine::new(489_940))),
    "brainbread2" => game!("BrainBread 2", 27015, Protocol::Valve(Engine::new(346_330))),
    "battlefield1942" => game!("Battlefield 1942", 23000, Protocol::Gamespy(GameSpyVersion::One)),
    "blackmesa" => game!("Black Mesa", 27015, Protocol::Valve(Engine::new(362_890))),
    "ballisticoverkill" => game!("Ballistic Overkill", 27016, Protocol::Valve(Engine::new(296_300))),
    "codbo3" => game!("Call Of Duty: Black Ops 3", 27017, Protocol::Valve(Engine::new(311_210))),
    "codenamecure" => game!("Codename CURE", 27015, Protocol::Valve(Engine::new(355_180))),
    "colonysurvival" => game!("Colony Survival", 27004, Protocol::Valve(Engine::new(366_090))),
    "conanexiles" => game!("Conan Exiles", 27015, Protocol::Valve(Engine::new(440_900)), GatheringSettings {
        players: GatherToggle::Skip,
        rules: GatherToggle::Enforce,
        check_app_id: true,
    }.into_extra()),
    "counterstrike" => game!("Counter-Strike", 27015, Protocol::Valve(Engine::new_gold_src(false))),
    "cscz" => game!("Counter Strike: Condition Zero", 27015, Protocol::Valve(Engine::new_gold_src(false))),
    "csgo" => game!("Counter-Strike: Global Offensive", 27015, Protocol::Valve(Engine::new(730))),
    "css" => game!("Counter-Strike: Source", 27015, Protocol::Valve(Engine::new(240))),
    "cmw" => game!("Chivalry: Medieval Warfare", 7779, Protocol::Valve(Engine::new(219_640))),
    "creativerse" => game!("Creativerse", 26901, Protocol::Valve(Engine::new(280_790))),
    "crysiswars" => game!("Crysis Wars", 64100, Protocol::Gamespy(GameSpyVersion::Three)),
    "dod" => game!("Day of Defeat", 27015, Protocol::Valve(Engine::new_gold_src(false))),
    "dods" => game!("Day of Defeat: Source", 27015, Protocol::Valve(Engine::new(300))),
    "doi" => game!("Day of Infamy", 27015, Protocol::Valve(Engine::new(447_820))),
    "dst" => game!("Don't Starve Together", 27016, Protocol::Valve(Engine::new(322_320))),
    "ffow" => game!("Frontlines: Fuel of War", 5478, Protocol::PROPRIETARY(ProprietaryProtocol::FFOW)),
    "garrysmod" => game!("Garry's Mod", 27016, Protocol::Valve(Engine::new(4000))),
    "hl2d" => game!("Half-Life 2 Deathmatch", 27015, Protocol::Valve(Engine::new(320))),
    "hce" => game!("Halo: Combat Evolved", 2302, Protocol::Gamespy(GameSpyVersion::Two)),
    "hlds" => game!("Half-Life Deathmatch: Source", 27015, Protocol::Valve(Engine::new(360))),
    "hll" => game!("Hell Let Loose", 26420, Protocol::Valve(Engine::new(686_810))),
    "insurgency" => game!("Insurgency", 27015, Protocol::Valve(Engine::new(222_880))),
    "imic" => game!("Insurgency: Modern Infantry Combat", 27015, Protocol::Valve(Engine::new(17700))),
    "insurgencysandstorm" => game!("Insurgency: Sandstorm", 27131, Protocol::Valve(Engine::new(581_320))),
    "l4d" => game!("Left 4 Dead", 27015, Protocol::Valve(Engine::new(500))),
    "l4d2" => game!("Left 4 Dead 2", 27015, Protocol::Valve(Engine::new(550))),
    "ohd" => game!("Operation: Harsh Doorstop", 27005, Protocol::Valve(Engine::new_with_dedicated(736_590, 950_900))),
    "onset" => game!("Onset", 7776, Protocol::Valve(Engine::new(1_105_810))),
    "postscriptum" => game!("Post Scriptum", 10037, Protocol::Valve(Engine::new(736_220))),
    "projectzomboid" => game!("Project Zomboid", 16261, Protocol::Valve(Engine::new(108_600))),
    "quake1" => game!("Quake 1", 27500, Protocol::Quake(QuakeVersion::One)),
    "quake2" => game!("Quake 2", 27910, Protocol::Quake(QuakeVersion::Two)),
    "q3a" => game!("Quake 3 Arena", 27960, Protocol::Quake(QuakeVersion::Three)),
    "risingworld" => game!("Rising World", 4254, Protocol::Valve(Engine::new(324_080)), GatheringSettings {
        players: GatherToggle::Enforce,
        rules: GatherToggle::Skip,
        check_app_id: true,
    }.into_extra()),
    "ror2" => game!("Risk of Rain 2", 27016, Protocol::Valve(Engine::new(632_360))),
    "rust" => game!("Rust", 27015, Protocol::Valve(Engine::new(252_490))),
    "savage2" => game!("Savage 2", 11235, Protocol::PROPRIETARY(ProprietaryProtocol::Savage2)),
    "sco" => game!("Sven Co-op", 27015, Protocol::Valve(Engine::new_gold_src(false))),
    "sdtd" => game!("7 Days to Die", 26900, Protocol::Valve(Engine::new(251_570))),
    "sof2" => game!("Soldier of Fortune 2", 20100, Protocol::Quake(QuakeVersion::Three)),
    "serioussam" => game!("Serious Sam", 25601, Protocol::Gamespy(GameSpyVersion::One)),
    "squad" => game!("Squad", 27165, Protocol::Valve(Engine::new(393_380))),
    "theforest" => game!("The Forest", 27016, Protocol::Valve(Engine::new(556_450))),
    "thefront" => game!("The Front", 27015, Protocol::Valve(Engine::new(2_285_150))),
    "teamfortress2" => game!("Team Fortress 2", 27015, Protocol::Valve(Engine::new(440))),
    "tfc" => game!("Team Fortress Classic", 27015, Protocol::Valve(Engine::new_gold_src(false))),
    "theship" => game!("The Ship", 27015, Protocol::PROPRIETARY(ProprietaryProtocol::TheShip)),
    "unturned" => game!("Unturned", 27015, Protocol::Valve(Engine::new(304_930))),
    "unrealtournament" => game!("Unreal Tournament", 7778, Protocol::Gamespy(GameSpyVersion::One)),
    "valheim" => game!("Valheim", 2457, Protocol::Valve(Engine::new(892_970)), GatheringSettings {
        players: GatherToggle::Enforce,
        rules: GatherToggle::Skip,
        check_app_id: true,
    }.into_extra()),
    "vrising" => game!("V Rising", 27016, Protocol::Valve(Engine::new(1_604_030))),
    "jc2m" => game!("Just Cause 2: Multiplayer", 7777, Protocol::PROPRIETARY(ProprietaryProtocol::JC2M)),
    "warsow" => game!("Warsow", 44400, Protocol::Quake(QuakeVersion::Three)),
    "dhe4445" => game!("Darkest Hour: Europe '44-'45 (2008)", 7758, Protocol::Unreal2),
    "devastation" => game!("Devastation (2003)", 7778, Protocol::Unreal2),
    "killingfloor" => game!("Killing Floor", 7708, Protocol::Unreal2),
    "redorchestra" => game!("Red Orchestra", 7759, Protocol::Unreal2),
    "unrealtournament2003" => game!("Unreal Tournament 2003", 7758, Protocol::Unreal2),
    "unrealtournament2004" => game!("Unreal Tournament 2004", 7778, Protocol::Unreal2),
    "eco" => game!("Eco", 3000, Protocol::PROPRIETARY(ProprietaryProtocol::Eco)),
    "zps" => game!("Zombie Panic: Source", 27015, Protocol::Valve(Engine::new(17_500))),
    "mindustry" => game!("Mindustry", crate::games::mindustry::DEFAULT_PORT, Protocol::PROPRIETARY(ProprietaryProtocol::Mindustry)),
};
