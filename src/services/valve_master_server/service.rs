use crate::bufferer::{Bufferer, Endianess};
use crate::socket::{Socket, UdpSocket};
use crate::valve_master_server::{Region, SearchFilters};
use crate::{GDError, GDResult};
use std::net::Ipv4Addr;

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
        &[b':'],
        // And the port, as a string
        last_port.to_string().as_bytes(),
        // Which needs to end with a NULL byte
        &[0x00],
        // Then the filters
        &filters_bytes,
    ]
    .concat()
}

pub fn query(region: Region, search_filters: Option<SearchFilters>) -> GDResult<Vec<(Ipv4Addr, u16)>> {
    let mut socket = UdpSocket::new("hl2master.steampowered.com", 27011)?;
    socket.apply_timeout(None)?;

    let initial_payload = construct_payload(region, search_filters, "0.0.0.0".to_string(), 0);
    socket.send(&initial_payload)?;

    let received_data = socket.receive(Some(1400))?;
    let mut buf = Bufferer::new_with_data(Endianess::Big, &received_data);

    if buf.get_u32()? != 4294967295 || buf.get_u16()? != 26122 {
        return Err(GDError::PacketBad);
    }

    let mut ips: Vec<(Ipv4Addr, u16)> = Vec::new();
    while buf.remaining_length() > 0 {
        let ip = Ipv4Addr::new(buf.get_u8()?, buf.get_u8()?, buf.get_u8()?, buf.get_u8()?);
        let port = buf.get_u16()?;

        ips.push((ip, port));
    }

    Ok(ips)
}

#[cfg(test)]
mod master_query {
    use crate::valve_master_server::{query, Filter, Region, SearchFilters};

    #[test]
    fn test_stuff() {
        let search_filters = SearchFilters::new()
            .add(Filter::AppId(440))
            .add(Filter::CanHavePassword(false));

        println!("{:?}", query(Region::Europe, Some(search_filters)).unwrap());
    }
}
