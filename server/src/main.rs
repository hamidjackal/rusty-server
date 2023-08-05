mod core;
mod helpers;
mod modules;

use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use helpers::http_response::HttpError;
use std::net::SocketAddr;

use migration::{Migrator, MigratorTrait};

use modules::router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let api_router = router::get_routers();

    let db_url = "postgres://user:password@localhost:5432/rsdb";
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None).await.expect("Migration failed");

    let state = AppState { conn };

    let router = Router::new();
    let app = router
        .nest("/api", api_router)
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
        .with_state(state)
        .fallback(|| async { HttpError::not_found(None).unwrap() });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}
