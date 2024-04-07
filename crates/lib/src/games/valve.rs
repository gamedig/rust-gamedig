//! Valve game query modules

use crate::protocols::valve::game_query_mod;

game_query_mod!(
    a2oa,
    "ARMA 2: Operation Arrowhead",
    Engine::new(33930),
    2304
);
game_query_mod!(basedefense, "Base Defense", Engine::new(632_730), 27015);
game_query_mod!(alienswarm, "Alien Swarm", Engine::new(630), 27015);
game_query_mod!(aoc, "Age of Chivalry", Engine::new(17510), 27015);
game_query_mod!(
    aapg,
    "America's Army: Proving Grounds",
    Engine::new(203_290),
    27020,
    GatheringSettings {
        players: GatherToggle::Enforce,
        rules: GatherToggle::Skip,
        check_app_id: true,
    }
);
game_query_mod!(ase, "ARK: Survival Evolved", Engine::new(346_110), 27015);
game_query_mod!(
    asrd,
    "Alien Swarm: Reactive Drop",
    Engine::new(563_560),
    2304
);
game_query_mod!(atlas, "ATLAS", Engine::new(834_910), 57561);
game_query_mod!(avorion, "Avorion", Engine::new(445_220), 27020);
game_query_mod!(
    ballisticoverkill,
    "Ballistic Overkill",
    Engine::new(296_300),
    27016
);
game_query_mod!(barotrauma, "Barotrauma", Engine::new(602_960), 27016);
game_query_mod!(blackmesa, "Black Mesa", Engine::new(362_890), 27015);
game_query_mod!(brainbread2, "BrainBread 2", Engine::new(346_330), 27015);
game_query_mod!(
    codbo3,
    "Call Of Duty: Black Ops 3",
    Engine::new(311_210),
    27017
);
game_query_mod!(codenamecure, "Codename CURE", Engine::new(355_180), 27015);
game_query_mod!(
    colonysurvival,
    "Colony Survival",
    Engine::new(366_090),
    27004
);
game_query_mod!(
    conanexiles,
    "Conan Exiles",
    Engine::new(440_900),
    27015,
    GatheringSettings {
        players: GatherToggle::Skip,
        rules: GatherToggle::Enforce,
        check_app_id: true,
    }
);
game_query_mod!(
    counterstrike,
    "Counter-Strike",
    Engine::new_gold_src(false),
    27015
);
game_query_mod!(counterstrike2, "Counter-Strike 2", Engine::new(730), 27015);
game_query_mod!(creativerse, "Creativerse", Engine::new(280_790), 26901);
game_query_mod!(
    cscz,
    "Counter Strike: Condition Zero",
    Engine::new_gold_src(false),
    27015
);
game_query_mod!(
    csgo,
    "Counter-Strike: Global Offensive",
    Engine::new(730),
    27015
);
game_query_mod!(css, "Counter-Strike: Source", Engine::new(240), 27015);
game_query_mod!(dab, "Double Action: Boogaloo", Engine::new(317_360), 27015);
game_query_mod!(dod, "Day of Defeat", Engine::new_gold_src(false), 27015);
game_query_mod!(dods, "Day of Defeat: Source", Engine::new(300), 27015);
game_query_mod!(doi, "Day of Infamy", Engine::new(447_820), 27015);
game_query_mod!(dst, "Don't Starve Together", Engine::new(322_320), 27016);
game_query_mod!(enshrouded, "Enshrouded", Engine::new(1_203_620), 15637);
game_query_mod!(garrysmod, "Garry's Mod", Engine::new(4000), 27016);
game_query_mod!(hl2d, "Half-Life 2 Deathmatch", Engine::new(320), 27015);
game_query_mod!(
    hlds,
    "Half-Life Deathmatch: Source",
    Engine::new(360),
    27015
);
game_query_mod!(hll, "Hell Let Loose", Engine::new(686_810), 26420);
game_query_mod!(
    imic,
    "Insurgency: Modern Infantry Combat",
    Engine::new(17700),
    27015
);
game_query_mod!(insurgency, "Insurgency", Engine::new(222_880), 27015);
game_query_mod!(
    insurgencysandstorm,
    "Insurgency: Sandstorm",
    Engine::new(581_320),
    27131
);
game_query_mod!(l4d, "Left 4 Dead", Engine::new(500), 27015);
game_query_mod!(l4d2, "Left 4 Dead 2", Engine::new(550), 27015);
game_query_mod!(
    ohd,
    "Operation: Harsh Doorstop",
    Engine::new_with_dedicated(736_590, 950_900),
    27005
);
game_query_mod!(onset, "Onset", Engine::new(1_105_810), 7776);
game_query_mod!(postscriptum, "Post Scriptum", Engine::new(736_220), 10037);
game_query_mod!(
    projectzomboid,
    "Project Zomboid",
    Engine::new(108_600),
    16261
);
game_query_mod!(risingworld, "Rising World", Engine::new(324_080), 4254);
game_query_mod!(ror2, "Risk of Rain 2", Engine::new(632_360), 27016);
game_query_mod!(rust, "Rust", Engine::new(252_490), 27015);
game_query_mod!(sco, "Sven Co-op", Engine::new_gold_src(false), 27015);
game_query_mod!(sdtd, "7 Days to Die", Engine::new(251_570), 26900);
game_query_mod!(squad, "Squad", Engine::new(393_380), 27165);
game_query_mod!(teamfortress2, "Team Fortress 2", Engine::new(440), 27015);
game_query_mod!(
    tfc,
    "Team Fortress Classic",
    Engine::new_gold_src(false),
    27015
);
game_query_mod!(theforest, "The Forest", Engine::new(556_450), 27016);
game_query_mod!(thefront, "The Front", Engine::new(2_285_150), 27015);
game_query_mod!(unturned, "Unturned", Engine::new(304_930), 27015);
game_query_mod!(
    valheim,
    "Valheim",
    Engine::new(892_970),
    2457,
    GatheringSettings {
        players: GatherToggle::Enforce,
        rules: GatherToggle::Skip,
        check_app_id: true,
    }
);
game_query_mod!(vrising, "V Rising", Engine::new(1_604_030), 27016);
game_query_mod!(zps, "Zombie Panic: Source", Engine::new(17_500), 27015);
game_query_mod!(moe, "Myth of Empires", Engine::new(1_371_580), 12888);
game_query_mod!(mordhau, "Mordhau", Engine::new(629_760), 27015);
