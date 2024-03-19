use gamedig::protocols::epic;
use gamedig::protocols::epic::Credentials;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// THIS IS JUST FOR TESTING DEV PURPOSES, REMOVE AFTERWARDS
// cargo r --example test --all-features

pub fn main() {
    let deployment = String::from("ad9a8feffb3b4b2ca315546f038c3ae2");
    let id = String::from("xyza7891muomRmynIIHaJB9COBKkwj6n");
    let secret = String::from("PP5UGxysEieNfSrEicaD1N2Bb3TdXuD7xHYcsdUHZ7s");
    let credentials = Credentials {
        deployment,
        id,
        secret,
        auth_by_external: false,
    };
    // 148.251.176.37:7080
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(148, 251, 176, 37)), 7080);
    let data = epic::query(credentials, &address).unwrap();
    println!("{:#?}", data);
}
