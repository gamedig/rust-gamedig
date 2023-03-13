use gamedig::games::mc;

fn main() {
    // or Some(<port>), None is the default protocol port (which is 25565 for java
    // and 19132 for bedrock)
    let response = mc::query("127.0.0.1", None);

    match response {
        Err(error) => println!("Couldn't query, error: {}", error),
        Ok(r) => println!("{:#?}", r),
    }
}
