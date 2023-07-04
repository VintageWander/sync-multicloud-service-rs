use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct SuccessResponse {
    pub code: String,
    pub message: String,
    pub data: (),
    pub error: String,
}
