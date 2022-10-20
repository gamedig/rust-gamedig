
use gamedig::games::csgo;

fn main() {
    let response = csgo::query("51.38.142.109", None);
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
