use axum::Router;

use super::health_check::router::get_router as get_health_check_router;

pub fn get_routers() -> Router {
    Router::new().nest("/health-check", get_health_check_router())
}
