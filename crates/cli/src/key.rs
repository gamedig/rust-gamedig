use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Debug, Clone)]
pub enum Game {
    #[strum(serialize = "aliens")]
    AlienSwarm,
    #[strum(serialize = "aoc")]
    AgeOfChivalry,
    #[strum(serialize = "arma2oa")]
    ARMA2OperationArrowhead,
    #[strum(serialize = "ase")]
    ARKSurvivalEvolved,
    #[strum(serialize = "asrd")]
    AlienSwarmReactiveDrop,
    #[strum(serialize = "avorion")]
    Avorion,
    #[strum(serialize = "bat1944")]
    Battalion1944,
    #[strum(serialize = "bb2")]
    BrainBread2,
    #[strum(serialize = "bf1942")]
    Battlefield1942,
    #[strum(serialize = "bm")]
    BlackMesa,
    #[strum(serialize = "bo")]
    BallisticOverkill,
    #[strum(serialize = "ccure")]
    CodenameCURE,
    #[strum(serialize = "cosu")]
    ColonySurvival,
    #[strum(serialize = "cs")]
    CounterStrike,
    #[strum(serialize = "cscz")]
    CounterStrikeConditionZero,
    #[strum(serialize = "csgo")]
    CounterStrikeGlobalOffensive,
    #[strum(serialize = "css")]
    CounterStrikeSource,
    #[strum(serialize = "dod")]
    DayOfDefeat,
    #[strum(serialize = "dods")]
    DayOfDefeatSource,
    #[strum(serialize = "doi")]
    DayOfInfamy,
    #[strum(serialize = "dst")]
    DontStarveTogether,
    #[strum(serialize = "gm")]
    GarrysMod,
    #[strum(serialize = "hl2dm")]
    HalfLife2Deathmatch,
    #[strum(serialize = "hldms")]
    HalfLifeDeathmatchSource,
    #[strum(serialize = "ins")]
    Insurgency,
    #[strum(serialize = "insmic")]
    InsurgencyModernInfantryCombat,
    #[strum(serialize = "inss")]
    InsurgencySandstorm,
    #[strum(serialize = "l4d")]
    Left4Dead,
    #[strum(serialize = "l4d2")]
    Left4Dead2,
    #[strum(serialize = "mc")]
    Minecraft,
    #[strum(serialize = "ohd")]
    OperationHarshDoorstop,
    #[strum(serialize = "onset")]
    Onset,
    #[strum(serialize = "pz")]
    ProjectZomboid,
    #[strum(serialize = "ror2")]
    RiskOfRain2,
    #[strum(serialize = "rust")]
    Rust,
    #[strum(serialize = "sc")]
    SvenCoOp,
    #[strum(serialize = "sdtd")]
    SevenDaysToDie,
    #[strum(serialize = "tf")]
    TeamFortress,
    #[strum(serialize = "tf2")]
    TeamFortress2,
    #[strum(serialize = "tfc")]
    TeamFortressClassic,
    #[strum(serialize = "ts")]
    TheShip,
    #[strum(serialize = "unturned")]
    Unturned,
    #[strum(serialize = "ut")]
    UnrealTournament,
    #[strum(serialize = "vr")]
    VRising,
}