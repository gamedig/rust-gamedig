# rust-GameDig [![CI](https://github.com/gamedig/rust-gamedig/actions/workflows/ci.yml/badge.svg)](https://github.com/gamedig/rust-gamedig/actions) [![Latest Version](https://img.shields.io/crates/v/gamedig.svg?color=yellow)](https://crates.io/crates/gamedig) [![Crates.io](https://img.shields.io/crates/d/gamedig?color=purple)](https://crates.io/crates/gamedig) [![License:MIT](https://img.shields.io/github/license/gamedig/rust-gamedig?color=blue)](LICENSE.md)

**Warning**: This project goes through frequent API breaking changes and hasn't been thoroughly tested.

**rust-GameDig** is a game servers/services query library that fetches the availability and details of those, this library brings what **[node-GameDig](https://github.com/gamedig/node-gamedig)** does, to pure Rust!  

## Community
Checkout the GameDig Community Discord Server [here](https://discord.gg/NVCMn3tnxH).  
Note that it isn't be a replacement for GitHub issues, if you have found a problem
within the library or want to request a feature, it's better to do so here rather than
on Discord.

## Usage
Minimum Supported Rust Version is `1.60.0` and the code is cross-platform.

Pick a game/service/protocol (check the [GAMES](GAMES.md), [SERVICES](SERVICES.md) and [PROTOCOLS](PROTOCOLS.md) files to see the currently supported ones), provide the ip and the port (be aware that some game servers use a separate port for the info queries, the port can also be optional if the server is running the default ports) then query on it.  

[Team Fortress 2](https://store.steampowered.com/app/440/Team_Fortress_2/) query example:
```rust
use gamedig::games::tf2;

fn main() {
    let response = tf2::query(&"127.0.0.1".parse().unwrap(), None); 
        // None is the default port (which is 27015), could also be Some(27015)
    
    match response { // Result type, must check what it is...
        Err(error) => println!("Couldn't query, error: {}", error),
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
  appid: 440,
  players_online: 0,
  players_details: [],
  players_maximum: 69,
  players_bots: 0,
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
    "mp_autoteambalance": "1",
    "mp_maxrounds": "5",
    //....
  ]
}
```

Want to see more examples? Checkout the [examples](examples) folder.

## Documentation
The documentation is available at [docs.rs](https://docs.rs/gamedig/latest/gamedig/).  
Curious about the history and what changed between versions? Everything is in the [CHANGELOG](CHANGELOG.md) file.

## Contributing
If you want see your favorite game/service being supported here, open an issue, and I'll prioritize it (or do a pull request if you want to implement it yourself)!
