use axum::{routing::get, Router};

use super::controller::health_check;

pub fn get_router() -> Router {
    Router::new().route("/", get(health_check))
}
