use axum::extract::{Path, State};
use axum::Json;
use sea_orm::prelude::Uuid;
use serde_json::Value;

use crate::core::repository::RepoErr;
use crate::AppState;

use crate::modules::user::service::UserService as user_service;

use super::serializer::{CreateUser, LoginCredentials, UpdateUser};

pub struct UserController {}

impl UserController {
    pub async fn create_user(
        state: State<AppState>,
        Json(payload): Json<CreateUser>,
    ) -> Result<Json<Value>, RepoErr> {
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

    pub async fn delete_user(
        state: State<AppState>,
        Path(user_id): Path<Uuid>,
    ) -> Result<(), RepoErr> {
        user_service::delete_user(&state.conn, Path(user_id)).await
    }

    pub async fn update_user(
        state: State<AppState>,
        Path(user_id): Path<Uuid>,
        Json(payload): Json<UpdateUser>,
    ) -> Result<Json<Value>, RepoErr> {
        user_service::update_user(&state.conn, Path(user_id), Json(payload)).await
    }

    pub async fn login(
        state: State<AppState>,
        Json(payload): Json<LoginCredentials>,
    ) -> Result<Json<Value>, RepoErr> {
        user_service::login(&state.conn, Json(payload)).await
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);
