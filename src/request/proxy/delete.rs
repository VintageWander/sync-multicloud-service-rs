use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::{error::Error, models::proxy::Proxy, Services};

#[derive(Deserialize, Validate, ToSchema)]
pub struct DeleteProxyRequest {
    #[validate(url(message = "Proxy url is invalid"))]
    pub url: String,
}

#[async_trait]
impl FromRequest<Services, Body> for DeleteProxyRequest {
    type Rejection = Error;
    async fn from_request(req: Request<Body>, state: &Services) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<DeleteProxyRequest>::from_request(req, state).await?;
        body.validate()?;
        Ok(body)
    }
}

impl From<DeleteProxyRequest> for Proxy {
    fn from(DeleteProxyRequest { url }: DeleteProxyRequest) -> Self {
        Self { url }
    }
}
