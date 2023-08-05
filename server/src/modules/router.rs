use axum::Router;

use crate::AppState;

use super::health_check::router::get_router as get_health_check_router;
use super::user::router::get_router as user_router;

pub fn get_routers() -> Router<AppState> {
    Router::new()
        .nest("/health-check", get_health_check_router())
        .nest("/users", user_router())
}
