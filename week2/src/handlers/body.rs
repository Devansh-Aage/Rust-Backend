use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::error::{AppError, Limit};

#[derive(Serialize, Deserialize)]
pub struct Creds {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub async fn parse_body(Json(payload): Json<Creds>) -> Result<Json<Value>, AppError> {
    let id = payload.id;
    let name = payload.name;
    let email = payload.email;

    if email != "devansh@gmail.com" {
        return Err(AppError::NotFound);
    }
    if id < 1 || id > u32::MAX {
        return Err(AppError::OutOfBounds {
            id,
            limit: Limit {
                li: 1,
                hi: u32::MAX,
            },
        });
    }
    Ok(Json(json!({
        "id":id,
        "name":name,
        "email":email
    })))
}
