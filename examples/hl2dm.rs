
use gamedig::games::hl2dm;

fn main() {
    let response = hl2dm::query("74.91.118.209", None);
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
