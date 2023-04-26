use crate::bufferer::{Bufferer, Endianess};
use crate::socket::{Socket, UdpSocket};
use crate::valve_master_server::{Region, SearchFilters};
use crate::{GDError, GDResult};

fn construct_payload(region: Region, filters: Option<SearchFilters>, last_ip: String, last_port: u16) -> Vec<u8> {
    let filters_bytes: Vec<u8> = match filters {
        None => vec![0x00],
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
        &filters_bytes,
    ]
    .concat()
}

pub fn query(region: Region, search_filters: Option<SearchFilters>) -> GDResult<Vec<String>> {
    let mut socket = UdpSocket::new("hl2master.steampowered.com", 27011)?;
    socket.apply_timeout(None)?;

    let initial_payload = construct_payload(region, search_filters, "0.0.0.0".to_string(), 0);
    println!("{:02X?}", initial_payload);
    socket.send(&initial_payload)?;

    let received_data = socket.receive(Some(1400))?;
    let mut buf = Bufferer::new_with_data(Endianess::Big, &received_data);

    if buf.get_u32()? != 4294967295 || buf.get_u16()? != 26122 {
        return Err(GDError::PacketBad);
    }

    while buf.remaining_length() > 0 {
        let first_octet = buf.get_u8()?;
        let second_octet = buf.get_u8()?;
        let third_octet = buf.get_u8()?;
        let fourth_octet = buf.get_u8()?;

        let port = buf.get_u16()?;

        println!("{first_octet}.{second_octet}.{third_octet}.{fourth_octet}:{port}");
    }

    println!("{:02X?}", buf.remaining_data());

    Ok(Vec::new())
}

#[cfg(test)]
mod master_query {
    use crate::valve_master_server::{query, Filter, Region, SearchFilters};

    #[test]
    fn test_stuff() {
        let search_filters = SearchFilters::new()
            .add(Filter::AppId(440))
            .add(Filter::CanHavePassword(false));

        query(Region::Europe, Some(search_filters)).unwrap();
    }
}
