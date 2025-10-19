use std::{net::SocketAddr, time::Duration};

pub struct TimeoutConnectionless {
    read: Option<Duration>,
    write: Option<Duration>,
}

pub struct TimeoutConnectionOriented {
    read: Option<Duration>,
    write: Option<Duration>,
    connect: Option<Duration>,
}

pub trait SelectTimeout {
    type Type;
}

pub struct ConnectionMode<const CONNECTIONLESS: bool>;

impl SelectTimeout for ConnectionMode<true> {
    type Type = TimeoutConnectionless;
}

impl SelectTimeout for ConnectionMode<false> {
    type Type = TimeoutConnectionOriented;
}

pub struct NetConfig<const CONNECTIONLESS: bool>
where ConnectionMode<CONNECTIONLESS>: SelectTimeout {
    address: SocketAddr,
    timeout: <ConnectionMode<CONNECTIONLESS> as SelectTimeout>::Type,
}

impl NetConfig<true> {
    pub const fn new(address: SocketAddr) -> Self {
        Self {
            address,
            timeout: TimeoutConnectionless {
                read: None,
                write: None,
            },
        }
    }

    pub const fn set_read_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout.read = timeout;
    }

    pub const fn set_write_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout.write = timeout;
    }

    pub const fn address(&self) -> SocketAddr { self.address }

    pub const fn read_timeout(&self) -> Option<Duration> { self.timeout.read }

    pub const fn write_timeout(&self) -> Option<Duration> { self.timeout.write }
}

impl NetConfig<false> {
    pub const fn new(address: SocketAddr) -> Self {
        Self {
            address,
            timeout: TimeoutConnectionOriented {
                read: None,
                write: None,
                connect: None,
            },
        }
    }

    pub const fn set_read_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout.read = timeout;
    }

    pub const fn set_write_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout.write = timeout;
    }

    pub const fn set_connect_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout.connect = timeout;
    }

    pub const fn address(&self) -> SocketAddr { self.address }

    pub const fn read_timeout(&self) -> Option<Duration> { self.timeout.read }

    pub const fn write_timeout(&self) -> Option<Duration> { self.timeout.write }

    pub const fn connect_timeout(&self) -> Option<Duration> { self.timeout.connect }
}
