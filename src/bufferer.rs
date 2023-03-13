use crate::{
    GDError::{PacketBad, PacketUnderflow},
    GDResult,
};

use byteorder::{BigEndian, ByteOrder, LittleEndian};

pub enum Endianess {
    Little,
    Big,
}

pub struct Bufferer {
    data: Vec<u8>,
    endianess: Endianess,
    position: usize,
}

impl Bufferer {
    pub fn new_with_data(endianess: Endianess, data: &[u8]) -> Self {
        Bufferer {
            data: data.to_vec(),
            endianess,
            position: 0,
        }
    }

    fn check_size(&self, by: usize) -> bool { by > self.remaining_length() }

    pub fn get_u8(&mut self) -> GDResult<u8> {
        if self.check_size(1) {
            return Err(PacketUnderflow);
        }

        let value = self.data[self.position];
        self.move_position_ahead(1);
        Ok(value)
    }

    pub fn get_u16(&mut self) -> GDResult<u16> {
        if self.check_size(2) {
            return Err(PacketUnderflow);
        }

        let value = match self.endianess {
            Endianess::Little => LittleEndian::read_u16(self.remaining_data()),
            Endianess::Big => BigEndian::read_u16(self.remaining_data()),
        };

        self.move_position_ahead(2);
        Ok(value)
    }

    pub fn get_u32(&mut self) -> GDResult<u32> {
        if self.check_size(4) {
            return Err(PacketUnderflow);
        }

        let value = match self.endianess {
            Endianess::Little => LittleEndian::read_u32(self.remaining_data()),
            Endianess::Big => BigEndian::read_u32(self.remaining_data()),
        };

        self.move_position_ahead(4);
        Ok(value)
    }

    pub fn get_f32(&mut self) -> GDResult<f32> {
        if self.check_size(4) {
            return Err(PacketUnderflow);
        }

        let value = match self.endianess {
            Endianess::Little => LittleEndian::read_f32(self.remaining_data()),
            Endianess::Big => BigEndian::read_f32(self.remaining_data()),
        };

        self.move_position_ahead(4);
        Ok(value)
    }

    pub fn get_u64(&mut self) -> GDResult<u64> {
        if self.check_size(8) {
            return Err(PacketUnderflow);
        }

        let value = match self.endianess {
            Endianess::Little => LittleEndian::read_u64(self.remaining_data()),
            Endianess::Big => BigEndian::read_u64(self.remaining_data()),
        };

        self.move_position_ahead(8);
        Ok(value)
    }

    pub fn get_string_utf8(&mut self) -> GDResult<String> {
        let sub_buf = self.remaining_data();
        if sub_buf.is_empty() {
            return Err(PacketUnderflow);
        }

        let first_null_position = sub_buf.iter().position(|&x| x == 0).ok_or(PacketBad)?;
        let value = std::str::from_utf8(&sub_buf[.. first_null_position])
            .map_err(|_| PacketBad)?
            .to_string();

        self.move_position_ahead(value.len() + 1);
        Ok(value)
    }

    pub fn get_string_utf8_unended(&mut self) -> GDResult<String> {
        let sub_buf = self.remaining_data();
        if sub_buf.is_empty() {
            return Err(PacketUnderflow);
        }

        let value = std::str::from_utf8(sub_buf)
            .map_err(|_| PacketBad)?
            .to_string();

        self.move_position_ahead(value.len());
        Ok(value)
    }

    pub fn get_string_utf16(&mut self) -> GDResult<String> {
        let sub_buf = self.remaining_data();
        if sub_buf.is_empty() {
            return Err(PacketUnderflow);
        }

        let paired_buf: Vec<u16> = sub_buf
            .chunks_exact(2)
            .map(|pair| {
                match self.endianess {
                    Endianess::Little => LittleEndian::read_u16(pair),
                    Endianess::Big => BigEndian::read_u16(pair),
                }
            })
            .collect();

        let value = String::from_utf16(&paired_buf).map_err(|_| PacketBad)?;

        self.move_position_ahead(value.len() * 2);
        Ok(value)
    }

    pub fn move_position_ahead(&mut self, by: usize) { self.position += by; }

    pub fn move_position_backward(&mut self, by: usize) { self.position -= by; }

    pub fn data_length(&self) -> usize { self.data.len() }

    pub fn remaining_data(&self) -> &[u8] { &self.data[self.position ..] }

    pub fn remaining_data_vec(&self) -> Vec<u8> { self.remaining_data().to_vec() }

    pub fn remaining_length(&self) -> usize { self.data_length() - self.position }

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
        let mut buffer =
            Bufferer::new_with_data(Endianess::Little, &[72, 29, 128, 99, 69, 4, 2, 0]);

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
        let mut buffer =
            Bufferer::new_with_data(Endianess::Little, &[72, 101, 108, 108, 111, 0, 72]);

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
        let mut buffer = Bufferer::new_with_data(
            Endianess::Little,
            &[0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00],
        );

        assert_eq!(buffer.get_string_utf16().unwrap(), "Hello");
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_string_utf16().is_err());
    }

    #[test]
    fn get_string_utf16_be() {
        let mut buffer = Bufferer::new_with_data(
            Endianess::Big,
            &[0x00, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f],
        );

        assert_eq!(buffer.get_string_utf16().unwrap(), "Hello");
        assert_eq!(buffer.remaining_length(), 0);
        assert!(buffer.get_string_utf16().is_err());
    }
}
