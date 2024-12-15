use {
    crate::{
        core::Buffer,
        error::{diagnostic::FailureReason, PacketError, Report, Result},
    },

    error_stack::ResultExt,
};

/// Represents the packet type in the Valve RCON protocol.
///
/// The packet type is stored as a `u8` but corresponds to specific integer values:
/// - `0`: `SERVERDATA_RESPONSE_VALUE`
/// - `2`: `SERVERDATA_AUTH_RESPONSE`
/// - `2`: `SERVERDATA_EXECCOMMAND`
/// - `3`: `SERVERDATA_AUTH`
///
/// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Packet_Type
pub type PacketType = u8;

/// Represents a packet in the Valve RCON protocol.
#[derive(Debug)]
pub struct Packet {
    /// The size of the packet (does not include the size field itself).
    ///
    /// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Packet_Size
    pub size: i32,
    /// The ID of the packet (cannot be negative).
    ///
    /// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Packet_ID
    pub id: i32,
    /// The type of the packet.
    ///
    /// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Packet_Type
    pub r#type: PacketType,
    /// The body of the packet.
    ///
    /// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Packet_Body
    pub body: Option<String>,
}

impl Packet {
    /// The length of the size field in bytes.
    pub const FIELD_SIZE_LENGTH: u8 = 4;
    /// The length of the ID field in bytes.
    pub const FIELD_ID_LENGTH: u8 = 4;
    /// The length of the type field in bytes.
    pub const FIELD_TYPE_LENGTH: u8 = 4;

    /// The delimiter used to end the body of the packet.
    pub const BODY_DELIMITER: u8 = 0x00;
    /// The terminator used to end the packet.
    pub const TAIL_TERMINATOR: u8 = 0x00;

    /// Padding added at the end of the packet.
    ///
    /// `[Self::BODY_DELIMITER, Self::TAIL_TERMINATOR]`
    pub const PADDING: [u8; 2] = [Self::BODY_DELIMITER, Self::TAIL_TERMINATOR];

    /// The length of the header in bytes (Excludes the `size` field).
    pub const HEADER_LENGTH: u8 = Self::FIELD_ID_LENGTH + Self::FIELD_TYPE_LENGTH;
    /// The length of the padding in bytes.
    pub const PADDING_LENGTH: u8 = Self::PADDING.len() as u8;

    /// The maximum size of the `size` field.
    pub const FIELD_SIZE_MAX: u16 = 0x1000;
    /// The minimum size of the `size` field.
    pub const FIELD_SIZE_MIN: u8 = Self::HEADER_LENGTH + Self::PADDING_LENGTH;

    /// The maximum size of the packet.
    pub const MAX_SIZE: u16 = Self::FIELD_SIZE_MAX + Self::FIELD_SIZE_LENGTH as u16;

    /// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#SERVERDATA_AUTH
    pub const TYPE_SERVERDATA_AUTH: PacketType = 3;
    /// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#SERVERDATA_AUTH_RESPONSE
    pub const TYPE_SERVERDATA_AUTH_RESPONSE: PacketType = 2;
    /// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#SERVERDATA_EXECCOMMAND
    pub const TYPE_SERVERDATA_EXECCOMMAND: PacketType = 2;
    /// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#SERVERDATA_RESPONSE_VALUE
    pub const TYPE_SERVERDATA_RESPONSE_VALUE: PacketType = 0;

    /// Creates a new `Packet` with the specified type, ID, and optional body.
    ///
    /// # Parameters
    /// - `r#type`: The packet type.
    /// - `id`: The packet ID.
    /// - `body`: An optional string containing the body of the packet.
    pub fn new(r#type: PacketType, id: i32, body: Option<String>) -> Self {
        Self {
            size: match &body {
                Some(body) => body.len() as i32 + Self::FIELD_SIZE_MIN as i32,

                None => Self::FIELD_SIZE_MIN as i32,
            },
            r#type,
            id,
            body,
        }
    }

    /// Serializes the `Packet` into a vector of bytes suitable for transmission.
    ///
    /// # Returns
    /// A `Vec<u8>` containing the serialized packet data.
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity((self.size + Self::FIELD_SIZE_LENGTH as i32) as usize);

        buf.extend(&self.size.to_le_bytes());
        buf.extend(&self.id.to_le_bytes());
        buf.extend(&(self.r#type as i32).to_le_bytes());

        if let Some(body) = &self.body {
            buf.extend(body.as_bytes());
        }

        buf.extend(&Self::PADDING);

        buf
    }

    /// Deserializes a `Packet` from the provided `Buffer`.
    ///
    /// This function reads the buffer and constructs a `Packet` by extracting the
    /// size, ID, type, and body fields in that order, following the Valve RCON packet structure.
    ///
    /// # Parameters
    /// - `b`: A mutable reference to a `Buffer` containing the packet data.
    #[allow(dead_code)]
    pub(crate) fn deserialize(b: &mut Buffer) -> Result<Self> {
        let size = b
            .read_i32_le()
            .map_err(|e| {
                Report::from(e).change_context(PacketError::PacketDeserializeError {}.into())
            })
            .attach_printable(FailureReason::new(
                "Failed to deserialize size field of packet.",
            ))?;

        let id = b
            .read_i32_le()
            .map_err(|e| {
                Report::from(e).change_context(PacketError::PacketDeserializeError {}.into())
            })
            .attach_printable(FailureReason::new(
                "Failed to deserialize id field of packet.",
            ))?;

        let r#type = b
            .read_i32_le()
            .map_err(|e| {
                Report::from(e).change_context(PacketError::PacketDeserializeError {}.into())
            })
            .attach_printable(FailureReason::new(
                "Failed to deserialize type field of packet.",
            ))? as PacketType;

        let body = match b.peek(1)?[0] {
            Self::BODY_DELIMITER => {
                b.move_pos(Self::PADDING_LENGTH as isize)?;

                None
            }

            _ => {
                let body = b.read_string_utf8(Some([Self::BODY_DELIMITER]), false)?;

                // skip tail
                b.move_pos(1)?;

                Some(body)
            }
        };

        Ok(Self {
            size,
            id,
            r#type,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Buffer;

    #[test]
    fn test_packet_creation_with_body() {
        let packet = Packet::new(
            Packet::TYPE_SERVERDATA_EXECCOMMAND,
            1,
            Some("test_command".into()),
        );

        assert_eq!(packet.size, 22);
        assert_eq!(packet.id, 1);
        assert_eq!(packet.r#type, Packet::TYPE_SERVERDATA_EXECCOMMAND);
        assert_eq!(packet.body, Some("test_command".into()));
    }

    #[test]
    fn test_packet_creation_without_body() {
        let packet = Packet::new(Packet::TYPE_SERVERDATA_AUTH, 2, None);

        assert_eq!(packet.size, Packet::FIELD_SIZE_MIN as i32);
        assert_eq!(packet.id, 2);
        assert_eq!(packet.r#type, Packet::TYPE_SERVERDATA_AUTH);
        assert_eq!(packet.body, None);
    }

    #[test]
    fn test_packet_serialization_with_body() {
        let packet = Packet::new(
            Packet::TYPE_SERVERDATA_RESPONSE_VALUE,
            42,
            Some("response".into()),
        );

        let serialized = packet.serialize();

        let expected_size = (packet.size as i32).to_le_bytes();
        let expected_id = 42i32.to_le_bytes();
        let expected_type = (Packet::TYPE_SERVERDATA_RESPONSE_VALUE as i32).to_le_bytes();

        let mut expected = Vec::new();
        expected.extend(&expected_size);
        expected.extend(&expected_id);
        expected.extend(&expected_type);
        expected.extend(b"response");
        expected.extend(&Packet::PADDING);

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_packet_serialization_without_body() {
        let packet = Packet::new(Packet::TYPE_SERVERDATA_AUTH_RESPONSE, 99, None);
        let serialized = packet.serialize();

        let expected_size = (packet.size as i32).to_le_bytes();
        let expected_id = 99i32.to_le_bytes();
        let expected_type = (Packet::TYPE_SERVERDATA_AUTH_RESPONSE as i32).to_le_bytes();

        let mut expected = Vec::new();
        expected.extend(&expected_size);
        expected.extend(&expected_id);
        expected.extend(&expected_type);
        expected.extend(&Packet::PADDING);

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_packet_deserialization_with_body() {
        let mut buffer = Buffer::new(vec![
            18, 0, 0, 0, // size (18)
            7, 0, 0, 0, // id (7)
            0, 0, 0, 0, // type (0)
            b'r', b'e', b's', b'p', b'o', b'n', b's', b'e', // body ("response")
            0x00, 0x00, // padding
        ]);

        let packet = Packet::deserialize(&mut buffer).unwrap();

        assert_eq!(packet.size, 18);
        assert_eq!(packet.id, 7);
        assert_eq!(packet.r#type, Packet::TYPE_SERVERDATA_RESPONSE_VALUE);
        assert_eq!(packet.body, Some("response".into()));
    }

    #[test]
    fn test_packet_deserialization_without_body() {
        let mut buffer = Buffer::new(vec![
            10, 0, 0, 0, // size (10)
            42, 0, 0, 0, // id (42)
            3, 0, 0, 0, // type (3)
            0x00, 0x00, // padding
        ]);

        let packet = Packet::deserialize(&mut buffer).unwrap();

        assert_eq!(packet.size, 10);
        assert_eq!(packet.id, 42);
        assert_eq!(packet.r#type, Packet::TYPE_SERVERDATA_AUTH);
        assert_eq!(packet.body, None);
    }
}
