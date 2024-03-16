use gamedig::protocols::epic::EpicProtocol;

// THIS IS JUST FOR TESTING DEV PURPOSES, REMOVE AFTERWARDS

pub fn main() {
    let deployment = String::from("ad9a8feffb3b4b2ca315546f038c3ae2");
    let id = String::from("xyza7891muomRmynIIHaJB9COBKkwj6n");
    let secret = String::from("PP5UGxysEieNfSrEicaD1N2Bb3TdXuD7xHYcsdUHZ7s");
    let mut epic = EpicProtocol::new(deployment, id, secret).unwrap();
    let token = epic.auth_by_client().unwrap();
    println!("{token}");
}

