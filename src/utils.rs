use crate::GDResult;
use crate::GDError::{PacketOverflow, PacketUnderflow};

pub fn error_by_expected_size(expected: usize, size: usize) -> GDResult<()> {
    if size < expected {
        Err(PacketUnderflow)
    }
    else if size > expected {
        Err(PacketOverflow)
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

#[cfg(test)]
mod tests {
    #[test]
    fn address_and_port_as_string() {
        assert_eq!(super::address_and_port_as_string("192.168.0.1", 27015), "192.168.0.1:27015");
    }

    #[test]
    fn u8_lower_upper() {
        assert_eq!(super::u8_lower_upper(171), (11, 10));
    }

    #[test]
    fn error_by_expected_size() {
        assert!(super::error_by_expected_size(69, 69).is_ok());
        assert!(super::error_by_expected_size(69, 68).is_err());
        assert!(super::error_by_expected_size(69, 70).is_err());
    }
}
