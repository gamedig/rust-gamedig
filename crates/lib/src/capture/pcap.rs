use pcap_file::pcapng::{blocks::enhanced_packet::EnhancedPacketOption, PcapNgBlock, PcapNgWriter};
use pnet_packet::{
    ethernet::{EtherType, MutableEthernetPacket},
    ip::{IpNextHeaderProtocol, IpNextHeaderProtocols},
    ipv4::MutableIpv4Packet,
    ipv6::MutableIpv6Packet,
    tcp::{MutableTcpPacket, TcpFlags},
    udp::MutableUdpPacket,
    PacketSize,
};
use std::{io::Write, net::IpAddr, time::Instant};

use super::packet::{
    CapturePacket, Direction, Protocol, HEADER_SIZE_ETHERNET, HEADER_SIZE_IP4, HEADER_SIZE_IP6, HEADER_SIZE_UDP,
    PACKET_SIZE,
};

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
}

pub(crate) struct State {
    pub(crate) start_time: Instant,
    pub(crate) send_seq: u32,
    pub(crate) rec_seq: u32,
    pub(crate) has_sent_handshake: bool,
    pub(crate) stream_count: u32,
}

impl<W: Write> Pcap<W> {
    pub(crate) fn new(writer: PcapNgWriter<W>) -> Self {
        Self {
            writer,
            state: State::default(),
        }
    }

    pub(crate) fn write_transport_packet(&mut self, info: &CapturePacket, payload: &[u8]) {
        let mut buf = vec![0; BUFFER_SIZE];

        let (source_port, dest_port) = info.ports_by_direction();

        match info.protocol {
            Protocol::TCP => {
                let buf_size = {
                    let mut tcp = MutableTcpPacket::new(&mut buf).unwrap();
                    tcp.set_source(source_port);
                    tcp.set_destination(dest_port);
                    tcp.set_payload(payload);
                    tcp.set_data_offset(5);
                    tcp.set_window(43440);
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

                    tcp.packet_size()
                };

                self.write_transport_payload(
                    info,
                    IpNextHeaderProtocols::Tcp,
                    &buf[..buf_size + payload.len()],
                    vec![],
                );

                let mut info = info.clone();
                let buf_size = {
                    let mut tcp = MutableTcpPacket::new(&mut buf).unwrap();
                    tcp.set_source(dest_port);
                    tcp.set_destination(source_port);
                    tcp.set_data_offset(5);
                    tcp.set_window(43440);
                    match &info.direction {
                        Direction::Send => {
                            tcp.set_sequence(self.state.rec_seq);
                            tcp.set_acknowledgement(self.state.send_seq);

                            info.direction = Direction::Receive;
                        }
                        Direction::Receive => {
                            tcp.set_sequence(self.state.send_seq);
                            tcp.set_acknowledgement(self.state.rec_seq);

                            info.direction = Direction::Send;
                        }
                    }
                    tcp.set_flags(TcpFlags::ACK);

                    tcp.packet_size()
                };

                self.write_transport_payload(
                    &info,
                    IpNextHeaderProtocols::Tcp,
                    &buf[..buf_size],
                    vec![EnhancedPacketOption::Comment("Generated TCP ack".into())],
                );
            }
            Protocol::UDP => {
                let buf_size = {
                    let mut udp = MutableUdpPacket::new(&mut buf).unwrap();
                    udp.set_source(source_port);
                    udp.set_destination(dest_port);
                    udp.set_length((payload.len() + HEADER_SIZE_UDP) as u16);
                    udp.set_payload(payload);

                    udp.packet_size()
                };

                self.write_transport_payload(
                    info,
                    IpNextHeaderProtocols::Udp,
                    &buf[..buf_size + payload.len()],
                    vec![],
                );
            }
        }
    }

    /// Encode a network layer (IP) packet with a payload.
    fn encode_ip_packet(
        &self,
        buf: &mut [u8],
        info: &CapturePacket,
        protocol: IpNextHeaderProtocol,
        payload: &[u8],
    ) -> (usize, EtherType) {
        match info.ip_addr() {
            (IpAddr::V4(_), IpAddr::V4(_)) => {
                let (source, destination) = info.ipvt_by_direction();

                let header_size = HEADER_SIZE_IP4 + (32 / 8);

                let mut ip = MutableIpv4Packet::new(buf).unwrap();
                ip.set_version(4);
                ip.set_total_length((payload.len() + header_size) as u16);
                ip.set_next_level_protocol(protocol);
                // https://en.wikipedia.org/wiki/Internet_Protocol_version_4#Total_Length

                ip.set_header_length((header_size / 4) as u8);
                ip.set_source(source);
                ip.set_destination(destination);
                ip.set_payload(payload);
                ip.set_ttl(64);
                ip.set_flags(pnet_packet::ipv4::Ipv4Flags::DontFragment);

                let mut options_writer =
                    pnet_packet::ipv4::MutableIpv4OptionPacket::new(ip.get_options_raw_mut()).unwrap();
                options_writer.set_copied(1);
                options_writer.set_class(0);
                options_writer.set_number(pnet_packet::ipv4::Ipv4OptionNumbers::SID);
                options_writer.set_length(&[4]);
                options_writer.set_data(&(self.state.stream_count as u16).to_be_bytes());

                ip.set_checksum(pnet_packet::ipv4::checksum(&ip.to_immutable()));

                (ip.packet_size(), pnet_packet::ethernet::EtherTypes::Ipv4)
            }
            (IpAddr::V6(_), IpAddr::V6(_)) => {
                let (source, destination) = info.ipvt_by_direction();

                let mut ip = MutableIpv6Packet::new(buf).unwrap();
                ip.set_version(6);
                ip.set_payload_length(payload.len() as u16);
                ip.set_next_header(protocol);
                ip.set_source(source);
                ip.set_destination(destination);
                ip.set_hop_limit(64);
                ip.set_payload(payload);
                ip.set_flow_label(self.state.stream_count);

                (ip.packet_size(), pnet_packet::ethernet::EtherTypes::Ipv6)
            }
            _ => unreachable!(),
        }
    }

    /// Encode a physical layer (ethernet) packet with a payload.
    fn encode_ethernet_packet(
        &self,
        buf: &mut [u8],
        ethertype: pnet_packet::ethernet::EtherType,
        payload: &[u8],
    ) -> usize {
        let mut ethernet = MutableEthernetPacket::new(buf).unwrap();
        ethernet.set_ethertype(ethertype);
        ethernet.set_payload(payload);

        ethernet.packet_size()
    }

    /// Write a TCP handshake.
    pub(crate) fn write_tcp_handshake(&mut self, info: &CapturePacket) {
        let (source_port, dest_port) = (info.local_address.port(), info.remote_address.port());

        let mut info = info.clone();
        info.direction = Direction::Send;
        let mut buf = vec![0; PACKET_SIZE];
        // Add a generated comment to all packets
        let options = vec![
            pcap_file::pcapng::blocks::enhanced_packet::EnhancedPacketOption::Comment("Generated TCP handshake".into()),
        ];

        // SYN
        let buf_size = {
            let mut tcp = MutableTcpPacket::new(&mut buf).unwrap();
            self.state.send_seq = 500;
            tcp.set_sequence(self.state.send_seq);
            tcp.set_flags(TcpFlags::SYN);
            tcp.set_source(source_port);
            tcp.set_destination(dest_port);
            tcp.set_window(43440);
            tcp.set_data_offset(5);

            tcp.packet_size()
        };
        self.write_transport_payload(
            &info,
            IpNextHeaderProtocols::Tcp,
            &buf[..buf_size],
            options.clone(),
        );

        // SYN + ACK
        info.direction = Direction::Receive;
        let buf_size = {
            let mut tcp = MutableTcpPacket::new(&mut buf).unwrap();
            self.state.send_seq = self.state.send_seq.wrapping_add(1);
            tcp.set_acknowledgement(self.state.send_seq);
            self.state.rec_seq = 1000;
            tcp.set_sequence(self.state.rec_seq);
            tcp.set_flags(TcpFlags::SYN | TcpFlags::ACK);
            tcp.set_source(dest_port);
            tcp.set_destination(source_port);
            tcp.set_window(43440);
            tcp.set_data_offset(5);

            tcp.packet_size()
        };
        self.write_transport_payload(
            &info,
            IpNextHeaderProtocols::Tcp,
            &buf[..buf_size],
            options.clone(),
        );

        // ACK
        info.direction = Direction::Send;
        let buf_size = {
            let mut tcp = MutableTcpPacket::new(&mut buf).unwrap();
            tcp.set_sequence(self.state.send_seq);
            self.state.rec_seq = self.state.rec_seq.wrapping_add(1);
            tcp.set_acknowledgement(self.state.rec_seq);
            tcp.set_flags(TcpFlags::ACK);
            tcp.set_source(source_port);
            tcp.set_destination(dest_port);
            tcp.set_window(43440);
            tcp.set_data_offset(5);

            tcp.packet_size()
        };
        self.write_transport_payload(&info, IpNextHeaderProtocols::Tcp, &buf[..buf_size], options);

        self.state.has_sent_handshake = true;
    }

    /// Take a transport layer packet as a buffer and write it after encoding
    /// all the layers under it.
    fn write_transport_payload(
        &mut self,
        info: &CapturePacket,
        protocol: IpNextHeaderProtocol,
        payload: &[u8],
        options: Vec<pcap_file::pcapng::blocks::enhanced_packet::EnhancedPacketOption>,
    ) {
        let mut network_packet = vec![0; PACKET_SIZE - HEADER_SIZE_ETHERNET];
        let (network_size, ethertype) = self.encode_ip_packet(&mut network_packet, info, protocol, payload);
        let network_size = network_size + payload.len();
        network_packet.truncate(network_size);

        let mut physical_packet = vec![0; PACKET_SIZE];
        let physical_size =
            self.encode_ethernet_packet(&mut physical_packet, ethertype, &network_packet) + network_size;

        physical_packet.truncate(physical_size);

        self.writer
            .write_block(
                &pcap_file::pcapng::blocks::enhanced_packet::EnhancedPacketBlock {
                    original_len: physical_size as u32,
                    data: physical_packet.into(),
                    interface_id: 0,
                    timestamp: self.state.start_time.elapsed(),
                    options,
                }
                .into_block(),
            )
            .unwrap();
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            send_seq: 0,
            rec_seq: 0,
            has_sent_handshake: false,
            stream_count: 0,
        }
    }
}
