use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;
use validator_derive::Validate;

use crate::{
    response::{error_response, success_response, validation_errors_to_json},
    service::hash_password,
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

pub async fn register(
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

    Ok((
        StatusCode::CREATED,
        Json(success_response(
            "User registered",
            json!({
                "username": payload.username,
                "email": payload.email,
                "password": password,
            }),
        )),
    ))
}
