use crate::GDErrorKind::{PacketSend, ProtocolFormat, SocketConnect};
use crate::{GDResult, TimeoutSettings};
use reqwest::blocking::*;
use serde::de::DeserializeOwned;
use std::net::SocketAddr;

pub struct HttpClient {
    client: Client,
    address: String,
}

impl HttpClient {
    pub fn new(address: &SocketAddr, timeout_settings: &Option<TimeoutSettings>) -> GDResult<Self>
    where Self: Sized {
        let client = Client::builder()
            .connect_timeout(TimeoutSettings::get_connect_or_default(timeout_settings))
            .timeout(TimeoutSettings::get_connect_or_default(timeout_settings))
            .build()
            .map_err(|e| SocketConnect.context(e))?;

        Ok(Self {
            client,
            address: format!("http://{}:{}", address.ip(), address.port()),
        })
    }

    pub fn concat_path(&self, path: &str) -> String { format!("{}{}", self.address, path) }

    pub fn request<T: DeserializeOwned>(&mut self, path: &str) -> GDResult<T> {
        self.client
            .get(self.concat_path(path))
            .send()
            .map_err(|e| PacketSend.context(e))?
            .json::<T>()
            .map_err(|e| ProtocolFormat.context(e))
    }
}
