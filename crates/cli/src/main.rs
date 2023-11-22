use std::net::ToSocketAddrs;

use clap::Parser;
use gamedig::{games::*, GDErrorKind};

mod error;

use self::error::Result;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)]
    game: String,

    #[arg(short, long, help = "Hostname or IP address of the server")]
    ip: String,

    #[arg(short, long)]
    port: Option<u16>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let game = match GAMES.get(&args.game) {
        Some(game) => game,
        None => return Err(error::Error::UnknownGame(args.game)),
    };

    let ip = if let Ok(ip) = args.ip.parse() {
        ip
    } else {
        // Use ToSocketAddrs to do a DNS lookup
        // unfortunatley this requires a format to add a port
        format!("{}:0", args.ip)
            .to_socket_addrs()
            .map_err(|e| GDErrorKind::InvalidInput.context(e))?
            .next()
            .ok_or(GDErrorKind::InvalidInput.context(format!("Could not resolve an IP address for {:?}", args.ip)))?
            .ip()
    };

    println!("{:#?}", query(game, &ip, args.port)?.as_json());

    Ok(())
}
