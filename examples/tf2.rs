
use gamedig::TF2;

fn main() {
    let response = TF2::query("91.216.250.10", None);
    match response {
        Err(error) => println!("Couldn't query, error: {}", error),
        Ok(r) => println!("{:?}", r)
    }
}
