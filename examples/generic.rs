use gamedig::{protocols::GenericResponse, query, GDResult, GAMES};

use std::net::IpAddr;

fn generic_query(game_name: &str, addr: &IpAddr, port: Option<u16>) -> GDResult<GenericResponse> {
    let game = GAMES.get(game_name).expect("Game doesn't exist");

    println!("Querying {:#?} with game {:#?}.", addr, game.name);

    let response = query(game, addr, port)?;
    println!("Response: {:#?}", response);

    let common = response.as_common();
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

    generic_query(&game_name, &addr, port).unwrap();
}

#[cfg(test)]
mod test {
    use gamedig::GAMES;
    use std::net::{IpAddr, Ipv4Addr};

    use super::generic_query;

    const ADDR: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

    fn test_game(game_name: &str) {
        assert!(generic_query(game_name, &ADDR, None).is_err());
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
