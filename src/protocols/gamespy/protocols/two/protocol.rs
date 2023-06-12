use crate::bufferer::{Bufferer, Endianess};
use crate::protocols::types::TimeoutSettings;
use crate::socket::{Socket, UdpSocket};
use crate::{GDError, GDResult};
use std::net::SocketAddr;

struct GameSpy2 {
    socket: UdpSocket,
}

const PACKET_SIZE: usize = 2048;

impl GameSpy2 {
    fn new(address: &SocketAddr, timeout_settings: Option<TimeoutSettings>) -> GDResult<Self> {
        let socket = UdpSocket::new(address)?;
        socket.apply_timeout(timeout_settings)?;

        Ok(Self { socket })
    }

    fn request(&mut self, data: &[u8]) -> GDResult<Bufferer> {
        self.socket
            .send(&*[data, &[0xFE, 0xFD, 0x00], &[0x00, 0x00, 0x00, 0x01]].concat())?;
        let received = self.socket.receive(None)?;
        let mut buf = Bufferer::new_with_data(Endianess::Big, &received);

        if buf.get_u8()? != 0 {
            return Err(GDError::PacketBad);
        }

        if buf.get_u32()? != 1 {
            return Err(GDError::PacketBad);
        }

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn gs2() {
        let mut gs2 = GameSpy2::new(
            &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(108, 61, 236, 22)), 15567),
            None,
        )
        .unwrap();

        let k = gs2.request(&[0xFF, 0x00, 0x00]).unwrap();
        println!("{:02X?}", k.remaining_data());
    }
}
