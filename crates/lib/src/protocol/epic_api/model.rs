use {
    chrono::{DateTime, TimeDelta, Utc},
    serde::{Deserialize, Serialize},
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

#[derive(Debug, Clone, Serialize)]
pub struct Criteria {
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Criterion {
    pub key: CriterionKey,
    pub op: CriteriaOp,
    pub value: CriteriaValue,
}

#[derive(Debug, Clone, Serialize)]
pub enum CriteriaOp {
    #[serde(rename = "EQUAL")]
    Equal,
    #[serde(rename = "NOT_EQUAL")]
    NotEqual,
    #[serde(rename = "CONTAINS")]
    Contains,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CriteriaValue {
    String(String),
    Number(u32),
    Bool(bool),
}

impl From<String> for CriteriaValue {
    fn from(v: String) -> Self { Self::String(v) }
}

impl From<&str> for CriteriaValue {
    fn from(v: &str) -> Self { Self::String(v.to_owned()) }
}
impl From<u8> for CriteriaValue {
    fn from(v: u8) -> Self { Self::Number(v as u32) }
}

impl From<u16> for CriteriaValue {
    fn from(v: u16) -> Self { Self::Number(v as u32) }
}

impl From<u32> for CriteriaValue {
    fn from(v: u32) -> Self { Self::Number(v) }
}

impl From<bool> for CriteriaValue {
    fn from(v: bool) -> Self { Self::Bool(v) }
}

#[derive(Debug, Clone, Serialize)]
pub enum CriterionKey {
    // Addressing
    #[serde(rename = "attributes.ADDRESS_s")]
    Address,

    #[serde(rename = "attributes.ADDRESSBOUND_s")]
    AddressBound,

    #[serde(rename = "attributes.ADDRESSDEV_s")]
    AddressDev,

    // Identity / Naming
    #[serde(rename = "id")]
    Id,

    #[serde(rename = "owner")]
    Owner,

    #[serde(rename = "ownerPlatformId")]
    OwnerPlatformId,

    #[serde(rename = "attributes.SESSIONNAME_s")]
    SessionName,

    #[serde(rename = "attributes.SESSIONNAMEUPPER_s")]
    SessionNameUpper,

    #[serde(rename = "attributes.CUSTOMSERVERNAME_s")]
    CustomServerName,
}
