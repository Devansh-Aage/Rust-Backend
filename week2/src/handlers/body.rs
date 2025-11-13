use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize)]
pub struct Creds {
    pub name: String,
    pub email: String,
}

pub async fn parse_body(Json(payload): Json<Creds>) -> Json<Value> {
    let name = payload.name;
    let email = payload.email;
    Json(json!({
        "name":name,
        "email":email
    }))
}
