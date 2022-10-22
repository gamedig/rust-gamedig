
use gamedig::games::inss;

fn main() {
    let response = inss::query("109.195.19.160", None); //The query port, not the server port
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
