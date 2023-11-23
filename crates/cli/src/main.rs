use std::net::{IpAddr, ToSocketAddrs};

use clap::Parser;
use gamedig::{
    games::*,
    protocols::types::{CommonResponse, ExtraRequestSettings, TimeoutSettings},
};

mod error;

use self::error::{Error, Result};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    /// Unique identifier of the game for which server information is being
    /// queried.
    #[arg(short, long)]
    game: String,

    /// Hostname or IP address of the server.
    #[arg(short, long)]
    ip: String,

    /// Optional query port number for the server. If not provided the default
    /// port for the game is used.
    #[arg(short, long)]
    port: Option<u16>,

    /// Flag indicating if the output should be in JSON format.
    #[cfg(feature = "json")]
    #[arg(short, long)]
    json: bool,

    /// Optional timeout settings for the server query.
    #[command(flatten)]
    timeout_settings: Option<TimeoutSettings>,

    /// Optional extra settings for the server query.
    #[command(flatten)]
    extra_options: Option<ExtraRequestSettings>,
}

/// Attempt to find a game from the [library game definitions](GAMES) based on
/// its unique identifier.
///
/// # Arguments
/// * `game_id` - A string slice containing the unique game identifier.
///
/// # Returns
/// * Result<&'static [Game]> - On sucess returns a reference to the game
///   definition; on failure returns a [Error::UnknownGame] error.
fn find_game(game_id: &str) -> Result<&'static Game> {
    // Attempt to retrieve the game from the predefined game list
    GAMES
        .get(game_id)
        .ok_or_else(|| Error::UnknownGame(game_id.to_string()))
}

/// Resolve an IP address by either parsing an IP address or doing a DNS lookup.
/// In the case of DNS lookup update extra request options with the hostname.
///
/// # Arguments
/// * `host` - A string slice containing the IP address or hostname of a server
///   to resolve.
/// * `extra_options` - Mutable reference to extra options for the game query.
///
/// # Returns
/// * `Result<IpAddr>` - On sucess returns a resolved IP address; on failure
///   returns an [Error::InvalidHostname] error.
fn resolve_ip_or_domain(host: &str, extra_options: &mut Option<ExtraRequestSettings>) -> Result<IpAddr> {
    if let Ok(parsed_ip) = host.parse() {
        Ok(parsed_ip)
    } else {
        set_hostname_if_missing(host, extra_options);

        resolve_domain(host)
    }
}

/// Resolve a domain name to one of its IP addresses (the first one returned).
///
/// # Arguments
/// * `domain` - A string slice containing the domain name to lookup.
///
/// # Returns
/// * `Result<IpAddr>` - On success, returns one of the resolved IP addresses;
///   on failure returns an [Error::InvalidHostname] error.
fn resolve_domain(domain: &str) -> Result<IpAddr> {
    // Append a dummy port to perform socket address resolution and then extract the
    // IP
    Ok(format!("{}:0", domain)
        .to_socket_addrs()
        .map_err(|_| Error::InvalidHostname(domain.to_string()))?
        .next()
        .ok_or_else(|| Error::InvalidHostname(domain.to_string()))?
        .ip())
}

/// Sets the hostname on extra request settings if it is not already set.
///
/// # Arguments
/// * `host` - A string slice containing the hostname.
/// * `extra_options` - A mutable reference to optional [ExtraRequestSettings].
fn set_hostname_if_missing(host: &str, extra_options: &mut Option<ExtraRequestSettings>) {
    if let Some(extra_options) = extra_options {
        if extra_options.hostname.is_none() {
            // If extra_options exists but hostname is None overwrite hostname in place
            extra_options.hostname = Some(host.to_string())
        }
    } else {
        // If extra_options is None create default settings with hostname
        *extra_options = Some(ExtraRequestSettings::default().set_hostname(host.to_string()));
    }
}

/// Output the result of a query to stdout.
///
/// # Arguments
/// * `args` - A reference to the command line options.
/// * `result` - A reference to the result of the query.
fn output_result(args: &Cli, result: &dyn CommonResponse) {
    #[cfg(feature = "json")]
    if args.json {
        // Output response as JSON (and early return)
        return output_result_json(result);
    }

    // Output debug formatted response
    output_result_debug(result);
}

/// Output the result using debug formatting.
///
/// # Arguments
/// * `result` - A reference to the result of the query.
fn output_result_debug(result: &dyn CommonResponse) {
    println!("{:#?}", result.as_original());
}

/// Output the result as a JSON object.
///
/// # Arguments
/// * `result` - A reference to the result of the query.
#[cfg(feature = "json")]
fn output_result_json(result: &dyn CommonResponse) {
    serde_json::to_writer_pretty(std::io::stdout(), &result.as_json()).unwrap();
}

fn main() -> Result<()> {
    // Parse the command line arguments
    let args = Cli::parse();

    // Retrieve the game based on the provided ID
    let game = find_game(&args.game)?;

    // Extract extra options for use in setup
    let mut extra_options = args.extra_options.clone();

    // Resolve the IP address
    let ip = resolve_ip_or_domain(&args.ip, &mut extra_options)?;

    // Query the server using game definition, parsed IP, and user command line
    // flags.
    let result = query_with_timeout_and_extra_settings(game, &ip, args.port, args.timeout_settings, extra_options)?;

    // Output the result in the specified format
    output_result(&args, result.as_ref());

    Ok(())
}
