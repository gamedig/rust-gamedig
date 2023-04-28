use crate::bufferer::{Bufferer, Endianess};
use crate::socket::{Socket, UdpSocket};
use crate::valve_master_server::{Region, SearchFilters};
use crate::{GDError, GDResult};
use std::net::Ipv4Addr;

/// The default master ip, which is the one for Source.
pub const DEFAULT_MASTER_IP: &str = "hl2master.steampowered.com";
/// The default master port.
pub const DEFAULT_MASTER_PORT: u16 = 27011;

fn construct_payload(region: Region, filters: &Option<SearchFilters>, last_ip: &str, last_port: u16) -> Vec<u8> {
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

/// The implementation, use this if you want to keep the same socket.
pub struct ValveMasterServer {
    socket: UdpSocket,
}

impl ValveMasterServer {
    /// Construct a new struct.
    pub fn new(master_ip: &str, master_port: u16) -> GDResult<Self> {
        let socket = UdpSocket::new(master_ip, master_port)?;
        socket.apply_timeout(None)?;

        Ok(Self { socket })
    }

    /// Make just a single query, providing `0.0.0.0` as the last ip and `0` as
    /// the last port will give the initial packet.
    pub fn query_specific(
        &mut self,
        region: Region,
        search_filters: &Option<SearchFilters>,
        last_address_ip: &str,
        last_address_port: u16,
    ) -> GDResult<Vec<(Ipv4Addr, u16)>> {
        let payload = construct_payload(region, search_filters, last_address_ip, last_address_port);
        println!("{:02X?}", payload);
        self.socket.send(&payload)?;

        let received_data = self.socket.receive(Some(1400))?;
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

    /// Make a complete query.
    pub fn query(&mut self, region: Region, search_filters: Option<SearchFilters>) -> GDResult<Vec<(Ipv4Addr, u16)>> {
        let mut ips: Vec<(Ipv4Addr, u16)> = Vec::new();

        let mut exit_fetching = false;
        let mut last_ip: String = "0.0.0.0".to_string();
        let mut last_port: u16 = 0;

        while !exit_fetching {
            let new_ips = self.query_specific(region, &search_filters, last_ip.as_str(), last_port)?;

            match new_ips.last() {
                None => exit_fetching = true,
                Some((latest_ip, latest_port)) => {
                    let mut remove_last = false;

                    let latest_ip_string = latest_ip.to_string();
                    if latest_ip_string == "0.0.0.0" && *latest_port == 0 {
                        exit_fetching = true;
                        remove_last = true;
                    } else if latest_ip_string == last_ip && *latest_port == last_port {
                        exit_fetching = true;
                    } else {
                        last_ip = latest_ip_string;
                        last_port = *latest_port;
                    }

                    ips.extend(new_ips);
                    if remove_last {
                        ips.pop();
                    }
                }
            }
        }

        Ok(ips)
    }
}

/// Take only the first response of (what would be a) complete query. This is
/// faster as it results in less packets being sent, received and processed but
/// yields less ips.
pub fn query_singular(region: Region, search_filters: Option<SearchFilters>) -> GDResult<Vec<(Ipv4Addr, u16)>> {
    let mut master_server = ValveMasterServer::new(DEFAULT_MASTER_IP, DEFAULT_MASTER_PORT)?;

    let mut ips = master_server.query_specific(region, &search_filters, "0.0.0.0", 0)?;

    if let Some((last_ip, last_port)) = ips.last() {
        if last_ip.to_string() == "0.0.0.0" && *last_port == 0 {
            ips.pop();
        }
    }

    Ok(ips)
}

/// Make a complete query.
pub fn query(region: Region, search_filters: Option<SearchFilters>) -> GDResult<Vec<(Ipv4Addr, u16)>> {
    let mut master_server = ValveMasterServer::new(DEFAULT_MASTER_IP, DEFAULT_MASTER_PORT)?;

    master_server.query(region, search_filters)
}
