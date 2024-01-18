use gamedig::minecraft;
use gamedig::minecraft::types::RequestSettings;

fn main() {
    // or Some(<port>), None is the default protocol port (which is 25565 for java
    // and 19132 for bedrock)
    let response = minecraft::query(&"127.0.0.1".parse().unwrap(), None);
    // This will fail if no server is available locally!

    match response {
        Err(error) => println!("Couldn't query, error: {}", error),
        Ok(r) => println!("{:#?}", r),
    }

    // This is an example to query a server with a hostname to be specified in the
    // packet. Passing -1 on the protocol_version means anything, note that
    // an invalid value here might result in server not responding.
    let response = minecraft::query_java(
        &"209.222.114.62".parse().unwrap(),
        Some(25565),
        Some(RequestSettings {
            hostname: "mc.hypixel.net".to_string(),
            protocol_version: -1,
        }),
    );

    match response {
        Err(error) => println!("Couldn't query, error: {}", error),
        Ok(r) => println!("{:#?}", r),
    }
}
