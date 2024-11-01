use std::sync::Arc;

use crate::{handler::register, AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/register", post(register))
        .with_state(app_state)
}
