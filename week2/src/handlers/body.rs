use axum::{Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use serde_valid::Validate;
use serde_valid::validation::Error;

use crate::error::{AppError, Limit};

fn validate_email(value: &str) -> Result<(), Error> {
    if value.contains("@") {
        Ok(())
    } else {
        Err(Error::Custom(
            "email field must be a valid Email".to_string(),
        ))
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Creds {
    #[validate(minimum = 1)]
    pub id: u32,
    #[validate(min_length = 3)]
    pub name: String,
    #[validate(custom= validate_email)]
    pub email: String,
}

pub async fn parse_body(Json(payload): Json<Creds>) -> Result<Json<Value>, AppError> {
    if let Err(errs) = payload.validate() {
        return Err(AppError::ValidationError(errs));
    }
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
