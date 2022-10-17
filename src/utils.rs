use std::ops::Add;
use crate::GDError;

pub fn concat_u8_arrays(first: &[u8], second: &[u8]) -> Vec<u8> {
    [first, second].concat()
}

pub fn complete_address(address: &str, port: u16) -> String {
    String::from(address.to_owned() + ":").add(&*port.to_string())
}

pub mod buffer {
    use super::*;

    pub fn get_u8(buf: &[u8], pos: &mut usize) -> Result<u8, GDError> {
        if buf.len() <= *pos {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet.".to_string()));
        }

        let value = buf[*pos];
        *pos += 1;
        Ok(value)
    }

    pub fn get_u16_le(buf: &[u8], pos: &mut usize) -> Result<u16, GDError> {
        if buf.len() <= *pos + 1 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet.".to_string()));
        }

        let value = u16::from_le_bytes([buf[*pos], buf[*pos + 1]]);
        *pos += 2;
        Ok(value)
    }

    pub fn get_u32_le(buf: &[u8], pos: &mut usize) -> Result<u32, GDError> {
        if buf.len() <= *pos + 3 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet.".to_string()));
        }

        let value = u32::from_le_bytes([buf[*pos], buf[*pos + 1], buf[*pos + 2], buf[*pos + 3]]);
        *pos += 4;
        Ok(value)
    }

    pub fn get_f32_le(buf: &[u8], pos: &mut usize) -> Result<f32, GDError> {
        if buf.len() <= *pos + 3 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet.".to_string()));
        }

        let value = f32::from_le_bytes([buf[*pos], buf[*pos + 1], buf[*pos + 2], buf[*pos + 3]]);
        *pos += 4;
        Ok(value)
    }

    pub fn get_u64_le(buf: &[u8], pos: &mut usize) -> Result<u64, GDError> {
        if buf.len() <= *pos + 7 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet.".to_string()));
        }

        let value = u64::from_le_bytes([buf[*pos], buf[*pos + 1], buf[*pos + 2], buf[*pos + 3], buf[*pos + 4], buf[*pos + 5], buf[*pos + 6], buf[*pos + 7]]);
        *pos += 8;
        Ok(value)
    }

    pub fn get_string(buf: &[u8], pos: &mut usize) -> Result<String, GDError> {
        let sub_buf = &buf[*pos..];
        let first_null_position = sub_buf.iter().position(|&x| x == 0).ok_or(GDError::PacketBad("Unexpectedly formatted packet.".to_string()))?;
        let value = std::str::from_utf8(&sub_buf[..first_null_position]).unwrap().to_string();
        *pos += value.len() + 1;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_u8_arrays_test() {
        let a: [u8; 2] = [1, 2];
        let b: [u8; 2] = [3, 4];
        let combined = concat_u8_arrays(&a, &b);
        assert_eq!(combined[0], a[0]);
        assert_eq!(combined[1], a[1]);
        assert_eq!(combined[2], b[0]);
        assert_eq!(combined[3], b[1]);
    }

    #[test]
    fn complete_address_test() {
        let address = "192.168.0.1";
        let port = 27015;
        assert_eq!(complete_address(address, port), "192.168.0.1:27015");
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
    fn get_string_test() {
        let data = [72, 101, 108, 108, 111, 0, 72];
        let mut pos = 0;
        assert_eq!(buffer::get_string(&data, &mut pos).unwrap(), "Hello");
        assert_eq!(pos, 6);
        assert!(buffer::get_string(&data, &mut pos).is_err());
        assert_eq!(pos, 6);
    }
}
