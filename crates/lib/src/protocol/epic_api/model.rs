use {
    chrono::{DateTime, TimeDelta, Utc},
    serde::Deserialize,
};

pub enum RoutingScope {
    Default,
    Wildcard,
}

pub struct Credentials {
    pub id: &'static str,
    pub secret: &'static str,
    pub deployment: &'static str,
    pub routing_scope: RoutingScope,
}

#[derive(Deserialize)]
pub(crate) struct OAuthToken {
    pub(crate) access_token: String,
    expires_at: DateTime<Utc>,
}

impl OAuthToken {
    pub(crate) fn is_valid(&self) -> bool { self.expires_at > Utc::now() + TimeDelta::seconds(30) }
}
