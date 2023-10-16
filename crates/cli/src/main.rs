mod error;
mod key;

use self::{error::Result, key::Game};

use clap::Parser;
use gamedig::games::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)]
    game: Game,

    #[arg(short, long)]
    ip: String,

    #[arg(short, long)]
    port: u16,
}

#[rustfmt::skip]
fn main() -> Result<()> {
    let args = Cli::parse();

    match args.game {
        Game::AlienSwarm => Ok(println!("{:#?}", aliens::query(&args.ip, Some(args.port))?)),
        Game::AgeOfChivalry => Ok(println!("{:#?}", aoc::query(&args.ip, Some(args.port))?)),
        Game::ARMA2OperationArrowhead => Ok(println!("{:#?}", arma2oa::query(&args.ip, Some(args.port))?)),
        Game::ARKSurvivalEvolved => Ok(println!("{:#?}", ase::query(&args.ip, Some(args.port))?)),
        Game::AlienSwarmReactiveDrop => Ok(println!("{:#?}", asrd::query(&args.ip, Some(args.port))?)),
        Game::Avorion => Ok(println!("{:#?}", avorion::query(&args.ip, Some(args.port))?)),
        Game::Battalion1944 => Ok(println!("{:#?}", bat1944::query(&args.ip, Some(args.port))?)),
        Game::BrainBread2 => Ok(println!("{:#?}", bb2::query(&args.ip, Some(args.port))?)),
        Game::Battlefield1942 => Ok(println!("{:#?}", bf1942::query(&args.ip, Some(args.port))?)),
        Game::BlackMesa => Ok(println!("{:#?}", bm::query(&args.ip, Some(args.port))?)),
        Game::BallisticOverkill => Ok(println!("{:#?}", bo::query(&args.ip, Some(args.port))?)),
        Game::CodenameCURE => Ok(println!("{:#?}", ccure::query(&args.ip, Some(args.port))?)),
        Game::ColonySurvival => Ok(println!("{:#?}", cosu::query(&args.ip, Some(args.port))?)),
        Game::CounterStrike => Ok(println!("{:#?}", cs::query(&args.ip, Some(args.port))?)),
        Game::CounterStrikeConditionZero => Ok(println!("{:#?}", cscz::query(&args.ip, Some(args.port))?)),
        Game::CounterStrikeGlobalOffensive => Ok(println!("{:#?}", csgo::query(&args.ip, Some(args.port))?)),
        Game::CounterStrikeSource => Ok(println!("{:#?}", css::query(&args.ip, Some(args.port))?)),
        Game::DayOfDefeat => Ok(println!("{:#?}", dod::query(&args.ip, Some(args.port))?)),
        Game::DayOfDefeatSource => Ok(println!("{:#?}", dods::query(&args.ip, Some(args.port))?)),
        Game::DayOfInfamy => Ok(println!("{:#?}", doi::query(&args.ip, Some(args.port))?)),
        Game::DontStarveTogether => Ok(println!("{:#?}", dst::query(&args.ip, Some(args.port))?)),
        Game::GarrysMod => Ok(println!("{:#?}", gm::query(&args.ip, Some(args.port))?)),
        Game::HalfLife2Deathmatch => Ok(println!("{:#?}", hl2dm::query(&args.ip, Some(args.port))?)),
        Game::HalfLifeDeathmatchSource => Ok(println!("{:#?}", hldms::query(&args.ip, Some(args.port))?)),
        Game::Insurgency => Ok(println!("{:#?}", ins::query(&args.ip, Some(args.port))?)),
        Game::InsurgencyModernInfantryCombat => Ok(println!("{:#?}", insmic::query(&args.ip, Some(args.port))?)),
        Game::InsurgencySandstorm => Ok(println!("{:#?}", inss::query(&args.ip, Some(args.port))?)),
        Game::Left4Dead => Ok(println!("{:#?}", l4d::query(&args.ip, Some(args.port))?)),
        Game::Left4Dead2 => Ok(println!("{:#?}", l4d2::query(&args.ip, Some(args.port))?)),
        Game::Minecraft => Ok(println!("{:#?}", mc::query(&args.ip, Some(args.port))?)),
        Game::OperationHarshDoorstop => Ok(println!("{:#?}", ohd::query(&args.ip, Some(args.port))?)),
        Game::Onset => Ok(println!("{:#?}", onset::query(&args.ip, Some(args.port))?)),
        Game::ProjectZomboid => Ok(println!("{:#?}", pz::query(&args.ip, Some(args.port))?)),
        Game::RiskOfRain2 => Ok(println!("{:#?}", ror2::query(&args.ip, Some(args.port))?)),
        Game::Rust => Ok(println!("{:#?}", rust::query(&args.ip, Some(args.port))?)),
        Game::SvenCoOp => Ok(println!("{:#?}", sc::query(&args.ip, Some(args.port))?)),
        Game::SevenDaysToDie => Ok(println!("{:#?}", sdtd::query(&args.ip, Some(args.port))?)),
        Game::TeamFortress => Ok(println!("{:#?}", tf::query(&args.ip, Some(args.port))?)),
        Game::TeamFortress2 => Ok(println!("{:#?}", tf2::query(&args.ip, Some(args.port))?)),
        Game::TeamFortressClassic => Ok(println!("{:#?}", tfc::query(&args.ip, Some(args.port))?)),
        Game::TheShip =>  Ok(println!("{:#?}", ts::query(&args.ip, Some(args.port))?)),
        Game::Unturned => Ok(println!("{:#?}", unturned::query(&args.ip, Some(args.port))?)),
        Game::UnrealTournament => Ok(println!("{:#?}", ut::query(&args.ip, Some(args.port))?)),
        Game::VRising => Ok(println!("{:#?}", vr::query(&args.ip, Some(args.port))?)),
      
    }
}