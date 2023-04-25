use crate::socket::{Socket, UdpSocket};
use crate::valve_master_server::{Filters, Region};
use crate::GDResult;

fn construct_payload(region: Region, filters: Option<Filters>, last_ip: String, last_port: u16) -> Vec<u8> {
    let filters_bytes: &[u8] = match filters {
        None => &[0x00],
        Some(f) => f.to_bytes(),
    };

    let region_byte = &[region as u8];

    [
        // Packet has to begin with the character '1'
        &[0x31],
        // The region byte is next
        region_byte,
        // The last fetched ip as a string
        last_ip.as_bytes(),
        // Followed by an ':'
        &[':' as u8],
        // And the port, as a string
        last_port.to_string().as_bytes(),
        // Which needs to end with a NULL byte
        &[0x00],
        // Then the filters
        filters_bytes,
    ]
    .concat()
}

pub fn query(region: Region, filters: Option<Filters>) -> GDResult<Vec<String>> {
    let mut socket = UdpSocket::new("hl2master.steampowered.com", 27011)?;
    socket.apply_timeout(None)?;

    let initial_payload = construct_payload(region, filters, "0.0.0.0".to_string(), 0);
    println!("{:02X?}", initial_payload);
    socket.send(&initial_payload)?;

    let received_data = socket.receive(Some(1400))?;
    println!("{:02X?}", received_data);

    Ok(Vec::new())
}

#[cfg(test)]
mod master_query {
    use crate::valve_master_server::{query, Region};

    #[test]
    fn test_stuff() { query(Region::Europe, None).unwrap(); }
}
