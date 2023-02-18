
use gamedig::games::tf2;

fn main() {
    let response = tf2::query("127.0.0.1", None); //or Some(27015), None is the default protocol port (which is 27015)
    match response { // Result type, must check what it is...
        Err(error) => println!("Couldn't query, error: {}", error),
        Ok(r) => println!("{:#?}", r)
    }
}
