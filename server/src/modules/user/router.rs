use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::fmt::Debug;

use crate::{
    core::{authorizer_middleware::UserAuthorizer, input_validation_middleware::BodyValidator},
    modules::user::serializer::CreateUser,
    AppState,
};

use super::controller::UserController;

#[derive(Debug, Clone, Copy, Default)]
struct CustomBody {}

pub fn get_router() -> Router<AppState> {
    let create_user: Router<AppState> = Router::new()
        .route("/", post(UserController::create_user))
        .route_layer(middleware::from_fn(BodyValidator::validate::<CreateUser>));

    let login: Router<AppState> = Router::new().route("/", post(UserController::login));

    let authorized_routes: Router<AppState> = Router::new()
        .route(
            "/:user_id",
            get(UserController::get_user)
                .put(UserController::update_user)
                .delete(UserController::delete_user),
        )
        .route("/", get(UserController::list_users))
        .nest("/", create_user.clone())
        .route_layer(middleware::from_fn(UserAuthorizer::auth));

    Router::new()
        .nest("/", authorized_routes)
        .nest("/login", login)
        .nest("/signup", create_user)
}
