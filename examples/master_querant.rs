
use std::env;
use gamedig::{aliens, ase, asrd, cscz, csgo, css, dod, dods, GDResult, gm, hl2dm, ins, insmic, inss, l4d, l4d2, mc, sdtd, tf2, ts, unturned};
use gamedig::protocols::minecraft::{LegacyGroup, Server};
use gamedig::protocols::valve;
use gamedig::protocols::valve::App;

fn main() -> GDResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args[1] == "help".to_string() {
        println!("Usage: <game> <ip> <port>");
        println!("       <game> - any game, example: tf2");
        println!("       <ip> - an ip, example: 192.168.0.0");
        println!("       <port> - an port, optional, example: 27015");
        return Ok(());
    } else if args.len() < 3 {
        println!("Minimum number of arguments: 3, try 'help' to see the details.");
        return Ok(());
    }

    let ip = args[2].as_str();
    let port = match args.len() == 4 {
        false => None,
        true => Some(args[3].parse::<u16>().expect("Invalid port!"))
    };

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
        "mc" => println!("{:#?}", mc::query(ip, port)?),
        "mc_java" => println!("{:#?}", mc::query_specific(Server::Java, ip, port)?),
        "mc_legacy_vb1_8" => println!("{:#?}", mc::query_specific(Server::Legacy(LegacyGroup::VB1_8), ip, port)?),
        "mc_legacy_v1_4" => println!("{:#?}", mc::query_specific(Server::Legacy(LegacyGroup::V1_4), ip, port)?),
        "mc_legacy_v1_6" => println!("{:#?}", mc::query_specific(Server::Legacy(LegacyGroup::V1_6), ip, port)?),
        "7dtd" => println!("{:#?}", sdtd::query(ip, port)?),
        "ase" => println!("{:#?}", ase::query(ip, port)?),
        "unturned" => println!("{:#?}", unturned::query(ip, port)?),
        "_src" => println!("{:#?}", valve::query(ip, 27015, App::Source(None), None, None)?),
        "_gld" => println!("{:#?}", valve::query(ip, 27015, App::GoldSrc(false), None, None)?),
        "_gld_f" => println!("{:#?}", valve::query(ip, 27015, App::GoldSrc(true), None, None)?),
        _ => panic!("Undefined game: {}", args[1])
    };

    Ok(())
}
