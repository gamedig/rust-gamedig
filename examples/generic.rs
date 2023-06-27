use gamedig::{
    protocols::types::{CommonResponse, TimeoutSettings},
    query_with_timeout,
    GDResult,
    GAMES,
};

use std::net::IpAddr;

fn generic_query(
    game_name: &str,
    addr: &IpAddr,
    port: Option<u16>,
    timeout_settings: Option<TimeoutSettings>,
) -> GDResult<Box<dyn CommonResponse>> {
    let game = GAMES.get(game_name).expect("Game doesn't exist");

    println!("Querying {:#?} with game {:#?}.", addr, game);

    let response = query_with_timeout(game, addr, port, timeout_settings)?;
    println!("Response: {:#?}", response.as_json());

    let common = response.as_original();
    println!("Common response: {:#?}", common);

    Ok(response)
}

fn main() {
    let mut args = std::env::args().skip(1);

    let game_name = args.next().expect("Must provide a game name");
    let addr: IpAddr = args
        .next()
        .map(|s| s.parse().unwrap())
        .expect("Must provide address");
    let port: Option<u16> = args.next().map(|s| s.parse().unwrap());

    generic_query(&game_name, &addr, port, None).unwrap();
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
