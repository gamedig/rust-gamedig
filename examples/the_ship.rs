
use gamedig::TheShip;

fn main() {
    let response = TheShip::query("46.4.48.226", Some(27017));
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
