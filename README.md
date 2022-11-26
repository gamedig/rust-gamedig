# rust-gamedig
[![CI](https://github.com/cosminperram/rust-gamedig/actions/workflows/ci.yml/badge.svg)](https://github.com/CosminPerRam/rust-gamedig/actions)
[![Latest Version](https://img.shields.io/crates/v/gamedig.svg?color=yellow)](https://crates.io/crates/gamedig)
[![Crates.io](https://img.shields.io/crates/d/gamedig?color=purple)](https://crates.io/crates/gamedig)
![Lines of code](https://img.shields.io/tokei/lines/github/cosminperram/rust-gamedig?color=blue)
[![License:MIT](https://img.shields.io/github/license/cosminperram/rust-gamedig?color=blue)](LICENSE.md)

**rust-GameDig** is a games/services server query library that can fetch the availability and/or details of those, this library brings what **[node-GameDig](https://github.com/gamedig/node-gamedig)** does, to pure Rust!  

MSRV is `1.58.1` and the code is cross-platform.

## Games/Services/Protocols List
To see the supported (or the planned to support) games/services/protocols, see [GAMES](GAMES.md), [SERVICES](SERVICES.md) and [PROTOCOLS](PROTOCOLS.md) respectively.

## Usage
Just pick a game/service/protocol, provide the ip and the port (can be optional) (some use a special query port) then query on it.  
Team Fortress 2 query example:
```rust
use gamedig::games::tf2;

fn main() {
    let response = tf2::query("localhost", None); //or Some(27015), None is the default protocol port
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:#?}", r)
    }
}
```
Response (note that some games have a different structure):
```json5
{
  protocol: 17,
  name: "Team Fortress 2 Dedicated Server.",
  map: "ctf_turbine",
  game: "tf2",
  players: 0,
  players_details: [],
  max_players: 69,
  bots: 0,
  server_type: Dedicated,
  has_password: false,
  vac_secured: true,
  version: "7638371",
  port: Some(27015),
  steam_id: Some(69753253289735296),
  tv_port: None,
  tv_name: None,
  keywords: Some("alltalk,nocrits"),
  rules: [
    ServerRule {
      name: "mp_autoteambalance",
      value: "1",
    }
    //....
  ]
}
```

To see more examples, see the [examples](examples) folder.

## Documentation
The documentation is available at [docs.rs](https://docs.rs/gamedig/latest/gamedig/).  
Curious about the history and what changed between versions? Check out the [CHANGELOG](CHANGELOG.md) file.

## Contributing
If you want see your favorite game/service being supported here, open an issue, and I'll prioritize it (or do a pull request if you want to implement it yourself)!
