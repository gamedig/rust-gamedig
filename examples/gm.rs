
use gamedig::games::gm;

fn main() {
    let response = gm::query("148.59.74.84", None);
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
