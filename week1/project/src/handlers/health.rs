use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;

use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    state.increment_requests();
    Json(json!({
        "status":"OK",
        "request_count":state.get_request()
    }))
}
