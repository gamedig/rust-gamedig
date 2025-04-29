use gamedig::games::fivem::query;

fn main() {
    let result = query(&"127.0.0.1".parse().expect("Failed to parse IP"), None);
    match result {
        Ok(response) => {
            println!("Response: {:#?}", response);
        }
        Err(error) => {
            println!("Error: {error}");
        }
    }
}
