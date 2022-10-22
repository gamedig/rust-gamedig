
use gamedig::games::ins;

fn main() {
    let response = ins::query("101.100.139.94", Some(27016));
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
