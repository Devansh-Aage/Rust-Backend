use axum::{Json, extract::Query};
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn name(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    let name = params.get("name").unwrap();
    Json(json!({
        "message": format!("hello, {}!", name)
    }))
}
