use {
    crate::{
        core::Buffer,
        error::{diagnostic::FailureReason, PacketError, Report, Result},
    },

    error_stack::ResultExt,
};

// packet type is i32 but we know
// it will always be 0, 2, or 3
pub type PacketType = u8;

pub struct Packet {
    pub size: i32,
    pub id: i32,
    pub r#type: PacketType,
    pub body: Option<String>,
}

impl Packet {
    pub const FIELD_SIZE_LENGTH: u8 = 4;
    pub const FIELD_ID_LENGTH: u8 = 4;
    pub const FIELD_TYPE_LENGTH: u8 = 4;

    pub const BODY_DELIMITER: u8 = 0x00;
    pub const TAIL_TERMINATOR: u8 = 0x00;

    pub const PADDING: [u8; 2] = [Self::BODY_DELIMITER, Self::TAIL_TERMINATOR];

    pub const HEADER_LENGTH: u8 = Self::FIELD_ID_LENGTH + Self::FIELD_TYPE_LENGTH;
    pub const PADDING_LENGTH: u8 = Self::PADDING.len() as u8;

    pub const FIELD_SIZE_MAX: u16 = 0x1000;
    pub const FIELD_SIZE_MIN: u8 = Self::HEADER_LENGTH + Self::PADDING_LENGTH;

    pub const MAX_SIZE: u16 = Self::FIELD_SIZE_MAX + Self::FIELD_SIZE_LENGTH as u16;

    pub const TYPE_SERVERDATA_AUTH: PacketType = 3;
    pub const TYPE_SERVERDATA_AUTH_RESPONSE: PacketType = 2;
    pub const TYPE_SERVERDATA_EXECCOMMAND: PacketType = 2;
    pub const TYPE_SERVERDATA_RESPONSE_VALUE: PacketType = 0;

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

    // buffer is a internal interface so the function cannot be pub
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
