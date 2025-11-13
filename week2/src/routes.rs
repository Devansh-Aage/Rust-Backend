use crate::handlers::body::parse_body;
use crate::handlers::query::name;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;

pub fn create_router() -> Router {
    Router::new()
        .route("/query", get(name))
        .route("/", get(|| async { Json(json!("Hello")) }))
        .route("/parse", post(parse_body))
}
