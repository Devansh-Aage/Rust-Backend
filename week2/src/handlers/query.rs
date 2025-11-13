use axum::{Json, extract::Query};
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::error::AppError;

pub async fn name(Query(params): Query<HashMap<String, String>>) -> Result<Json<Value>, AppError> {
    let name = params
        .get("name")
        .ok_or_else(|| AppError::BadRequest("Query Params not found!".into()))?; //.into on str cause Err variant expects String and not &str
    Ok(Json(json!({
        "message": format!("hello, {}!", name)
    })))
}
