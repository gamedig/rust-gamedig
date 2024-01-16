use std::{
    net::{IpAddr, ToSocketAddrs},
    path::PathBuf,
};

use clap::{Parser, Subcommand, ValueEnum};
use gamedig::{
    games::*,
    protocols::types::{CommonResponse, ExtraRequestSettings, TimeoutSettings},
};

mod error;

use self::error::{Error, Result};

const GAMEDIG_HEADER: &str = r"

  _____                      _____  _          _____ _      _____ 
 / ____|                    |  __ \(_)        / ____| |    |_   _|
| |  __  __ _ _ __ ___   ___| |  | |_  __ _  | |    | |      | |  
| | |_ |/ _` | '_ ` _ \ / _ \ |  | | |/ _` | | |    | |      | |  
| |__| | (_| | | | | | |  __/ |__| | | (_| | | |____| |____ _| |_ 
 \_____|\__,_|_| |_| |_|\___|_____/|_|\__, |  \_____|______|_____|
                                       __/ |                      
                                      |___/      

        A command line interface for querying game servers.
  Copyright (C) 2022 - 2023 GameDig Organization & Contributors
                  Licensed under the MIT license
";

// NOTE: For some reason without setting long_about here the doc comment for
// ExtraRequestSettings gets set as the about for the CLI.
#[derive(Debug, Parser)]
#[command(author, version, about = GAMEDIG_HEADER, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Query game server information
    Query {
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

        /// Specifies the output format
        #[arg(short, long, default_value = "debug", value_enum)]
        format: OutputFormat,

        /// Which response variant to use when outputting
        #[arg(short, long, default_value = "generic")]
        output_mode: OutputMode,

        /// Optional file path for packet capture file writer
        #[cfg(feature = "packet_capture")]
        #[arg(short, long)]
        capture: Option<PathBuf>,

        /// Optional timeout settings for the server query
        #[command(flatten, next_help_heading = "Timeouts")]
        timeout_settings: Option<TimeoutSettings>,

        /// Optional extra settings for the server query
        #[command(flatten, next_help_heading = "Query options")]
        extra_options: Option<ExtraRequestSettings>,
    },

    /// Check out the source code
    Source,
    /// Display the MIT License information
    License,
}

#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
enum OutputMode {
    /// A generalised response that maps common fields from all game types to
    /// the same name.
    Generic,
    /// The raw result returned from the protocol query, formatted similarly to
    /// how the server returned it.
    ProtocolSpecific,
}

#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    /// Human readable structured output
    Debug,
    /// RFC 8259
    #[cfg(feature = "json")]
    JsonPretty,
    /// RFC 8259
    #[cfg(feature = "json")]
    Json,
    /// Parser tries to be mostly XML 1.1 (RFC 7303) compliant
    #[cfg(feature = "xml")]
    Xml,
    /// RFC 4648 section 8
    #[cfg(feature = "bson")]
    BsonHex,
    /// RFC 4648 section 4
    #[cfg(feature = "bson")]
    BsonBase64,
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
fn resolve_ip_or_domain<T: AsRef<str>>(host: T, extra_options: &mut Option<ExtraRequestSettings>) -> Result<IpAddr> {
    let host_str = host.as_ref();
    if let Ok(parsed_ip) = host_str.parse() {
        Ok(parsed_ip)
    } else {
        set_hostname_if_missing(host_str, extra_options);
        resolve_domain(host_str)
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
fn output_result<T: CommonResponse + ?Sized>(output_mode: OutputMode, format: OutputFormat, result: &T) {
    match format {
        OutputFormat::Debug => match output_mode {
            OutputMode::Generic => output_result_debug(result.as_json()),
            OutputMode::ProtocolSpecific => output_result_debug(result.as_original()),
        },
        #[cfg(feature = "json")]
        OutputFormat::JsonPretty => match output_mode {
            OutputMode::Generic => output_result_json_pretty(result.as_json()),
            OutputMode::ProtocolSpecific => output_result_json_pretty(result.as_original()),
        },
        #[cfg(feature = "json")]
        OutputFormat::Json => match output_mode {
            OutputMode::Generic => output_result_json(result.as_json()),
            OutputMode::ProtocolSpecific => output_result_json(result.as_original()),
        },
        #[cfg(feature = "xml")]
        OutputFormat::Xml => match output_mode {
            OutputMode::Generic => output_result_xml(result.as_json()),
            //BUG: In this case we get a writer error with all serde write methods some reason with xml 0.6.0
            //BUG: gamedig-cli.exe query -g <GAME> -i <IP> -p <PORT> -f xml --output-mode protocol-specific
            //BUG: Writer: emitter error: document start event has already been emitted
            //BUG: With xml 0.5.1 we get unsupported operation: 'serialize_unit_variant'
            OutputMode::ProtocolSpecific => panic!("XML format is not supported for protocol specific output"),
        },
        #[cfg(feature = "bson")]
        OutputFormat::BsonHex => match output_mode {
            OutputMode::Generic => output_result_bson_hex(result.as_json()),
            OutputMode::ProtocolSpecific => output_result_bson_hex(result.as_original()),
        },
        #[cfg(feature = "bson")]
        OutputFormat::BsonBase64 => match output_mode {
            OutputMode::Generic => output_result_bson_base64(result.as_json()),
            OutputMode::ProtocolSpecific => output_result_bson_base64(result.as_original()),
        },
    }
}

/// Output the result using debug formatting.
///
/// # Arguments
/// * `result` - A result that can be output using the debug formatter.
fn output_result_debug<R: std::fmt::Debug>(result: R) {
    println!("{:#?}", result);
}

/// Output the result as a JSON object.
///
/// # Arguments
/// * `result` - A serde serializable result.
#[cfg(feature = "json")]
fn output_result_json<T: serde::Serialize>(result: T) {
    println!("{}", serde_json::to_string(&result).unwrap());
}

#[cfg(feature = "json")]
fn output_result_json_pretty<T: serde::Serialize>(result: T) {
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

/// Output the result as an XML object.
/// # Arguments
/// * `result` - A serde serializable result.
fn output_result_xml<T: serde::Serialize>(result: T) {
    println!("{}", serde_xml_rs::to_string(&result).unwrap());
}

#[cfg(feature = "bson")]
fn output_result_bson_hex<T: serde::Serialize>(result: T) {
    let bson = bson::to_bson(&result).unwrap();

    if let bson::Bson::Document(document) = bson {
        let bytes = bson::to_vec(&document).unwrap();

        println!("{}", hex::encode(bytes));
    } else {
        panic!("Failed to convert result to BSON Hex");
    }
}

#[cfg(feature = "bson")]
fn output_result_bson_base64<T: serde::Serialize>(result: T) {
    use base64::Engine;

    let bson = bson::to_bson(&result).unwrap();

    if let bson::Bson::Document(document) = bson {
        let bytes = bson::to_vec(&document).unwrap();

        println!("{}", base64::prelude::BASE64_STANDARD.encode(&bytes));
    } else {
        panic!("Failed to convert result to BSON Base64");
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.action {
        Action::Query {
            game,
            ip,
            port,
            format,
            output_mode,
            capture,
            timeout_settings,
            extra_options,
        } => {
            // Process the query command
            let game = find_game(&game)?;
            let mut extra_options = extra_options;
            let ip = resolve_ip_or_domain(&ip, &mut extra_options)?;

            #[cfg(feature = "packet_capture")]
            gamedig::capture::setup_capture(capture);

            let result = query_with_timeout_and_extra_settings(game, &ip, port, timeout_settings, extra_options)?;
            output_result(output_mode, format, result.as_ref());
        }
        Action::Source => {
            println!("{}", GAMEDIG_HEADER);

            #[cfg(feature = "browser")]
            {
                // Directly offering to open the URL
                println!("\nWould you like to open the GitHub repository in your default browser? [Y/n]");

                let mut choice = String::new();
                std::io::stdin().read_line(&mut choice).unwrap();
                if choice.trim().eq_ignore_ascii_case("Y") {
                    if webbrowser::open("https://github.com/gamedig/rust-gamedig").is_ok() {
                        println!("Opening GitHub repository in default browser...");
                    } else {
                        println!("Failed to open GitHub repository in default browser.");
                        println!("Please use the following URL: https://github.com/gamedig/rust-gamedig");
                    }
                } else {
                    println!("Not to worry, you can always open the repository manually");
                    println!("by visiting the following URL: https://github.com/gamedig/rust-gamedig");
                }
            }

            #[cfg(not(feature = "browser"))]
            {
                println!("\nYou can find the source code for this project at the following URL:");
                println!("https://github.com/gamedig/rust-gamedig");
            }

            println!("\nBe sure to leave a star if you like the project :)");
        }
        Action::License => {
            // Bake the license into the binary
            // so we don't have to ship it separately
            println!("{}", include_str!("../../../LICENSE.md"));
        }
    }

    Ok(())
}
