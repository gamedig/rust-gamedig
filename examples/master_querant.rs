
use std::env;
use gamedig::{aliens, asrd, csgo, css, dods, gm, hl2dm, ins, insmic, inss, l4d, l4d2, tf2, ts};
use gamedig::valve::ValveProtocol;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args[1] == "help".to_string() {
        println!("Usage: <game> <ip> <port>");
        println!("       <game> - any game, example: tf2");
        println!("       <ip> - an ip, example: 192.168.0.0");
        println!("       <port> - an port, optional, example: 27015");
        return;
    } else if args.len() < 3 {
        println!("Minimum number of arguments: 3, try 'help' to see the details.");
        return;
    }

    let ip = args[2].as_str();
    let port = match args.len() == 4 {
        false => None,
        true => Some(args[3].parse::<u16>().expect("Invalid port!"))
    };

    match args[1].as_str() {
        "aliens" => println!("{:?}", aliens::query(ip, port)),
        "asrd" => println!("{:?}", asrd::query(ip, port)),
        "csgo" => println!("{:?}", csgo::query(ip, port)),
        "css" => println!("{:?}", css::query(ip, port)),
        "dods" => println!("{:?}", dods::query(ip, port)),
        "gm" => println!("{:?}", gm::query(ip, port)),
        "hl2dm" => println!("{:?}", hl2dm::query(ip, port)),
        "tf2" => println!("{:?}", tf2::query(ip, port)),
        "insmic" => println!("{:?}", insmic::query(ip, port)),
        "ins" => println!("{:?}", ins::query(ip, port)),
        "inss" => println!("{:?}", inss::query(ip, port)),
        "l4d" => println!("{:?}", l4d::query(ip, port)),
        "l4d2" => println!("{:?}", l4d2::query(ip, port)),
        "ts" => println!("{:?}", ts::query(ip, port)),
        "_" => println!("{:?}", ValveProtocol::query(ip, 27015, None, None)),
        _ => panic!("Undefined game: {}", args[1])
    };
}
