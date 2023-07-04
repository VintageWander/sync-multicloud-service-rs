use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub data: (),
    pub error: String,
}
