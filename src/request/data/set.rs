use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::{error::Error, Services};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SetDataRequest {
    #[serde(rename = "type")]
    pub _type: String,
    pub key: String,
    pub value: Value,
}

#[async_trait]
impl FromRequest<Services, Body> for SetDataRequest {
    type Rejection = Error;
    async fn from_request(req: Request<Body>, state: &Services) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<SetDataRequest>::from_request(req, state).await?;
        Ok(body)
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SetMultiDataRequest {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: Value,
}

#[async_trait]
impl FromRequest<Services, Body> for SetMultiDataRequest {
    type Rejection = Error;
    async fn from_request(req: Request<Body>, state: &Services) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<SetMultiDataRequest>::from_request(req, state).await?;
        Ok(body)
    }
}
