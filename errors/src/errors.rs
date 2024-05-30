use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde::Serialize;
use strum::EnumString;
use utoipa::ToSchema;



pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error, ToSchema)]
pub enum AppError {
    #[error("{0} not found")]
    NotFoundError(Resource),
    #[error("{0} not available")]
    NotAvailableError(Resource),

}


impl AppError {
    pub fn response(self) -> (StatusCode, AppResponseError) {
        use AppError::*;
        let message = self.to_string();
        let (kind, code, details, status_code) = match self {

            NotAvailableError(resource) => (
                format!("{resource}_NOT_AVAILABLE_ERROR"),
                None,
                vec![],
                StatusCode::NOT_FOUND,
            ),
            NotFoundError(resource) => (
                format!("{resource}_NOT_FOUND_ERROR"),
                Some(resource.resource_type as i32),
                resource.details.clone(),
                StatusCode::NOT_FOUND,
            ),
        };

        (
            status_code,
            AppResponseError::new(kind, message, code, details),
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, body) = self.response();
        (status_code, Json(body)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, utoipa::ToSchema)]
pub struct AppResponseError {
    pub kind: String,
    pub error_message: String,
    pub code: Option<i32>,
    pub details: Vec<(String, String)>,
}

impl AppResponseError {
    pub fn new(
        kind: impl Into<String>,
        message: impl Into<String>,
        code: Option<i32>,
        details: Vec<(String, String)>,
    ) -> Self {
        Self {
            kind: kind.into(),
            error_message: message.into(),
            code,
            details,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resource {
    pub details: Vec<(String, String)>,
    pub resource_type: ResourceType,
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        self.resource_type.fmt(f)
    }
}

#[derive(Debug, EnumString, strum::Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceType {
    #[strum(serialize = "USER")]
    User,
    #[strum(serialize = "FILE")]
    File,
    #[strum(serialize = "SESSION")]
    Session,
    #[strum(serialize = "MESSAGE")]
    Message,
}


