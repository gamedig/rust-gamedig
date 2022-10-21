
use gamedig::games::css;

fn main() {
    let response = css::query("104.128.58.206", None);
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
