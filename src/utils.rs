use crate::{GDResult, GDError};

pub fn error_by_expected_size(expected: usize, size: usize) -> GDResult<()> {
    if size < expected {
        Err(GDError::PacketUnderflow("Unexpectedly short packet.".to_string()))
    }
    else if size > expected {
        Err(GDError::PacketOverflow("Unexpectedly long packet.".to_string()))
    }
    else {
        Ok(())
    }
}

pub fn address_and_port_as_string(address: &str, port: u16) -> String {
    address.to_string() + ":" + &*port.to_string()
}

pub fn u8_lower_upper(n: u8) -> (u8, u8) {
    (n & 15, n >> 4)
}

pub mod buffer {
    use super::*;

    pub fn get_u8(buf: &[u8], pos: &mut usize) -> GDResult<u8> {
        if buf.len() <= *pos {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an u8.".to_string()));
        }

        let value = buf[*pos];
        *pos += 1;
        Ok(value)
    }

    pub fn get_u16_le(buf: &[u8], pos: &mut usize) -> GDResult<u16> {
        if buf.len() <= *pos + 1 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an u16.".to_string()));
        }

        let value = u16::from_le_bytes([buf[*pos], buf[*pos + 1]]);
        *pos += 2;
        Ok(value)
    }

    pub fn get_u16_be(buf: &[u8], pos: &mut usize) -> GDResult<u16> {
        if buf.len() <= *pos + 1 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an u16.".to_string()));
        }

        let value = u16::from_be_bytes([buf[*pos], buf[*pos + 1]]);
        *pos += 2;
        Ok(value)
    }

    pub fn get_u32_le(buf: &[u8], pos: &mut usize) -> GDResult<u32> {
        if buf.len() <= *pos + 3 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an u32.".to_string()));
        }

        let value = u32::from_le_bytes([buf[*pos], buf[*pos + 1], buf[*pos + 2], buf[*pos + 3]]);
        *pos += 4;
        Ok(value)
    }

    pub fn get_f32_le(buf: &[u8], pos: &mut usize) -> GDResult<f32> {
        if buf.len() <= *pos + 3 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an f32.".to_string()));
        }

        let value = f32::from_le_bytes([buf[*pos], buf[*pos + 1], buf[*pos + 2], buf[*pos + 3]]);
        *pos += 4;
        Ok(value)
    }

    pub fn get_u64_le(buf: &[u8], pos: &mut usize) -> GDResult<u64> {
        if buf.len() <= *pos + 7 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an u64.".to_string()));
        }

        let value = u64::from_le_bytes([buf[*pos], buf[*pos + 1], buf[*pos + 2], buf[*pos + 3], buf[*pos + 4], buf[*pos + 5], buf[*pos + 6], buf[*pos + 7]]);
        *pos += 8;
        Ok(value)
    }

    pub fn get_string_utf8_le(buf: &[u8], pos: &mut usize) -> GDResult<String> {
        let sub_buf = &buf[*pos..];
        if sub_buf.len() == 0 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an utf8 LE string.".to_string()));
        }

        let first_null_position = sub_buf.iter().position(|&x| x == 0)
            .ok_or(GDError::PacketBad("Unexpectedly formatted packet for getting a utf8 LE string.".to_string()))?;
        let value = std::str::from_utf8(&sub_buf[..first_null_position])
            .map_err(|_| GDError::PacketBad("Badly formatted utf8 LE string.".to_string()))?.to_string();

        *pos += value.len() + 1;
        Ok(value)
    }

    pub fn get_string_utf16_be(buf: &[u8], pos: &mut usize) -> GDResult<String> {
        let sub_buf = &buf[*pos..];
        if sub_buf.len() == 0 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an utf16 BE string.".to_string()));
        }

        let paired_buf: Vec<u16> = sub_buf.chunks_exact(2)
            .into_iter().map(|a| u16::from_be_bytes([a[0], a[1]])).collect();

        let value = String::from_utf16(&paired_buf)
            .map_err(|_| GDError::PacketBad("Badly formatted utf16 BE string.".to_string()))?.to_string();

        *pos += value.len() * 2;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_and_port_as_string_test() {
        assert_eq!(address_and_port_as_string("192.168.0.1", 27015), "192.168.0.1:27015");
    }

    #[test]
    fn get_u8_test() {
        let data = [72];
        let mut pos = 0;
        assert_eq!(buffer::get_u8(&data, &mut pos).unwrap(), 72);
        assert_eq!(pos, 1);
        assert!(buffer::get_u8(&data, &mut pos).is_err());
        assert_eq!(pos, 1);
    }

    #[test]
    fn get_u16_le_test() {
        let data = [72, 29];
        let mut pos = 0;
        assert_eq!(buffer::get_u16_le(&data, &mut pos).unwrap(), 7496);
        assert_eq!(pos, 2);
        assert!(buffer::get_u16_le(&data, &mut pos).is_err());
        assert_eq!(pos, 2);
    }

    #[test]
    fn get_u16_be_test() {
        let data = [29, 72];
        let mut pos = 0;
        assert_eq!(buffer::get_u16_be(&data, &mut pos).unwrap(), 7496);
        assert_eq!(pos, 2);
        assert!(buffer::get_u16_be(&data, &mut pos).is_err());
        assert_eq!(pos, 2);
    }

    #[test]
    fn get_u32_le_test() {
        let data = [72, 29, 128, 100];
        let mut pos = 0;
        assert_eq!(buffer::get_u32_le(&data, &mut pos).unwrap(), 1686117704);
        assert_eq!(pos, 4);
        assert!(buffer::get_u32_le(&data, &mut pos).is_err());
        assert_eq!(pos, 4);
    }

    #[test]
    fn get_f32_le_test() {
        let data = [72, 29, 128, 100];
        let mut pos = 0;
        assert_eq!(buffer::get_f32_le(&data, &mut pos).unwrap(), 1.8906345e22);
        assert_eq!(pos, 4);
        assert!(buffer::get_f32_le(&data, &mut pos).is_err());
        assert_eq!(pos, 4);
    }

    #[test]
    fn get_u64_le_test() {
        let data = [72, 29, 128, 99, 69, 4, 2, 0];
        let mut pos = 0;
        assert_eq!(buffer::get_u64_le(&data, &mut pos).unwrap(), 567646022016328);
        assert_eq!(pos, 8);
        assert!(buffer::get_u64_le(&data, &mut pos).is_err());
        assert_eq!(pos, 8);
    }

    #[test]
    fn get_string_utf8_le_test() {
        let data = [72, 101, 108, 108, 111, 0, 72];
        let mut pos = 0;
        assert_eq!(buffer::get_string_utf8_le(&data, &mut pos).unwrap(), "Hello");
        assert_eq!(pos, 6);
        assert!(buffer::get_string_utf8_le(&data, &mut pos).is_err());
        assert_eq!(pos, 6);
    }

    #[test]
    fn get_string_utf16_be_test() {
        let data = [0x00, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f];
        let mut pos = 0;
        assert_eq!(buffer::get_string_utf16_be(&data, &mut pos).unwrap(), "Hello");
        assert_eq!(pos, 10);
        assert!(buffer::get_string_utf16_be(&data, &mut pos).is_err());
        assert_eq!(pos, 10);
    }

    #[test]
    fn error_by_expected_size_test() {
        assert!(error_by_expected_size(69, 69).is_ok());
        assert!(error_by_expected_size(69, 68).is_err());
        assert!(error_by_expected_size(69, 70).is_err());
    }
}
