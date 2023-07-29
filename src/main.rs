mod controllers;
mod services;

use axum::{routing::get, Router};
use controllers::health_check::health_check;
use services::http_response::HttpError;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let router = Router::new();
    let app = router
        .route("/health-check", get(health_check))
        .fallback(|| async { HttpError::not_found(None).unwrap() });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
