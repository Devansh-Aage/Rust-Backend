use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Limit {
    pub li: u32,
    pub hi: u32,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation Error: {0}")]
    ValidationError(serde_valid::validation::Errors),

    #[error("Invalid ID: {id},expected atleast {} and atmost {}", .limit.li, .limit.hi)]
    // `.` used cause limit is nested
    OutOfBounds { id: u32, limit: Limit },

    #[error("Something went wrong")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
enum ErrorResponse {
    Message {
        error: String,
    },
    Validation {
        error: String,
        details: serde_json::Value,
    },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(errs) => {
                let json = serde_json::to_value(&errs)
                    .unwrap_or(json!({"error":"Failed to serialize validation error to json"}));

                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse::Validation {
                        error: "Validation Error".into(),
                        details: json,
                    }),
                )
                    .into_response()
            }

            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::Message { error: msg }),
            )
                .into_response(),

            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::Message {
                    error: "Resource not found".into(),
                }),
            )
                .into_response(),

            AppError::OutOfBounds { id, limit } => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::Message {
                    error: format!(
                        "Invalid ID {id}, must be between {} and {}",
                        limit.li, limit.hi
                    ),
                }),
            )
                .into_response(),

            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::Message {
                    error: "Internal Server Error".into(),
                }),
            )
                .into_response(),
        }
    }
}
