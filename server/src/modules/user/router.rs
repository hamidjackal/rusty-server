use axum::{routing::get, Router};

use crate::AppState;

use super::controller::{create_user, get_user, list_users};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/:user_id", get(get_user))
        .route("/", get(list_users).post(create_user))
}
