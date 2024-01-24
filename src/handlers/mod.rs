use axum::{routing::get, Router};

use crate::AppState;

mod healthcheck;
pub mod openapi;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/__healthcheck", get(healthcheck::get))
        .route("/openapi", get(openapi::get))
}
