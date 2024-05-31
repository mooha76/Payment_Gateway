
use errors::errors::AppResult;

use axum::Json;
use response::response::MessageResponse;

// Health check.
#[utoipa::path(
    get,
    path = "/api/v1/server/health_check",
    responses(
        (status = 200, description = "check service is up", body = [MessageResponse])
    )
)]
pub async fn health_check() -> AppResult<Json<MessageResponse>> {
    Ok(Json(MessageResponse::new("Ok")))
}