use std::collections::HashMap;

pub(crate) type Header<'a> = HashMap<&'a str, &'a str>;
pub(crate) type QueryParam<'a> = HashMap<&'a str, &'a str>;

pub(crate) type Json = serde_json::Value;
pub(crate) type Form<'a> = HashMap<&'a str, &'a str>;

pub(crate) enum Body<'a> {
    Json(Json),
    Form(Form<'a>),
}

pub(crate) struct RequestBuilder<'a> {
    pub(crate) url: &'a str,
    pub(crate) body: Option<Body<'a>>,
    pub(crate) header: Option<Header<'a>>,
    pub(crate) query_params: Option<QueryParam<'a>>,
}

impl<'a> RequestBuilder<'a> {
    pub(crate) fn new(url: &'a str) -> Self {
        Self {
            url,
            body: None,
            header: None,
            query_params: None,
        }
    }

    pub(crate) fn body(mut self, body: Body<'a>) -> Self {
        self.body = Some(body);

        self
    }

    pub(crate) fn header(mut self, key: &'a str, value: &'a str) -> Self {
        self.header
            .get_or_insert_with(Header::new)
            .insert(key, value);

        self
    }

    pub(crate) fn query_param(mut self, key: &'a str, value: &'a str) -> Self {
        self.query_params
            .get_or_insert_with(QueryParam::new)
            .insert(key, value);

        self
    }
}
