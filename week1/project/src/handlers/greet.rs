use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::json;

use crate::state::AppState;

pub async fn greet_user(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    state.increment_requests();
    Json(json!({
        "message":format!("Hello, {}!",name)
    }))
}
