use std::net::IpAddr;

use clap::Parser;
use gamedig::games::*;

mod error;

use self::error::Result;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)]
    game: String,

    #[arg(short, long)]
    ip: IpAddr,

    #[arg(short, long)]
    port: Option<u16>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let game = match GAMES.get(&args.game) {
        Some(game) => game,
        None => return Err(error::Error::UnknownGame(args.game)),
    };

    println!("{:#?}", query(game, &args.ip, args.port)?.as_json());

    Ok(())
}
