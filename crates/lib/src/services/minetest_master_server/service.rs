use crate::http::HttpClient;
use crate::minetest_master_server::types::Response;
use crate::{GDResult, TimeoutSettings};

pub fn query(timeout_settings: TimeoutSettings) -> GDResult<Response> {
    let mut client = HttpClient::from_url(
        "https://servers.minetest.net",
        &Some(timeout_settings),
        None,
    )?;

    client.get_json("/list", Default::default())
}
