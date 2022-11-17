
use gamedig::games::minecraft;
use gamedig::protocols::minecraft::Server;

fn main() {
    let response = minecraft::query(Server::Java, "top.mccentral.org", None); //or Some(25565), None is the default protocol port (which is 25565)
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
