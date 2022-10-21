
use gamedig::games::l4d;

fn main() {
    let response = l4d::query("207.246.72.170", Some(26999));
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
