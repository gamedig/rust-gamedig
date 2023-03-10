use std::io::{Read, Write};
use std::net;
use crate::GDResult;
use crate::GDError::{PacketReceive, PacketSend, SocketBind, SocketConnect};
use crate::protocols::types::TimeoutSettings;
use crate::utils::address_and_port_as_string;

const DEFAULT_PACKET_SIZE: usize = 1024;

pub trait Socket {
    fn new(address: &str, port: u16) -> GDResult<Self> where Self: Sized;

    fn apply_timeout(&self, timeout_settings: Option<TimeoutSettings>) -> GDResult<()>;

    fn send(&mut self, data: &[u8]) -> GDResult<()>;
    fn receive(&mut self, size: Option<usize>) -> GDResult<Vec<u8>>;
}

pub struct TcpSocket {
    socket: net::TcpStream
}

impl Socket for TcpSocket {
    fn new(address: &str, port: u16) -> GDResult<Self> {
        let complete_address = address_and_port_as_string(address, port);

        Ok(Self {
            socket: net::TcpStream::connect(complete_address).map_err(|_| SocketConnect)?
        })
    }

    fn apply_timeout(&self, timeout_settings: Option<TimeoutSettings>) -> GDResult<()> {
        let settings = timeout_settings.unwrap_or_default();
        self.socket.set_read_timeout(settings.get_read()).unwrap();   //unwrapping because TimeoutSettings::new
        self.socket.set_write_timeout(settings.get_write()).unwrap(); //checks if these are 0 and throws an error

        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> GDResult<()> {
        self.socket.write(data).map_err(|_| PacketSend)?;
        Ok(())
    }

    fn receive(&mut self, size: Option<usize>) -> GDResult<Vec<u8>> {
        let mut buf = Vec::with_capacity(size.unwrap_or(DEFAULT_PACKET_SIZE));
        self.socket.read_to_end(&mut buf).map_err(|_| PacketReceive)?;

        Ok(buf)
    }
}

pub struct UdpSocket {
    socket: net::UdpSocket,
    complete_address: String
}

impl Socket for UdpSocket {
    fn new(address: &str, port: u16) -> GDResult<Self> {
        let complete_address = address_and_port_as_string(address, port);
        let socket = net::UdpSocket::bind("0.0.0.0:0").map_err(|_| SocketBind)?;

        Ok(Self {
            socket,
            complete_address
        })
    }

    fn apply_timeout(&self, timeout_settings: Option<TimeoutSettings>) -> GDResult<()> {
        let settings = timeout_settings.unwrap_or_default();
        self.socket.set_read_timeout(settings.get_read()).unwrap();   //unwrapping because TimeoutSettings::new
        self.socket.set_write_timeout(settings.get_write()).unwrap(); //checks if these are 0 and throws an error

        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> GDResult<()> {
        self.socket.send_to(data, &self.complete_address).map_err(|_| PacketSend)?;
        Ok(())
    }

    fn receive(&mut self, size: Option<usize>) -> GDResult<Vec<u8>> {
        let mut buf: Vec<u8> = vec![0; size.unwrap_or(DEFAULT_PACKET_SIZE)];
        let (number_of_bytes_received, _) = self.socket.recv_from(&mut buf).map_err(|_| PacketReceive)?;

        Ok(buf[..number_of_bytes_received].to_vec())
    }
}
