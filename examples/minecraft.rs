
use gamedig::games::minecraft;
use gamedig::protocols::minecraft::{LegacyVersion, Server};

fn main() {
    let response = minecraft::query(Server::Legacy(LegacyVersion::V1_4), "localhost", None); //or Some(25565), None is the default protocol port (which is 25565)
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
