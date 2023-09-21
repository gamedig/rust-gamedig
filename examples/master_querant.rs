use gamedig::protocols::minecraft::LegacyGroup;
use gamedig::protocols::valve;
use gamedig::protocols::valve::Engine;
use gamedig::protocols::{gamespy, quake};
use gamedig::{
    ageofchivalry,
    alienswarm,
    arkse,
    arma2oa,
    asrd,
    avorion,
    ballisticoverkill,
    bat1944,
    bf1942,
    blackmesa,
    brainbread2,
    codenamecure,
    colonysurvival,
    creativerse,
    crysiswars,
    cs,
    cscz,
    csgo,
    css,
    dod,
    dods,
    doi,
    dst,
    ffow,
    garrysmod,
    haloce,
    hl2dm,
    hldms,
    hll,
    insurgency,
    insurgencymic,
    insurgencysandstorm,
    jc2mp,
    left4dead,
    left4dead2,
    mc,
    ohd,
    onset,
    przomboid,
    quake1,
    quake2,
    quake3,
    ror2,
    rust,
    sd2d,
    ship,
    sof2,
    ss,
    svencoop,
    tf,
    tf2,
    tfc,
    unturned,
    ut,
    vrising,
    warsow,
    GDResult,
};
use std::env;
use std::net::{IpAddr, SocketAddr};

fn main() -> GDResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args[1] == *"help" {
        println!("Usage: <game> <ip> <port>");
        println!("       <game> - any game, example: tf2");
        println!("       <ip> - an ip, example: 192.168.0.0");
        println!("       <port> - an port, optional, example: 27015");
        return Ok(());
    } else if args.len() < 3 {
        println!("Minimum number of arguments: 3, try 'help' to see the details.");
        return Ok(());
    }

    let ip = &args[2].as_str().parse::<IpAddr>().unwrap();
    let port = match args.len() == 4 {
        false => {
            if args[1].starts_with('_') {
                panic!("The port must be specified with an anonymous query.")
            }

            None
        }
        true => Some(args[3].parse::<u16>().expect("Invalid port!")),
    };
    let address = &SocketAddr::new(*ip, port.unwrap_or(0));

    match args[1].as_str() {
        "alienswarm" => println!("{:#?}", alienswarm::query(ip, port)?),
        "asrd" => println!("{:#?}", asrd::query(ip, port)?),
        "csgo" => println!("{:#?}", csgo::query(ip, port)?),
        "css" => println!("{:#?}", css::query(ip, port)?),
        "dods" => println!("{:#?}", dods::query(ip, port)?),
        "garrysmod" => println!("{:#?}", garrysmod::query(ip, port)?),
        "hl2dm" => println!("{:#?}", hl2dm::query(ip, port)?),
        "tf2" => println!("{:#?}", tf2::query(ip, port)?),
        "insurgencymic" => println!("{:#?}", insurgencymic::query(ip, port)?),
        "insurgency" => println!("{:#?}", insurgency::query(ip, port)?),
        "insurgencysandstorm" => println!("{:#?}", insurgencysandstorm::query(ip, port)?),
        "left4dead" => println!("{:#?}", left4dead::query(ip, port)?),
        "left4dead2" => println!("{:#?}", left4dead2::query(ip, port)?),
        "ship" => println!("{:#?}", ship::query(ip, port)?),
        "cscz" => println!("{:#?}", cscz::query(ip, port)?),
        "dod" => println!("{:#?}", dod::query(ip, port)?),
        "_src" => {
            println!(
                "{:#?}",
                valve::query(address, Engine::Source(None), None, None)?
            )
        }
        "_gld" => {
            println!(
                "{:#?}",
                valve::query(address, Engine::GoldSrc(false), None, None)?
            )
        }
        "_gld_f" => {
            println!(
                "{:#?}",
                valve::query(address, Engine::GoldSrc(true), None, None)?
            )
        }
        "mc" => println!("{:#?}", mc::query(ip, port)?),
        "mc_java" => println!("{:#?}", mc::query_java(ip, port, None)?),
        "mc_bedrock" => println!("{:#?}", mc::query_bedrock(ip, port)?),
        "mc_legacy" => println!("{:#?}", mc::query_legacy(ip, port)?),
        "mc_legacy_vb1_8" => {
            println!(
                "{:#?}",
                mc::query_legacy_specific(LegacyGroup::VB1_8, ip, port)?
            )
        }
        "mc_legacy_v1_4" => {
            println!(
                "{:#?}",
                mc::query_legacy_specific(LegacyGroup::V1_4, ip, port)?
            )
        }
        "mc_legacy_v1_6" => {
            println!(
                "{:#?}",
                mc::query_legacy_specific(LegacyGroup::V1_6, ip, port)?
            )
        }
        "7dtd" => println!("{:#?}", sd2d::query(ip, port)?),
        "arkse" => println!("{:#?}", arkse::query(ip, port)?),
        "unturned" => println!("{:#?}", unturned::query(ip, port)?),
        "tf" => println!("{:#?}", tf::query(ip, port)?),
        "tfc" => println!("{:#?}", tfc::query(ip, port)?),
        "svencoop" => println!("{:#?}", svencoop::query(ip, port)?),
        "rust" => println!("{:#?}", rust::query(ip, port)?),
        "cs" => println!("{:#?}", cs::query(ip, port)?),
        "arma2oa" => println!("{:#?}", arma2oa::query(ip, port)?),
        "doi" => println!("{:#?}", doi::query(ip, port)?),
        "hldms" => println!("{:#?}", hldms::query(ip, port)?),
        "ror2" => println!("{:#?}", ror2::query(ip, port)?),
        "bat1944" => println!("{:#?}", bat1944::query(ip, port)?),
        "blackmesa" => println!("{:#?}", blackmesa::query(ip, port)?),
        "przomboid" => println!("{:#?}", przomboid::query(ip, port)?),
        "ageofchivalry" => println!("{:#?}", ageofchivalry::query(ip, port)?),
        "dst" => println!("{:#?}", dst::query(ip, port)?),
        "colonysurvival" => println!("{:#?}", colonysurvival::query(ip, port)?),
        "onset" => println!("{:#?}", onset::query(ip, port)?),
        "codenamecure" => println!("{:#?}", codenamecure::query(ip, port)?),
        "ballisticoverkill" => println!("{:#?}", ballisticoverkill::query(ip, port)?),
        "brainbread2" => println!("{:#?}", brainbread2::query(ip, port)?),
        "avorion" => println!("{:#?}", avorion::query(ip, port)?),
        "ohd" => println!("{:#?}", ohd::query(ip, port)?),
        "vrising" => println!("{:#?}", vrising::query(ip, port)?),
        "_gamespy1" => println!("{:#?}", gamespy::one::query(address, None)),
        "_gamespy1_vars" => println!("{:#?}", gamespy::one::query_vars(address, None)),
        "ut" => println!("{:#?}", ut::query(ip, port)),
        "bf1942" => println!("{:#?}", bf1942::query(ip, port)),
        "ss" => println!("{:#?}", ss::query(ip, port)),
        "_gamespy3" => println!("{:#?}", gamespy::three::query(address, None)),
        "_gamespy3_vars" => println!("{:#?}", gamespy::three::query_vars(address, None)),
        "ffow" => println!("{:#?}", ffow::query(ip, port)),
        "crysiswars" => println!("{:#?}", crysiswars::query(ip, port)),
        "_quake1" => println!("{:#?}", quake::one::query(address, None)),
        "_quake2" => println!("{:#?}", quake::two::query(address, None)),
        "_quake3" => println!("{:#?}", quake::three::query(address, None)),
        "quake2" => println!("{:#?}", quake2::query(ip, port)?),
        "quake1" => println!("{:#?}", quake1::query(ip, port)?),
        "quake3" => println!("{:#?}", quake3::query(ip, port)?),
        "hll" => println!("{:#?}", hll::query(ip, port)?),
        "sof2" => println!("{:#?}", sof2::query(ip, port)?),
        "_gamespy2" => println!("{:#?}", gamespy::two::query(address, None)),
        "haloce" => println!("{:#?}", haloce::query(ip, port)?),
        "jc2mp" => println!("{:#?}", jc2mp::query(ip, port)?),
        "warsow" => println!("{:#?}", warsow::query(ip, port)?),
        "creativerse" => println!("{:#?}", creativerse::query(ip, port)?),
        _ => panic!("Undefined game: {}", args[1]),
    };

    Ok(())
}
