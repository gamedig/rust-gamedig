use pcap_file::pcapng::{
    blocks::enhanced_packet::{EnhancedPacketBlock, EnhancedPacketOption},
    PcapNgBlock, PcapNgWriter,
};
use pnet_packet::{
    ethernet::{EtherType, EtherTypes, MutableEthernetPacket},
    ip::{IpNextHeaderProtocol, IpNextHeaderProtocols},
    ipv4::MutableIpv4Packet,
    ipv6::MutableIpv6Packet,
    tcp::{MutableTcpPacket, TcpFlags},
    udp::MutableUdpPacket,
    PacketSize,
};
use std::{
    io::Write,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    time::Instant,
};

use super::packet::{
    CapturePacket, Direction, Protocol, HEADER_SIZE_ETHERNET, HEADER_SIZE_IP4, HEADER_SIZE_IP6, HEADER_SIZE_UDP,
    PACKET_SIZE,
};

const DEFAULT_TTL: u8 = 64;
const TCP_WINDOW_SIZE: u16 = 43440;
const BUFFER_SIZE: usize = PACKET_SIZE
    - (if HEADER_SIZE_IP4 > HEADER_SIZE_IP6 {
        HEADER_SIZE_IP4
    } else {
        HEADER_SIZE_IP6
    })
    - HEADER_SIZE_ETHERNET;

pub(crate) struct Pcap<W: Write> {
    writer: PcapNgWriter<W>,
    pub(crate) state: State,
    buffer: Vec<u8>,
}

pub(crate) struct State {
    pub(crate) start_time: Instant,
    pub(crate) send_seq: u32,
    pub(crate) rec_seq: u32,
    pub(crate) has_sent_handshake: bool,
    pub(crate) has_sent_fin: bool,
    pub(crate) stream_count: u32,
}

impl<W: Write> Pcap<W> {
    pub fn new(writer: PcapNgWriter<W>) -> Self {
        Self {
            writer,
            state: State::default(),
            buffer: vec![0; BUFFER_SIZE],
        }
    }

    pub fn write_transport_packet(&mut self, info: &CapturePacket, payload: &[u8]) -> Result<(), std::io::Error> {
        let (source_port, dest_port) = info.ports_by_direction();

        match info.protocol {
            Protocol::TCP => self.handle_tcp(info, payload, source_port, dest_port)?,
            Protocol::UDP => self.handle_udp(info, payload, source_port, dest_port)?,
        }

        Ok(())
    }

    fn handle_tcp(
        &mut self,
        info: &CapturePacket,
        payload: &[u8],
        source_port: u16,
        dest_port: u16,
    ) -> Result<(), std::io::Error> {
        let buf_size = self.setup_tcp_packet(info, payload, source_port, dest_port)?;
        self.write_transport_payload(
            info,
            IpNextHeaderProtocols::Tcp,
            &self.buffer[..buf_size + payload.len()],
            vec![],
        );

        Ok(())
    }

    fn setup_tcp_packet(
        &mut self,
        info: &CapturePacket,
        payload: &[u8],
        source_port: u16,
        dest_port: u16,
    ) -> Result<usize, std::io::Error> {
        let mut tcp = MutableTcpPacket::new(&mut self.buffer)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to create TCP packet"))?;
        tcp.set_source(source_port);
        tcp.set_destination(dest_port);
        tcp.set_payload(payload);
        tcp.set_data_offset(5);
        tcp.set_window(TCP_WINDOW_SIZE);

        // Set sequence and acknowledgement numbers
        match info.direction {
            Direction::Send => {
                tcp.set_sequence(self.state.send_seq);
                tcp.set_acknowledgement(self.state.rec_seq);
                self.state.send_seq = self.state.send_seq.wrapping_add(payload.len() as u32);
            }
            Direction::Receive => {
                tcp.set_sequence(self.state.rec_seq);
                tcp.set_acknowledgement(self.state.send_seq);
                self.state.rec_seq = self.state.rec_seq.wrapping_add(payload.len() as u32);
            }
        }
        tcp.set_flags(TcpFlags::PSH | TcpFlags::ACK);

        Ok(tcp.packet_size())
    }

    pub fn write_tcp_handshake(&mut self, info: &CapturePacket) {
        // Initialize sequence numbers for demonstration purposes
        self.state.send_seq = 500;
        self.state.rec_seq = 1000;

        // Common setup for TCP handshake packets
        let mut tcp_handshake_packet =
            |info: &CapturePacket, direction: Direction, flags: u8| -> Result<(), std::io::Error> {
                let (source_port, dest_port) = info.ports_by_direction();
                let adjusted_info = CapturePacket {
                    direction,
                    ..info.clone()
                };
                self.setup_tcp_packet(&adjusted_info, &[], source_port, dest_port)?;
                Ok(self.write_transport_payload(
                    &adjusted_info,
                    IpNextHeaderProtocols::Tcp,
                    &self.buffer,
                    vec![EnhancedPacketOption::Comment(
                        format!(
                            "Generated TCP {}",
                            match flags {
                                TcpFlags::SYN => "SYN",
                                TcpFlags::SYN | TcpFlags::ACK => "SYN-ACK",
                                TcpFlags::ACK => "ACK",
                            }
                        )
                        .into(),
                    )],
                ))
            };

        // Send SYN
        tcp_handshake_packet(info, Direction::Send, TcpFlags::SYN);

        // Send SYN-ACK
        self.state.send_seq = self.state.send_seq.wrapping_add(1); // Update sequence number after SYN
        tcp_handshake_packet(info, Direction::Receive, TcpFlags::SYN | TcpFlags::ACK);

        // Send ACK
        self.state.rec_seq = self.state.rec_seq.wrapping_add(1); // Update sequence number after SYN-ACK
        tcp_handshake_packet(info, Direction::Send, TcpFlags::ACK);
    }

    fn handle_udp(
        &mut self,
        info: &CapturePacket,
        payload: &[u8],
        source_port: u16,
        dest_port: u16,
    ) -> Result<(), std::io::Error> {
        let mut udp = MutableUdpPacket::new(&mut self.buffer)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to create UDP packet"))?;
        udp.set_source(source_port);
        udp.set_destination(dest_port);
        udp.set_length((payload.len() + HEADER_SIZE_UDP) as u16);
        udp.set_payload(payload);

        let buf_size = udp.packet_size();
        self.write_transport_payload(
            info,
            IpNextHeaderProtocols::Udp,
            &self.buffer[..buf_size + payload.len()],
            vec![],
        );

        Ok(())
    }

    fn write_transport_payload(
        &mut self,
        info: &CapturePacket,
        protocol: IpNextHeaderProtocol,
        payload: &[u8],
        options: Vec<EnhancedPacketOption>,
    ) {
        let network_packet_size = self.encode_ip_packet(info, protocol, payload).unwrap().0;
        let ethertype = self.encode_ip_packet(info, protocol, payload).unwrap().1;
        let ethernet_packet_size = self.encode_ethernet_packet(info, ethertype, &self.buffer[..network_packet_size]).unwrap();

        let enhanced_packet_block = EnhancedPacketBlock {
            original_len: ethernet_packet_size as u32,
            data: self.buffer[..ethernet_packet_size].to_vec().into(),
            interface_id: 0,
            timestamp: self.state.start_time.elapsed(),
            options,
        };

        self.writer.write_block(&enhanced_packet_block.into_block());
    }

    fn encode_ethernet_packet(
        &mut self,
        info: &CapturePacket,
        ethertype: EtherType,
        payload: &[u8],
    ) -> Result<usize, std::io::Error> {
        let mut ethernet_packet = MutableEthernetPacket::new(&mut self.buffer).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create Ethernet packet",
            )
        })?;

        ethernet_packet.set_ethertype(ethertype);
        ethernet_packet.set_payload(payload);

        Ok(ethernet_packet.packet_size())
    }

    fn encode_ip_packet(
        &mut self,
        info: &CapturePacket,
        protocol: IpNextHeaderProtocol,
        payload: &[u8],
    ) -> Result<(usize, EtherType), std::io::Error> {
        match info.ip_addr() {
            (IpAddr::V4(_), IpAddr::V4(_)) => {
                let (source, destination) = info.ipvt_by_direction();

                let mut ip_packet = MutableIpv4Packet::new(&mut self.buffer)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to create IPv4 packet"))?;
                self.set_ipv4_packet_fields(&mut ip_packet, source, destination, payload, protocol);
                ip_packet.set_checksum(pnet_packet::ipv4::checksum(&ip_packet.to_immutable()));

                Ok((ip_packet.packet_size(), EtherTypes::Ipv4))
            }
            (IpAddr::V6(_), IpAddr::V6(_)) => {
                let (source, destination) = info.ipvt_by_direction();

                let mut ip_packet = MutableIpv6Packet::new(&mut self.buffer)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to create IPv6 packet"))?;
                self.set_ipv6_packet_fields(&mut ip_packet, source, destination, payload, protocol);

                Ok((ip_packet.packet_size(), EtherTypes::Ipv6))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported or mismatched IP address types",
            )),
        }
    }

    fn set_ipv4_packet_fields(
        &mut self,
        ip_packet: &mut MutableIpv4Packet,
        source: Ipv4Addr,
        destination: Ipv4Addr,
        payload: &[u8],
        protocol: IpNextHeaderProtocol,
    ) {
        ip_packet.set_version(4);
        ip_packet.set_header_length(5); // No options
        ip_packet.set_total_length((payload.len() + HEADER_SIZE_IP4) as u16);
        ip_packet.set_next_level_protocol(protocol);
        ip_packet.set_source(source);
        ip_packet.set_destination(destination);
        ip_packet.set_ttl(DEFAULT_TTL);
        ip_packet.set_payload(payload);
    }

    fn set_ipv6_packet_fields(
        &mut self,
        ip_packet: &mut MutableIpv6Packet,
        source: Ipv6Addr,
        destination: Ipv6Addr,
        payload: &[u8],
        protocol: IpNextHeaderProtocol,
    ) {
        ip_packet.set_version(6);
        ip_packet.set_payload_length(payload.len() as u16);
        ip_packet.set_next_header(protocol);
        ip_packet.set_source(source);
        ip_packet.set_destination(destination);
        ip_packet.set_hop_limit(DEFAULT_TTL);
        ip_packet.set_payload(payload);
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            send_seq: 0,
            rec_seq: 0,
            has_sent_handshake: false,
            has_sent_fin: false,
            stream_count: 0,
        }
    }
}
