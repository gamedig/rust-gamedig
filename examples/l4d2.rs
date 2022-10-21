
use gamedig::games::l4d2;

fn main() {
    let response = l4d2::query("74.91.124.246", None);
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
