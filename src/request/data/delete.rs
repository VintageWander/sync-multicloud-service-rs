use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{error::Error, Services};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeleteDataRequest {
    #[serde(rename = "type")]
    pub _type: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
}

#[async_trait]
impl FromRequest<Services, Body> for DeleteDataRequest {
    type Rejection = Error;
    async fn from_request(req: Request<Body>, state: &Services) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<DeleteDataRequest>::from_request(req, state).await?;
        Ok(body)
    }
}
