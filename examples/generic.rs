use gamedig::{
    protocols::types::{CommonResponse, TimeoutSettings},
    query_with_timeout,
    GDResult,
    GAMES,
};

use std::net::{IpAddr, SocketAddr, ToSocketAddrs};

/// Make a query given the name of a game
fn generic_query(
    game_name: &str,
    addr: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Box<dyn CommonResponse>> {
    let game = GAMES
        .get(game_name)
        .expect("Game doesn't exist, run without arguments to see a list of games");

    println!("Querying {:#?} with game {:#?}.", addr, game);

    let response = query_with_timeout(game, addr, port, timeout_settings)?;
    println!("Response: {:#?}", response.as_json());

    let common = response.as_original();
    println!("Common response: {:#?}", common);

    Ok(response)
}

fn main() {
    let mut args = std::env::args().skip(1);

    // Handle arguments
    if let Some(game_name) = args.next() {
        // Use to_socket_addrs to resolve hostname to IP
        let addr: SocketAddr = args
            .next()
            .map(|s| format!("{}:0", s).to_socket_addrs().unwrap())
            .expect("Must provide address")
            .next()
            .expect("Could not lookup host");
        let port: Option<u16> = args.next().map(|s| s.parse().unwrap());

        generic_query(&game_name, &addr.ip(), port, None).unwrap();
    } else {
        // Without arguments print a list of games

        for (name, game) in gamedig::games::GAMES.entries() {
            println!("{}\t{}", name, game.name);
        }
    }
}

#[cfg(test)]
mod test {
    use gamedig::{protocols::types::TimeoutSettings, GAMES};
    use std::{
        net::{IpAddr, Ipv4Addr},
        time::Duration,
    };

    use super::generic_query;

    const ADDR: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

    fn test_game(game_name: &str) {
        let timeout_settings =
            Some(TimeoutSettings::new(Some(Duration::from_nanos(1)), Some(Duration::from_nanos(1))).unwrap());
        assert!(generic_query(game_name, &ADDR, None, timeout_settings).is_err());
    }

    #[test]
    fn battlefield() { test_game("bf1942"); }

    #[test]
    fn minecraft() { test_game("mc"); }

    #[test]
    fn tf2() { test_game("tf2"); }

    #[test]
    fn quake() { test_game("quake3a"); }

    #[test]
    fn all_games() {
        for game_name in GAMES.keys() {
            test_game(game_name);
        }
    }
}
