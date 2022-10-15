
use gamedig::TF2;

fn main() {
    let response = TF2::query("5.15.202.107", None);
    match response {
        Err(_) => println!("fuck"),
        Ok(r) => {
            println!("{:?}", r);

            ()
        }
    }
}
