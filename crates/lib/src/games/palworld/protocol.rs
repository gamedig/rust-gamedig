use crate::http::HttpClient;
use crate::palworld::types::{Endpoint, Response};
use crate::GDErrorKind::SocketConnect;
use crate::{GDResult, TimeoutSettings};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde_json::Value;
use std::net::IpAddr;
use url::Url;

fn make_call(client: &mut HttpClient, endpoint: Endpoint) -> GDResult<Value> {
    client.get_json(endpoint.into(), Default::default())
}

pub fn query(
    address: &IpAddr,
    port: Option<u16>,
    username: String,
    password: String,
    timeout_settings: TimeoutSettings,
) -> GDResult<Response> {
    let url = Url::parse(&format!("{address}:{}", port.unwrap_or(8212))).map_err(|e| SocketConnect.context(e))?;

    let auth_format = format!("{}:{}", username, password);
    let auth_base = BASE64_STANDARD.encode(auth_format);
    let auth = format!("Basic {}", auth_base.as_str());
    let authorization = auth.as_str();
    let headers = [
        ("Authorization", authorization),
        ("Accept", "application/json"),
    ];

    let mut client = HttpClient::from_url(url, &Some(timeout_settings), Some((&headers).to_vec()))?;

    let info = make_call(&mut client, Endpoint::Info)?;
    println!("{info:#?}");

    Ok(Response {})
}
