use crate::{GDError, GDResult};

pub enum Endianess {
    Little, Big
}

pub struct Bufferer {
    data: Vec<u8>,
    endianess: Endianess,
    data_position: usize
}

impl Bufferer {
    pub fn new(endianess: Endianess) -> Self {
        Bufferer::new_with_data(endianess, &[])
    }
    
    pub fn new_with_data(endianess: Endianess, data: &[u8]) -> Self {
        Bufferer {
            data: data.to_vec(),
            endianess,
            data_position: 0
        }
    }

    pub fn get_u8(&mut self) -> GDResult<u8> {
        if self.data.len() <= self.data_position {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an u8.".to_string()));
        }

        let value = self.data[self.data_position];
        self.data_position += 1;
        Ok(value)
    }

    pub fn get_u16(&mut self) -> GDResult<u16> {
        let source_data: [u8; 2] = (&self.data[self.data_position..self.data_position + 1]).try_into()
            .map_err(|_| GDError::PacketUnderflow("Unexpectedly short packet for getting an u16.".to_string()))?;

        let value = match self.endianess {
            Endianess::Little => u16::from_le_bytes(source_data),
            Endianess::Big => u16::from_be_bytes(source_data)
        };

        self.data_position += 2;
        Ok(value)
    }
    
    pub fn get_u32(&mut self) -> GDResult<u32> {
        let source_data: [u8; 4] = (&self.data[self.data_position..self.data_position + 3]).try_into()
            .map_err(|_| GDError::PacketUnderflow("Unexpectedly short packet for getting an u32.".to_string()))?;

        let value = match self.endianess {
            Endianess::Little => u32::from_le_bytes(source_data),
            Endianess::Big => u32::from_be_bytes(source_data)
        };

        self.data_position += 4;
        Ok(value)
    }

    pub fn get_f32(&mut self) -> GDResult<f32> {
        let source_data: [u8; 4] = (&self.data[self.data_position..self.data_position + 3]).try_into()
            .map_err(|_| GDError::PacketUnderflow("Unexpectedly short packet for getting an f32.".to_string()))?;

        let value = match self.endianess {
            Endianess::Little => f32::from_le_bytes(source_data),
            Endianess::Big => f32::from_be_bytes(source_data)
        };

        self.data_position += 4;
        Ok(value)
    }

    pub fn get_u64(&mut self) -> GDResult<u64> {
        let source_data: [u8; 8] = (&self.data[self.data_position..self.data_position + 7]).try_into()
            .map_err(|_| GDError::PacketUnderflow("Unexpectedly short packet for getting an u64.".to_string()))?;
        
        let value = match self.endianess {
            Endianess::Little => u64::from_le_bytes(source_data),
            Endianess::Big => u64::from_be_bytes(source_data)
        };

        self.data_position += 8;
        Ok(value)
    }

    pub fn get_string_utf8(&mut self) -> GDResult<String> {
        let sub_buf = &self.data[self.data_position..];
        if sub_buf.len() == 0 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an utf8 string.".to_string()));
        }

        let first_null_position = sub_buf.iter().position(|&x| x == 0)
            .ok_or(GDError::PacketBad("Unexpectedly formatted packet for getting an utf8 string.".to_string()))?;
        let value = std::str::from_utf8(&sub_buf[..first_null_position])
            .map_err(|_| GDError::PacketBad("Badly formatted utf8 string.".to_string()))?.to_string();

        self.data_position += value.len() + 1;
        Ok(value)
    }

    pub fn get_string_utf8_unended(&mut self) -> GDResult<String> {
        let sub_buf = &self.data[self.data_position..];
        if sub_buf.len() == 0 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an utf8 unended string.".to_string()));
        }

        let value = std::str::from_utf8(&sub_buf)
            .map_err(|_| GDError::PacketBad("Badly formatted utf8 unended string.".to_string()))?.to_string();

        self.data_position += value.len();
        Ok(value)
    }

    pub fn get_string_utf16(&mut self) -> GDResult<String> {
        let sub_buf = &self.data[self.data_position..];
        if sub_buf.len() == 0 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an utf16 string.".to_string()));
        }

        let paired_buf: Vec<u16> = sub_buf.chunks_exact(2)
            .into_iter().map(|a| match self.endianess {
            Endianess::Little => u16::from_le_bytes([a[0], a[1]]),
            Endianess::Big => u16::from_be_bytes([a[0], a[1]])
        }).collect();

        let value = String::from_utf16(&paired_buf)
            .map_err(|_| GDError::PacketBad("Badly formatted utf16 string.".to_string()))?.to_string();

        self.data_position += value.len() * 2;
        Ok(value)
    }
    
    pub fn move_position_ahead(&mut self, by: usize) {
        self.data_position += by;
    }

    pub fn move_position_backward(&mut self, by: usize) {
        self.data_position -= by;
    }
    
    pub fn get_data_in_front_of_position(&self) -> Vec<u8> {
        self.data[self.data_position..].to_vec()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_and_port_as_string_test() {
        assert_eq!(address_and_port_as_string("192.168.0.1", 27015), "192.168.0.1:27015");
    }

    #[test]
    fn u8_lower_upper_test() {
        assert_eq!(u8_lower_upper(171), (11, 10));
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
    fn get_string_utf8_le_unended_test() {
        let data = [72, 101, 108, 108, 111];
        let mut pos = 0;
        assert_eq!(buffer::get_string_utf8_le_unended(&data, &mut pos).unwrap(), "Hello");
        assert_eq!(pos, 5);
        assert!(buffer::get_string_utf8_le_unended(&data, &mut pos).is_err());
        assert_eq!(pos, 5);
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

 */
