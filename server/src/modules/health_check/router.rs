use axum::{routing::get, Router};

use crate::AppState;

use super::controller::health_check;

pub fn get_router() -> Router<AppState> {
    Router::new().route("/", get(health_check))
}
