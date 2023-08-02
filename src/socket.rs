use crate::{
    protocols::types::TimeoutSettings,
    GDErrorKind::{PacketReceive, PacketSend, SocketBind, SocketConnect},
    GDResult,
};

use std::net::SocketAddr;
use std::{
    io::{Read, Write},
    net,
};

const DEFAULT_PACKET_SIZE: usize = 1024;

pub trait Socket {
    fn new(address: &SocketAddr) -> GDResult<Self>
    where Self: Sized;

    fn apply_timeout(&self, timeout_settings: Option<TimeoutSettings>) -> GDResult<()>;

    fn send(&mut self, data: &[u8]) -> GDResult<()>;
    fn receive(&mut self, size: Option<usize>) -> GDResult<Vec<u8>>;
}

pub struct TcpSocket {
    socket: net::TcpStream,
}

impl Socket for TcpSocket {
    fn new(address: &SocketAddr) -> GDResult<Self> {
        Ok(Self {
            socket: net::TcpStream::connect(address).map_err(|_| SocketConnect)?,
        })
    }

    fn apply_timeout(&self, timeout_settings: Option<TimeoutSettings>) -> GDResult<()> {
        let settings = timeout_settings.unwrap_or_default();
        self.socket.set_read_timeout(settings.get_read()).unwrap(); // unwrapping because TimeoutSettings::new
        self.socket.set_write_timeout(settings.get_write()).unwrap(); // checks if these are 0 and throws an error

        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> GDResult<()> {
        self.socket.write(data).map_err(|_| PacketSend)?;
        Ok(())
    }

    fn receive(&mut self, size: Option<usize>) -> GDResult<Vec<u8>> {
        let mut buf = Vec::with_capacity(size.unwrap_or(DEFAULT_PACKET_SIZE));
        self.socket
            .read_to_end(&mut buf)
            .map_err(|_| PacketReceive)?;

        Ok(buf)
    }
}

pub struct UdpSocket {
    socket: net::UdpSocket,
    address: SocketAddr,
}

impl Socket for UdpSocket {
    fn new(address: &SocketAddr) -> GDResult<Self> {
        let socket = net::UdpSocket::bind("0.0.0.0:0").map_err(|_| SocketBind)?;

        Ok(Self {
            socket,
            address: *address,
        })
    }

    fn apply_timeout(&self, timeout_settings: Option<TimeoutSettings>) -> GDResult<()> {
        let settings = timeout_settings.unwrap_or_default();
        self.socket.set_read_timeout(settings.get_read()).unwrap(); // unwrapping because TimeoutSettings::new
        self.socket.set_write_timeout(settings.get_write()).unwrap(); // checks if these are 0 and throws an error

        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> GDResult<()> {
        self.socket
            .send_to(data, self.address)
            .map_err(|_| PacketSend)?;

        Ok(())
    }

    fn receive(&mut self, size: Option<usize>) -> GDResult<Vec<u8>> {
        let mut buf: Vec<u8> = vec![0; size.unwrap_or(DEFAULT_PACKET_SIZE)];
        let (number_of_bytes_received, _) = self.socket.recv_from(&mut buf).map_err(|_| PacketReceive)?;

        Ok(buf[.. number_of_bytes_received].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_tcp_socket_send_and_receive() {
        // Spawn a thread to run the server
        let listener = net::TcpListener::bind("127.0.0.1:0").unwrap();
        let bound_address = listener.local_addr().unwrap();
        let server_thread = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0; 1024];
            let _ = stream.read(&mut buf).unwrap();
            let _ = stream.write(&buf).unwrap();
        });

        // Create a TCP socket and send a message to the server
        let mut socket = TcpSocket::new(&bound_address).unwrap();
        let message = b"hello, world!";
        socket.send(message).unwrap();

        // Receive the response from the server
        let received_message: Vec<u8> = socket
            .receive(None)
            .unwrap()
            // Iterate over the buffer and remove 0s that are alone in the buffer
            // just added to pass default size
            .into_iter()
            .filter(|&x| x != 0)
            .collect();

        server_thread.join().expect("server thread panicked");

        assert_eq!(message, &received_message[..]);
    }

    #[test]
    fn test_udp_socket_send_and_receive() {
        // Spawn a thread to run the server
        let socket = net::UdpSocket::bind("127.0.0.1:0").unwrap();
        let bound_address = socket.local_addr().unwrap();
        let server_thread = thread::spawn(move || {
            let mut buf = [0; 1024];
            let (_, src_addr) = socket.recv_from(&mut buf).unwrap();
            socket.send_to(&buf, src_addr).unwrap();
        });

        // Create a UDP socket and send a message to the server
        let mut socket = UdpSocket::new(&bound_address).unwrap();
        let message = b"hello, world!";
        socket.send(message).unwrap();

        // Receive the response from the server
        let received_message: Vec<u8> = socket
            .receive(None)
            .unwrap()
            // Iterate over the buffer and remove 0s that are alone in the buffer
            // just added to pass default size
            .into_iter()
            .filter(|&x| x != 0)
            .collect();

        server_thread.join().expect("server thread panicked");

        assert_eq!(message, &received_message[..]);
    }
}
