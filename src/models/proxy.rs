use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// This model is used to interact with the mongodb database

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Proxy {
    pub url: String,
}
