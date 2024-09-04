use std::collections::HashMap;

mod sealed;

pub(crate) type Header = HashMap<String, String>;
pub(crate) type Query<'a> = HashMap<&'a str, &'a str>;
pub(crate) type Body = serde_json::Value;

#[allow(dead_code)]
pub(crate) struct HttpClient {}
