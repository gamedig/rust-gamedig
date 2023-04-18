use crate::protocols::valve::{Engine, ValveProtocol};
use crate::GDResult;

pub fn query(address: &str, port: Option<u16>) -> GDResult<()> {
    let mut client = ValveProtocol::new(address, port.unwrap_or(5478), None)?;
    let buffer = client.get_request_data(
        &Engine::Source(None),
        0,
        0x46,
        String::from("LSQ").into_bytes(),
    )?;

    println!("{:02X?}", buffer.remaining_data());

    Ok(())
}
