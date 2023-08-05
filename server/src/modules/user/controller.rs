use axum::extract::{Path, State};
use axum::Json;
use sea_orm::prelude::Uuid;
use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

use crate::core::repository::RepoErr;
use crate::AppState;

use crate::modules::user::service::UserService as user_service;

#[derive(Deserialize, Validate, Debug)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    state: State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<Value>, RepoErr> {
    println!("Payload: {:?}", payload);
    match payload.validate() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    };
    user_service::create_user(&state.conn, Json(payload)).await
}

pub async fn list_users(state: State<AppState>) -> Result<Json<Value>, RepoErr> {
    user_service::list_users(&state.conn).await
}

pub async fn get_user(
    state: State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, RepoErr> {
    user_service::get_user(&state.conn, Path(user_id)).await
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);
