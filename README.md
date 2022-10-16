# rust-gamedig
rust-GameDig is a game server/services query library, capable of querying the status of many games/services, this library brings what [node-GameDig](https://github.com/gamedig/node-gamedig) does, to pure Rust!  

MSRV is `1.58.1` and the code is cross-platform.

# Example
Basic usage of the library is:
```rust
use gamedig::TF2;

fn main() {
    let response = TF2::query("91.216.250.10", None);
    //query your favorite game/protocol/service, some might come with different parameters
    //here its just the IP and the port (if None, its gonna be the default from the protocol)
    
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
```
To see more examples, see the [examples](examples) folder.

# Documentation
The documentation is available at [docs.rs](https://docs.rs/gamedig/latest/gamedig/).

# Games List
To see the supported games, see [GAMES](GAMES.md).

# Contributing
If you want see your favorite game/service being supported here, open an issue (or do a pull request if you want to implement it yourself)!
