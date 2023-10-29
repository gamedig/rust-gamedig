use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use gamedig::protocols::types::TimeoutSettings;
use gamedig::protocols::valve;
use gamedig::protocols::valve::{Engine, GatheringSettings};

fn main() {
    let address = &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 27015);
    let engine = Engine::Source(None); // We don't specify a steam app id, let the query try to find it.
    let gather_settings = GatheringSettings {
        players: true, // We want to query for players
        rules: false, // We don't want to query for rules
        check_app_id: false, // Loosen up the query a bit by not checking app id
    };

    let read_timeout = Duration::from_secs(2);
    let write_timeout = Duration::from_secs(3);
    let retries = 1; // does another request if the first one fails.
    let timeout_settings = TimeoutSettings::new(Some(read_timeout), Some(write_timeout), retries).unwrap();

    let response = valve::query(address, engine, Some(gather_settings), Some(timeout_settings));
    println!("{response:#?}");
}
