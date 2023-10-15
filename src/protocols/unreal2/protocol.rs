use crate::buffer::{Buffer, StringDecoder};
use crate::errors::GDErrorKind::PacketBad;
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::utils::retry_on_timeout;
use crate::GDResult;

use super::{MutatorsAndRules, PacketKind, Players, Response, ServerInfo};

use std::net::SocketAddr;

use byteorder::{ByteOrder, LittleEndian};

// TODO: Validate this is the correct packet size
pub const PACKET_SIZE: usize = 5012;

/// The Unreal2 protocol implementation.
pub(crate) struct Unreal2Protocol {
    socket: UdpSocket,
    retry_count: usize,
}

impl Unreal2Protocol {
    pub fn new(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address)?;
        let retry_count = timeout_settings
            .as_ref()
            .map(|t| t.get_retries())
            .unwrap_or_else(|| TimeoutSettings::default().get_retries());
        socket.apply_timeout(&timeout_settings)?;

        Ok(Self {
            socket,
            retry_count,
        })
    }

    /// Send a request packet and recieve the first response (with retries).
    fn get_request_data(&mut self, packet_type: PacketKind) -> GDResult<Vec<u8>> {
        retry_on_timeout(self.retry_count, move || {
            self.get_request_data_impl(packet_type)
        })
    }

    /// Send a request packet
    fn get_request_data_impl(&mut self, packet_type: PacketKind) -> GDResult<Vec<u8>> {
        let request = [0x79, 0, 0, 0, packet_type as u8];
        self.socket.send(&request)?;

        let data = self.socket.receive(Some(PACKET_SIZE))?;

        Ok(data)
    }

    /// Consume the header part of a response packet, validate that the packet
    /// type matches what is expected.
    fn consume_response_headers<B: ByteOrder>(
        buffer: &mut Buffer<B>,
        expected_packet_type: PacketKind,
    ) -> GDResult<()> {
        // Skip header
        buffer.move_cursor(4)?;

        let packet_type: u8 = buffer.read()?;

        let packet_type: PacketKind = packet_type.try_into()?;

        if packet_type != expected_packet_type {
            Err(PacketBad.context(format!(
                "Packet response ({:?}) didn't match request ({:?}) packet type",
                packet_type, expected_packet_type
            )))
        } else {
            Ok(())
        }
    }

    /// Make a full server query.
    pub fn query(&mut self) -> GDResult<Response> {
        // Fetch the server info, this can only handle one response packet
        let server_info = {
            let data = self.get_request_data(PacketKind::ServerInfo)?;
            let mut buffer = Buffer::<LittleEndian>::new(&data);
            // TODO: Maybe put consume headers in individual packet parse methods
            Self::consume_response_headers(&mut buffer, PacketKind::ServerInfo)?;
            ServerInfo::parse(&mut buffer)?
        };

        // TODO: Remove debug logging
        println!("{:#?}", server_info);

        // Fetch mutators and rules, this is a required packet so we validate that we
        // get at least one response. However there can be many packets in
        // response to a single request so we greedily handle packets until
        // we get a timeout (or any receive error).
        let mut mutators_and_rules = MutatorsAndRules::default();
        {
            let data = self.get_request_data(PacketKind::MutatorsAndRules)?;
            let mut buffer = Buffer::<LittleEndian>::new(&data);
            // TODO: Maybe put consume headers in individual packet parse methods
            Self::consume_response_headers(&mut buffer, PacketKind::MutatorsAndRules)?;
            mutators_and_rules.parse(&mut buffer)?
        };

        // We could receive multiple packets in response
        while let Ok(data) = self.socket.receive(Some(PACKET_SIZE)) {
            let mut buffer = Buffer::<LittleEndian>::new(&data);

            let r = Self::consume_response_headers(&mut buffer, PacketKind::MutatorsAndRules);
            if r.is_err() {
                println!("{:?}", r);
                break;
            }

            mutators_and_rules.parse(&mut buffer)?;
        }

        // TODO: Remove debug logging
        println!("{:#?}", mutators_and_rules);

        // Pre-allocate the player arrays, but don't over allocate memory if the server
        // specifies an insane number of players.
        let mut players = Players::with_capacity(server_info.num_players.try_into().unwrap_or(10).min(50));

        // Fetch first players packet (with retries)
        let mut players_data = self.get_request_data(PacketKind::Players);
        // Players are non required so if we don't get any responses we continue to
        // return
        while let Ok(data) = players_data {
            let mut buffer = Buffer::<LittleEndian>::new(&data);

            Self::consume_response_headers(&mut buffer, PacketKind::Players)?;

            players.parse(&mut buffer)?;

            // Receive next packet
            players_data = self.socket.receive(Some(PACKET_SIZE));
        }

        // TODO: Handle extra info parsing when we detect certain game types (or maybe
        // include that in gather settings).

        Ok(Response {
            server_info,
            mutators_and_rules,
            players,
        })
    }
}

/// Unreal 2 string decoder
pub struct Unreal2StringDecoder;
impl StringDecoder for Unreal2StringDecoder {
    type Delimiter = [u8; 1];

    const DELIMITER: Self::Delimiter = [0x00];

    fn decode_string(data: &[u8], cursor: &mut usize, delimiter: Self::Delimiter) -> GDResult<String> {
        let mut ucs2 = false;
        let mut length: usize = (*data
            .first()
            .ok_or(PacketBad.context("Tried to decode string without length"))?)
        .into();

        let mut start = 0;

        // Check if it is a UCS-2 string
        if length >= 0x80 {
            ucs2 = true;

            length = (length & 0x7f) * 2;

            start += 1;

            // For UCS-2 strings, some unreal 2 games randomly insert an extra 0x01 here,
            // not included in the length. Skip it if present (hopefully this never happens
            // legitimately)
            if let Some(1) = data[start ..].first() {
                start += 1;
            }
        }

        // If UCS2 the first byte is the masked length of the string
        let string_data = if ucs2 {
            let string_data = &data[start .. start + length];
            if string_data.len() != length {
                return Err(PacketBad.context("Not enough data in buffer to read string"));
            }
            string_data
        } else {
            // Else the string is null-delimited latin1

            // TODO: Replace this with delimiter finder helper
            let position = data
            // Create an iterator over the data.
                .iter()
                // Find the position of the delimiter
                .position(|&b| b == delimiter.as_ref()[0])
                // If the delimiter is not found, use the whole data slice.
                .unwrap_or(data.len());

            length = position + 1;

            &data[1.min(position) .. position]
        };

        // FIXME: This should be latin1 or usc2
        let result = String::from_utf8_lossy(string_data);

        // Strip color encodings
        // TODO: Improve efficiency
        // TODO: There might be a nicer way to do this once string patterns are stable
        //       https://github.com/rust-lang/rust/issues/27721

        // After '0x1b' skip 3 characters (including the '0x1b')
        let mut char_skip = 0usize;
        let result: String = result
            .chars()
            .filter(|c: &char| {
                if '\x1b'.eq(c) {
                    char_skip = 4;
                    return false;
                }
                char_skip = char_skip.saturating_sub(1);
                char_skip == 0
            })
            .collect();

        // Remove all characters between 0x00 and 0x1a
        let result = result.replace(|c: char| c > '\x00' && c <= '\x1a', "");

        *cursor += start + length;

        // Strip delimiter that wasn't included in length
        Ok(result.trim_matches('\0').to_string())
    }
}

///  Make an unreal2 query.
pub fn query(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    let mut client = Unreal2Protocol::new(address, timeout_settings)?;

    client.query()
}

// TODO: Add tests
