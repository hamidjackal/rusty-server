mod modules;
mod services;

use axum::Router;
use services::http_response::HttpError;
use std::net::SocketAddr;

use modules::router;

#[tokio::main]
async fn main() {
    let health_check_router = router::get_routers();

    let router = Router::new();
    let app = router
        .nest("/api", health_check_router)
        .fallback(|| async { HttpError::not_found(None).unwrap() });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
