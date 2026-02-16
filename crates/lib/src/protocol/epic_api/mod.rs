mod client;
mod error;
mod model;

// Public
pub use {
    client::EpicApiClient,
    error::EpicApiClientError,
    model::{
        Credentials,
        Criteria,
        CriteriaOp,
        CriteriaValue,
        Criterion,
        CriterionKey,
        RoutingScope,
    },
};

// Private
pub(crate) use model::OAuthToken;
