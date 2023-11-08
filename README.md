<h1 align="center">rust-GameDig</h1>

<h5 align="center">The fast library for querying game servers/services.</h5>

<div align="center">
  <a href="https://github.com/gamedig/rust-gamedig/actions">
    <img src="https://github.com/gamedig/rust-gamedig/actions/workflows/ci.yml/badge.svg" alt="CI">
  </a>
  <a href="https://crates.io/crates/gamedig">
    <img src="https://img.shields.io/crates/v/gamedig.svg?color=yellow" alt="Latest Version">
  </a>
  <a href="https://crates.io/crates/gamedig">
    <img src="https://img.shields.io/crates/d/gamedig?color=purple" alt="Crates.io">
  </a>
  <a href="https://github.com/gamedig/node-gamedig">
    <img src="./.github/badges/node.svg" alt="Node-GameDig Game Coverage">
  </a>
</div>

<h5 align="center">
  This library brings what
  <a href="https://github.com/gamedig/node-gamedig">
    node-GameDig
  </a>
  does (and not only), to pure Rust!
</h5>

**Warning**: This project goes through frequent API breaking changes and hasn't been thoroughly tested.

## Community
Checkout the GameDig Community Discord Server [here](https://discord.gg/NVCMn3tnxH).  
Note that it isn't be a replacement for GitHub issues, if you have found a problem
within the library or want to request a feature, it's better to do so here rather than
on Discord.

## Usage
Minimum Supported Rust Version is `1.65.0` and the code is cross-platform.

Pick a game/service/protocol (check the [GAMES](GAMES.md), [SERVICES](SERVICES.md) and [PROTOCOLS](PROTOCOLS.md) files to see the currently supported ones), provide the ip and the port (be aware that some game servers use a separate port for the info queries, the port can also be optional if the server is running the default ports) then query on it.  

[Team Fortress 2](https://store.steampowered.com/app/440/Team_Fortress_2/) query example:
```rust
use gamedig::games::teamfortress2;

fn main() {
    let response = teamfortress2::query(&"127.0.0.1".parse().unwrap(), None); 
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
If you want to see your favorite game/service being supported here, open an issue, and I'll prioritize it (or do a pull request if you want to implement it yourself)!

Before contributing please read [CONTRIBUTING](CONTRIBUTING.md).
