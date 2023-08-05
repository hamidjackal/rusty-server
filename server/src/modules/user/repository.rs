use axum::{async_trait, Json};
use sea_orm::prelude::Uuid;
use sea_orm::DbConn;
use sea_orm::*;
use serde_json::Value;

use crate::core::repository::{BaseRepo, RepoErr, RepoErrorType};
use crate::helpers::http_response::HttpSuccess;
use crate::modules::user::model::{self as user};

use super::controller::CreateUser;
pub struct UserRepository<'a> {
    db: &'a DbConn,
}

impl<'a> UserRepository<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }
}

#[async_trait]
impl BaseRepo<Json<Value>, CreateUser> for UserRepository<'_> {
    async fn create(&self, user: Json<CreateUser>) -> Result<Json<Value>, RepoErr> {
        let user_model = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            first_name: Set(user.first_name.clone()),
            last_name: Set(user.last_name.clone()),
            email: Set(user.email.clone()),
            password: Set(user.password.clone()),
        };
        let created_user = user_model.insert(self.db).await;
        match created_user {
            Ok(created_user) => Ok(HttpSuccess::single(created_user)),
            Err(e) => Err(RepoErr::BaseRepo(RepoErrorType::InvalidRequest {
                msg: Some(e.to_string()),
            })),
        }
    }

    async fn update(&self, id: Uuid, body: Json<CreateUser>) -> Result<Json<Value>, RepoErr> {
        unimplemented!()
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepoErr> {
        unimplemented!()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Json<Value>, RepoErr> {
        let user = user::Entity::find_by_id(id).one(self.db).await;
        let user = match user {
            Ok(user) => user,
            Err(_) => return Err(RepoErr::BaseRepo(RepoErrorType::InternalServerError)),
        };
        match user {
            Some(user) => Ok(HttpSuccess::single(user)),
            None => Err(RepoErr::BaseRepo(RepoErrorType::NotFound)),
        }
    }

    async fn find_all(&self) -> Result<Json<Value>, RepoErr> {
        let users = user::Entity::find().all(self.db).await;
        match users {
            Ok(users) => Ok(HttpSuccess::list(users)),
            Err(_) => Err(RepoErr::BaseRepo(RepoErrorType::InternalServerError)),
        }
    }
}
