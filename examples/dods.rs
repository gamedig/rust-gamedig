
use gamedig::games::dods;

fn main() {
    let response = dods::query("88.99.28.151", Some(27055));
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
