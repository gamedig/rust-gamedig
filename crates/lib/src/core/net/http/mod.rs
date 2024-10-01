#[allow(unused_imports)]
use sealed::client::AbstractHttp;

pub(crate) mod request;
mod sealed;

#[allow(dead_code)]
pub(crate) struct HttpClient {}
