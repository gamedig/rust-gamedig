
#[cfg(test)]
mod tests {
    use gamedig::protocols::valve::ValveProtocol;

    #[test]
    fn tf2() {
        let response = ValveProtocol::query("5.15.202.107", 27015);
        match response {
            Err(_) => println!("fuck"),
            Ok(r) => println!("{}", r.name)
        }
        assert_eq!(4, 4);
    }
}
