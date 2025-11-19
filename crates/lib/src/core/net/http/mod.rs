use std::collections::HashMap;
use serde_json::Value;

mod sealed;

pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;
pub type Form<'a> = HashMap<&'a str, &'a str>;

pub enum Payload<'a> {
    Json(&'a Value),
    Form(&'a Form<'a>),
}

pub(crate) struct HttpsClient {
    inner: sealed::client::Inner,
}

