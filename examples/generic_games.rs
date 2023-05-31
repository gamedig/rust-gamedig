fn main() {
    let games = gamedig::games::games();

    for entry in &games {
        println!("{:?} -> {:?}", entry.0, entry.1.name());
    }

    let game = games.get("tf2").unwrap();
    let response = game.query(&"127.0.0.1".parse().unwrap(), None).unwrap();
    println!("{:?} {:?}", response.server_name(), response.server_map());
}
