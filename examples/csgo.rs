
use gamedig::CSGO;

fn main() {
    let response = CSGO::query("51.38.142.109", None);
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
