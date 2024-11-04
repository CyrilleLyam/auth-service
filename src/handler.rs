use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;
use validator_derive::Validate;

use crate::{
    response::{error_response, success_response, validation_errors_to_json},
    service::{encode_jwt, hash_password},
    AppState,
};

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 1, message = "Username is required"))]
    username: String,
    #[validate(email(message = "Invalid email format"))]
    email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Validate the payload
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(error_response(
                "Validation failed",
                validation_errors_to_json(validation_errors),
            )),
        ));
    }

    let password = match hash_password(&payload.password) {
        Ok(pwd) => pwd,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error_response("Password hashing failed", json!({}))),
            ));
        }
    };

    let query_result = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        payload.username,
        payload.email,
        password,
    )
    .fetch_one(&app_state.db)
    .await;

    match query_result {
        Ok(record) => Ok((
            StatusCode::CREATED,
            Json(success_response(
                "User registered",
                json!({ "id": record.id }),
            )),
        )),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error_response("Failed to register user", json!({}))),
            ))
        }
    }
}
