use gamedig::protocols::minecraft::LegacyGroup;
use gamedig::protocols::valve;
use gamedig::protocols::valve::Engine;
use gamedig::protocols::{gamespy, quake};
use gamedig::{
    aliens,
    aoc,
    arma2oa,
    ase,
    asrd,
    avorion,
    bat1944,
    bb2,
    bf1942,
    bm,
    bo,
    ccure,
    cosu,
    creativerse,
    cs,
    cscz,
    csgo,
    css,
    cw,
    dod,
    dods,
    doi,
    dst,
    ffow,
    gm,
    haloce,
    hl2dm,
    hldms,
    hll,
    ins,
    insmic,
    inss,
    jc2mp,
    l4d,
    l4d2,
    mc,
    ohd,
    onset,
    pz,
    quake1,
    quake2,
    quake3a,
    ror2,
    rust,
    sc,
    sdtd,
    sof2,
    ss,
    tf,
    tf2,
    tfc,
    ts,
    unturned,
    ut,
    vr,
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
        "aliens" => println!("{:#?}", aliens::query(ip, port)?),
        "asrd" => println!("{:#?}", asrd::query(ip, port)?),
        "csgo" => println!("{:#?}", csgo::query(ip, port)?),
        "css" => println!("{:#?}", css::query(ip, port)?),
        "dods" => println!("{:#?}", dods::query(ip, port)?),
        "gm" => println!("{:#?}", gm::query(ip, port)?),
        "hl2dm" => println!("{:#?}", hl2dm::query(ip, port)?),
        "tf2" => println!("{:#?}", tf2::query(ip, port)?),
        "insmic" => println!("{:#?}", insmic::query(ip, port)?),
        "ins" => println!("{:#?}", ins::query(ip, port)?),
        "inss" => println!("{:#?}", inss::query(ip, port)?),
        "l4d" => println!("{:#?}", l4d::query(ip, port)?),
        "l4d2" => println!("{:#?}", l4d2::query(ip, port)?),
        "ts" => println!("{:#?}", ts::query(ip, port)?),
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
        "7dtd" => println!("{:#?}", sdtd::query(ip, port)?),
        "ase" => println!("{:#?}", ase::query(ip, port)?),
        "unturned" => println!("{:#?}", unturned::query(ip, port)?),
        "tf" => println!("{:#?}", tf::query(ip, port)?),
        "tfc" => println!("{:#?}", tfc::query(ip, port)?),
        "sc" => println!("{:#?}", sc::query(ip, port)?),
        "rust" => println!("{:#?}", rust::query(ip, port)?),
        "cs" => println!("{:#?}", cs::query(ip, port)?),
        "arma2oa" => println!("{:#?}", arma2oa::query(ip, port)?),
        "doi" => println!("{:#?}", doi::query(ip, port)?),
        "hldms" => println!("{:#?}", hldms::query(ip, port)?),
        "ror2" => println!("{:#?}", ror2::query(ip, port)?),
        "bat1944" => println!("{:#?}", bat1944::query(ip, port)?),
        "bm" => println!("{:#?}", bm::query(ip, port)?),
        "pz" => println!("{:#?}", pz::query(ip, port)?),
        "aoc" => println!("{:#?}", aoc::query(ip, port)?),
        "dst" => println!("{:#?}", dst::query(ip, port)?),
        "cosu" => println!("{:#?}", cosu::query(ip, port)?),
        "onset" => println!("{:#?}", onset::query(ip, port)?),
        "ccure" => println!("{:#?}", ccure::query(ip, port)?),
        "bo" => println!("{:#?}", bo::query(ip, port)?),
        "bb2" => println!("{:#?}", bb2::query(ip, port)?),
        "avorion" => println!("{:#?}", avorion::query(ip, port)?),
        "ohd" => println!("{:#?}", ohd::query(ip, port)?),
        "vr" => println!("{:#?}", vr::query(ip, port)?),
        "_gamespy1" => println!("{:#?}", gamespy::one::query(address, None)),
        "_gamespy1_vars" => println!("{:#?}", gamespy::one::query_vars(address, None)),
        "ut" => println!("{:#?}", ut::query(ip, port)),
        "bf1942" => println!("{:#?}", bf1942::query(ip, port)),
        "ss" => println!("{:#?}", ss::query(ip, port)),
        "_gamespy3" => println!("{:#?}", gamespy::three::query(address, None)),
        "_gamespy3_vars" => println!("{:#?}", gamespy::three::query_vars(address, None)),
        "ffow" => println!("{:#?}", ffow::query(ip, port)),
        "cw" => println!("{:#?}", cw::query(ip, port)),
        "_quake1" => println!("{:#?}", quake::one::query(address, None)),
        "_quake2" => println!("{:#?}", quake::two::query(address, None)),
        "_quake3" => println!("{:#?}", quake::three::query(address, None)),
        "quake2" => println!("{:#?}", quake2::query(ip, port)?),
        "quake1" => println!("{:#?}", quake1::query(ip, port)?),
        "quake3a" => println!("{:#?}", quake3a::query(ip, port)?),
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
