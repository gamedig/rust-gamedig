
use gamedig::games::asrd;

fn main() {
    let response = asrd::query("5.199.135.237", Some(30000));
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
