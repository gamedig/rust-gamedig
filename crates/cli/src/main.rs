use std::net::ToSocketAddrs;

use clap::Parser;
use gamedig::{
    games::*,
    protocols::types::{ExtraRequestSettings, TimeoutSettings},
};

mod error;

use self::error::Result;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Game ID
    #[arg(short, long)]
    game: String,

    /// Hostname or IP address of the server
    #[arg(short, long)]
    ip: String,

    /// Optional port number to override the default for the game
    #[arg(short, long)]
    port: Option<u16>,

    /// Output result as JSON
    #[cfg(feature = "json")]
    #[arg(short, long)]
    json: bool,

    #[command(flatten)]
    timeout_settings: Option<TimeoutSettings>,

    #[command(flatten)]
    extra_options: Option<ExtraRequestSettings>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let game = match GAMES.get(&args.game) {
        Some(game) => game,
        None => return Err(error::Error::UnknownGame(args.game)),
    };

    let mut extra_request_settings = if let Some(extra) = args.extra_options {
        extra
    } else {
        gamedig::protocols::ExtraRequestSettings::default()
    };

    let ip = if let Ok(ip) = args.ip.parse() {
        ip
    } else {
        // Set hostname in extra request settings
        if extra_request_settings.hostname.is_none() {
            extra_request_settings.hostname = Some(args.ip.clone());
        }

        // Use ToSocketAddrs to do a DNS lookup
        // unfortunatley this requires a format to add a port
        format!("{}:0", args.ip)
            .to_socket_addrs()
            .map_err(|_| error::Error::InvalidHostname(args.ip.clone()))?
            .next()
            .ok_or(error::Error::InvalidHostname(args.ip))?
            .ip()
    };

    let result = query_with_timeout_and_extra_settings(
        game,
        &ip,
        args.port,
        args.timeout_settings,
        Some(extra_request_settings),
    )?;

    #[cfg(feature = "json")]
    if args.json {
        serde_json::to_writer_pretty(std::io::stdout(), &result.as_json()).unwrap();
    } else {
        println!("{:#?}", result.as_original());
    }
    #[cfg(not(feature = "json"))]
    println!("{:#?}", result.as_original());

    Ok(())
}
