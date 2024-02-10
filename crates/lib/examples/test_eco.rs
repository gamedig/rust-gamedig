use gamedig::games::eco;
use std::net::IpAddr;
use std::str::FromStr;

fn main() {
    let ip = IpAddr::from_str("142.132.154.69").unwrap();
    let port = 31111;
    let r = eco::query(&ip, Some(port));
    println!("{:#?}", r);
}
