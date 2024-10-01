use std::collections::HashMap;

pub(crate) type Header = HashMap<String, String>;
pub(crate) type UrlQueryParam = HashMap<String, String>;

pub(crate) type Json = serde_json::Value;
pub(crate) type Form = HashMap<String, String>;

pub(crate) enum Body {
    Json(Json),
    Form(Form),
}

pub(crate) struct RequestBuilder {
    url: String,
    body: Option<Body>,
    header: Option<Header>,
    query_params: Option<UrlQueryParam>,
}

impl RequestBuilder {
    pub(crate) fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            body: None,
            header: None,
            query_params: None,
        }
    }

    pub(crate) fn body(mut self, body: Body) -> Self {
        self.body = Some(body);

        self
    }

    pub(crate) fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.header
            .get_or_insert_with(Header::new)
            .insert(key.into(), value.into());

        self
    }

    pub(crate) fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params
            .get_or_insert_with(UrlQueryParam::new)
            .insert(key.into(), value.into());

        self
    }

    pub(crate) const fn build(self) -> Self { self }
}
