use axum::{Json, extract::State};
use chrono::Utc;
use serde_json::json;

use crate::state::AppState;

pub async fn get_time(State(state): State<AppState>) -> Json<serde_json::Value> {
    state.increment_requests();
    let now = Utc::now().to_rfc3339();
    Json(json!({"current_time":now}))
}
