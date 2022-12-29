use crate::{GDError, GDResult};

pub enum Endianess {
    Little, Big
}

pub struct Bufferer {
    data: Vec<u8>,
    endianess: Endianess,
    position: usize
}

impl Bufferer {
    pub fn new(endianess: Endianess) -> Self {
        Bufferer::new_with_data(endianess, &[])
    }
    
    pub fn new_with_data(endianess: Endianess, data: &[u8]) -> Self {
        Bufferer {
            data: data.to_vec(),
            endianess,
            position: 0
        }
    }

    fn check_size(&self, by: usize) -> bool {
        by > self.remaining_length()
    }

    pub fn get_u8(&mut self) -> GDResult<u8> {
        if self.check_size(1) {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an u8.".to_string()));
        }

        let value = self.data[self.position];
        self.position += 1;
        Ok(value)
    }

    pub fn get_u16(&mut self) -> GDResult<u16> {
        if self.check_size(2) {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an u16.".to_string()));
        }

        let source_data: [u8; 2] = (&self.data[self.position..self.position + 2]).try_into().unwrap();

        let value = match self.endianess {
            Endianess::Little => u16::from_le_bytes(source_data),
            Endianess::Big => u16::from_be_bytes(source_data)
        };

        self.position += 2;
        Ok(value)
    }
    
    pub fn get_u32(&mut self) -> GDResult<u32> {
        if self.check_size(4) {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an u32.".to_string()));
        }

        let source_data: [u8; 4] = (&self.data[self.position..self.position + 4]).try_into().unwrap();

        let value = match self.endianess {
            Endianess::Little => u32::from_le_bytes(source_data),
            Endianess::Big => u32::from_be_bytes(source_data)
        };

        self.position += 4;
        Ok(value)
    }

    pub fn get_f32(&mut self) -> GDResult<f32> {
        if self.check_size(4) {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an f32.".to_string()));
        }

        let source_data: [u8; 4] = (&self.data[self.position..self.position + 4]).try_into().unwrap();

        let value = match self.endianess {
            Endianess::Little => f32::from_le_bytes(source_data),
            Endianess::Big => f32::from_be_bytes(source_data)
        };

        self.position += 4;
        Ok(value)
    }

    pub fn get_u64(&mut self) -> GDResult<u64> {
        if self.check_size(8) {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an u64.".to_string()));
        }

        let source_data: [u8; 8] = (&self.data[self.position..self.position + 8]).try_into().unwrap();
        
        let value = match self.endianess {
            Endianess::Little => u64::from_le_bytes(source_data),
            Endianess::Big => u64::from_be_bytes(source_data)
        };

        self.position += 8;
        Ok(value)
    }

    pub fn get_string_utf8(&mut self) -> GDResult<String> {
        let sub_buf = &self.data[self.position..];
        if sub_buf.len() == 0 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an utf8 string.".to_string()));
        }

        let first_null_position = sub_buf.iter().position(|&x| x == 0)
            .ok_or(GDError::PacketBad("Unexpectedly formatted packet for getting an utf8 string.".to_string()))?;
        let value = std::str::from_utf8(&sub_buf[..first_null_position])
            .map_err(|_| GDError::PacketBad("Badly formatted utf8 string.".to_string()))?.to_string();

        self.position += value.len() + 1;
        Ok(value)
    }

    pub fn get_string_utf8_unended(&mut self) -> GDResult<String> {
        let sub_buf = &self.data[self.position..];
        if sub_buf.len() == 0 {
            return Err(GDError::PacketUnderflow("Unexpectedly short packet for getting an utf8 unended string.".to_string()));
        }

        let value = std::str::from_utf8(&sub_buf)
            .map_err(|_| GDError::PacketBad("Badly formatted utf8 unended string.".to_string()))?.to_string();

        self.position += value.len();
        Ok(value)
    }

    pub fn get_string_utf16(&mut self) -> GDResult<String> {
        let sub_buf = &self.data[self.position..];
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

        self.position += value.len() * 2;
        Ok(value)
    }
    
    pub fn move_position_ahead(&mut self, by: usize) {
        self.position += by;
    }

    pub fn move_position_backward(&mut self, by: usize) {
        self.position -= by;
    }
    
    pub fn get_data_in_front_of_position(&self) -> Vec<u8> {
        self.data[self.position..].to_vec()
    }

    pub fn data_length(&self) -> usize {
        self.data.len()
    }

    pub fn remaining_data(&self) -> &[u8] {
        &self.data[self.position..]
    }

    pub fn remaining_length(&self) -> usize {
        self.data.len() - self.position
    }

    pub fn as_endianess(&self, endianess: Endianess) -> Self {
        Bufferer {
            data: self.data.clone(),
            endianess,
            position: self.position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_u8() {
        let mut buffer = Bufferer::new_with_data(Endianess::Little, &[72]);

        assert_eq!(buffer.get_u8().unwrap(), 72);
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_u8().is_err());
    }

    #[test]
    fn get_u16_le() {
        let mut buffer = Bufferer::new_with_data(Endianess::Little, &[72, 79]);

        assert_eq!(buffer.get_u16().unwrap(), 20296);
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_u16().is_err());
    }

    #[test]
    fn get_u16_be() {
        let mut buffer = Bufferer::new_with_data(Endianess::Big, &[29, 72]);

        assert_eq!(buffer.get_u16().unwrap(), 7496);
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_u16().is_err());
    }

    #[test]
    fn get_u32_le() {
        let mut buffer = Bufferer::new_with_data(Endianess::Little, &[72, 29, 128, 100]);

        assert_eq!(buffer.get_u32().unwrap(), 1686117704);
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_u32().is_err());
    }

    #[test]
    fn get_u32_be() {
        let mut buffer = Bufferer::new_with_data(Endianess::Big, &[72, 29, 128, 100]);

        assert_eq!(buffer.get_u32().unwrap(), 1209892964);
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_u32().is_err());
    }

    #[test]
    fn get_f32_le() {
        let mut buffer = Bufferer::new_with_data(Endianess::Little, &[72, 29, 128, 100]);

        assert_eq!(buffer.get_f32().unwrap(), 1.8906345e22);
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_f32().is_err());
    }

    #[test]
    fn get_f32_be() {
        let mut buffer = Bufferer::new_with_data(Endianess::Big, &[72, 29, 128, 100]);

        assert_eq!(buffer.get_f32().unwrap(), 161281.56);
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_f32().is_err());
    }

    #[test]
    fn get_u64_le() {
        let mut buffer = Bufferer::new_with_data(Endianess::Little, &[72, 29, 128, 99, 69, 4, 2, 0]);

        assert_eq!(buffer.get_u64().unwrap(), 567646022016328);
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_u64().is_err());
    }

    #[test]
    fn get_u64_be() {
        let mut buffer = Bufferer::new_with_data(Endianess::Big, &[72, 29, 128, 99, 69, 4, 2, 0]);

        assert_eq!(buffer.get_u64().unwrap(), 5196450708903428608);
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_u64().is_err());
    }

    #[test]
    fn get_string_utf8() {
        let mut buffer = Bufferer::new_with_data(Endianess::Little, &[72, 101, 108, 108, 111, 0, 72]);

        assert_eq!(buffer.get_string_utf8().unwrap(), "Hello");
        assert_eq!(buffer.remaining_length(), 1);
        assert!(buffer.get_string_utf8().is_err());
    }

    #[test]
    fn get_string_utf8_unended() {
        let mut buffer = Bufferer::new_with_data(Endianess::Little, &[72, 101, 108, 108, 111]);

        assert_eq!(buffer.get_string_utf8_unended().unwrap(), "Hello");
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_string_utf8_unended().is_err());
    }

    #[test]
    fn get_string_utf16_le() {
        let mut buffer = Bufferer::new_with_data(Endianess::Little, &[0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00]);

        assert_eq!(buffer.get_string_utf16().unwrap(), "Hello");
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_string_utf16().is_err());
    }

    #[test]
    fn get_string_utf16_be() {
        let mut buffer = Bufferer::new_with_data(Endianess::Big, &[0x00, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f]);

        assert_eq!(buffer.get_string_utf16().unwrap(), "Hello");
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_string_utf16().is_err());
    }
}
