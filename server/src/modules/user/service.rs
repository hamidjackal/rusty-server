use axum::extract::Path;
use axum::Json;
use sea_orm::{prelude::Uuid, DbConn};
use serde_json::Value;

use super::repository::UserRepository;
use super::serializer::CreateUser;
use crate::core::repository::{BaseRepo, RepoErr};

pub struct UserService {}

impl UserService {
    pub async fn create_user(db: &DbConn, user: Json<CreateUser>) -> Result<Json<Value>, RepoErr> {
        UserRepository::new(db).create(user).await
    }

    pub async fn list_users(db: &DbConn) -> Result<Json<Value>, RepoErr> {
        UserRepository::new(db).find_all().await
    }

    pub async fn get_user(db: &DbConn, user_id: Path<Uuid>) -> Result<Json<Value>, RepoErr> {
        UserRepository::new(db).find_by_id(user_id.to_owned()).await
    }
}
