
use gamedig::games::csgo;

fn main() {
    let response = csgo::query("216.52.148.47", None);
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
