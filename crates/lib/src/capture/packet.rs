use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

/// Size of a standard network packet.
pub(crate) const PACKET_SIZE: usize = 5012;
/// Size of an Ethernet header.
pub(crate) const HEADER_SIZE_ETHERNET: usize = 14;
/// Size of an IPv4 header.
pub(crate) const HEADER_SIZE_IP4: usize = 20;
/// Size of an IPv6 header.
pub(crate) const HEADER_SIZE_IP6: usize = 40;
/// Size of a UDP header.
pub(crate) const HEADER_SIZE_UDP: usize = 4;

/// Represents the direction of a network packet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Direction {
    /// Packet is outgoing (sent by us).
    Send,
    /// Packet is incoming (received by us).
    Receive,
}

/// Defines the protocol of a network packet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Protocol {
    /// Transmission Control Protocol.
    TCP,
    /// User Datagram Protocol.
    UDP,
}

/// Trait for handling different types of IP addresses (IPv4, IPv6).
pub(crate) trait IpAddress: Sized {
    /// Creates an instance from a standard `IpAddr`, returning `None` if the types are incompatible.
    fn from_std(ip: IpAddr) -> Option<Self>;
}

/// Represents a captured network packet with metadata.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CapturePacket<'a> {
    /// Direction of the packet (Send/Receive).
    pub(crate) direction: Direction,
    /// Protocol of the packet (TCP/UDP).
    pub(crate) protocol: Protocol,
    /// Remote socket address.
    pub(crate) remote_address: &'a SocketAddr,
    /// Local socket address.
    pub(crate) local_address: &'a SocketAddr,
}

impl CapturePacket<'_> {
    /// Retrieves the local and remote ports based on the packet's direction.
    ///
    /// Returns:
    /// - (u16, u16): Tuple of (source port, destination port).
    pub(super) const fn ports_by_direction(&self) -> (u16, u16) {
        let (local, remote) = (self.local_address.port(), self.remote_address.port());
        self.direction.order(local, remote)
    }

    /// Retrieves the local and remote IP addresses.
    ///
    /// Returns:
    /// - (IpAddr, IpAddr): Tuple of (local IP, remote IP).
    pub(super) fn ip_addr(&self) -> (IpAddr, IpAddr) {
        let (local, remote) = (self.local_address.ip(), self.remote_address.ip());
        (local, remote)
    }

    /// Retrieves IP addresses of a specific type (IPv4 or IPv6) based on the packet's direction.
    ///
    /// Panics if the IP type of the addresses does not match the requested type.
    ///
    /// Returns:
    /// - (T, T): Tuple of (source IP, destination IP) of the specified type in order.
    pub(super) fn ipvt_by_direction<T: IpAddress>(&self) -> (T, T) {
        let (local, remote) = (
            T::from_std(self.local_address.ip()).expect("Incorrect IP type for local address"),
            T::from_std(self.remote_address.ip()).expect("Incorrect IP type for remote address"),
        );

        self.direction.order(local, remote)
    }
}

impl Direction {
    /// Orders two elements (source and destination) based on the packet's direction.
    ///
    /// Returns:
    /// - (T, T): Ordered tuple (source, destination).
    pub(self) const fn order<T>(&self, source: T, remote: T) -> (T, T) {
        match self {
            Direction::Send => (source, remote),
            Direction::Receive => (remote, source),
        }
    }
}

/// Implements the `IpAddress` trait for `Ipv4Addr`.
impl IpAddress for Ipv4Addr {
    /// Creates an `Ipv4Addr` from a standard `IpAddr`, if it's IPv4.
    fn from_std(ip: IpAddr) -> Option<Self> {
        match ip {
            IpAddr::V4(ipv4) => Some(ipv4),
            _ => None,
        }
    }
}

/// Implements the `IpAddress` trait for `Ipv6Addr`.
impl IpAddress for Ipv6Addr {
    /// Creates an `Ipv6Addr` from a standard `IpAddr`, if it's IPv6.
    fn from_std(ip: IpAddr) -> Option<Self> {
        match ip {
            IpAddr::V6(ipv6) => Some(ipv6),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // Helper function to create a SocketAddr from a string
    fn socket_addr(addr: &str) -> SocketAddr {
        SocketAddr::from_str(addr).unwrap()
    }

    #[test]
    fn test_ports_by_direction() {
        let packet_send = CapturePacket {
            direction: Direction::Send,
            protocol: Protocol::TCP,
            local_address: &socket_addr("127.0.0.1:8080"),
            remote_address: &socket_addr("192.168.1.1:80"),
        };

        let packet_receive = CapturePacket {
            direction: Direction::Receive,
            protocol: Protocol::TCP,
            local_address: &socket_addr("127.0.0.1:8080"),
            remote_address: &socket_addr("192.168.1.1:80"),
        };

        assert_eq!(packet_send.ports_by_direction(), (8080, 80));
        assert_eq!(packet_receive.ports_by_direction(), (80, 8080));
    }

    #[test]
    fn test_ip_addr() {
        let packet_send = CapturePacket {
            direction: Direction::Send,
            protocol: Protocol::TCP,
            local_address: &socket_addr("127.0.0.1:8080"),
            remote_address: &socket_addr("192.168.1.1:80"),
        };

        let packet_receive = CapturePacket {
            direction: Direction::Receive,
            protocol: Protocol::TCP,
            local_address: &socket_addr("127.0.0.1:8080"),
            remote_address: &socket_addr("192.168.1.1:80"),
        };

        assert_eq!(
            packet_send.ip_addr(),
            (
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))
            )
        );
        assert_eq!(
            packet_receive.ip_addr(),
            (
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))
            )
        );
    }

    #[test]
    fn test_ip_by_direction_type_specific() {
        let packet = CapturePacket {
            direction: Direction::Send,
            protocol: Protocol::TCP,
            local_address: &socket_addr("127.0.0.1:8080"),
            remote_address: &socket_addr("192.168.1.1:80"),
        };

        let ipv4_result: Result<(Ipv4Addr, Ipv4Addr), _> =
            std::panic::catch_unwind(|| packet.ipvt_by_direction::<Ipv4Addr>());
        assert!(ipv4_result.is_ok());

        let ipv6_result: Result<(Ipv6Addr, Ipv6Addr), _> =
            std::panic::catch_unwind(|| packet.ipvt_by_direction::<Ipv6Addr>());
        assert!(ipv6_result.is_err());
    }
}
