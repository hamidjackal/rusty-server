use axum::extract::Path;
use axum::Json;
use chrono::{Duration, Utc};
use josekit::jws::{JwsHeader, HS256};
use josekit::jwt::{self, JwtPayload};
use pwhash::bcrypt;
use sea_orm::{prelude::Uuid, DbConn};
use serde_json::{json, Value};
use std::str::FromStr;
use std::time::SystemTime;

use super::serializer::{CreateUser, LoginCredentials};
use super::{repository::UserRepository, serializer::UpdateUser};
use crate::core::authorizer_middleware::CurrentUser;
use crate::core::repository::{BaseRepo, RepoErr, RepoErrorType};
use crate::helpers::http_response::HttpSuccess;

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

    pub async fn delete_user(db: &DbConn, user_id: Path<Uuid>) -> Result<(), RepoErr> {
        UserRepository::new(db).delete(user_id.to_owned()).await
    }

    pub async fn update_user(
        db: &DbConn,
        user_id: Path<Uuid>,
        user: Json<UpdateUser>,
    ) -> Result<Json<Value>, RepoErr> {
        UserRepository::new(db)
            .update(user_id.to_owned(), user)
            .await
    }

    pub async fn login(
        db: &DbConn,
        credentials: Json<LoginCredentials>,
    ) -> Result<Json<Value>, RepoErr> {
        let user = UserRepository::new(db)
            .find_by_email(credentials.email.clone())
            .await?;
        if bcrypt::verify(credentials.password.clone(), &user.password) == false {
            return Err(RepoErr::BaseRepo(RepoErrorType::InvalidRequest {
                msg: None,
            }));
        }

        let jwt = Self::generate_jwt_token(user.id.to_string())?;

        return Ok(HttpSuccess::single(json!({
            "id": user.id,
            "email": user.email,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "token": jwt,
        })));
    }

    fn generate_jwt_token(id: String) -> Result<String, RepoErr> {
        let mut header = JwsHeader::new();
        header.set_token_type("JWT");

        let payload = Self::get_payload(id);

        let key = b"0123456789ABCDEF0123456789ABCDEF";

        let signer = match HS256.signer_from_bytes(key) {
            Ok(signer) => signer,
            Err(_) => return Err(RepoErr::BaseRepo(RepoErrorType::InternalServerError)),
        };
        let jwt = match jwt::encode_with_signer(&payload, &header, &signer) {
            Ok(jwt) => jwt,
            Err(_) => return Err(RepoErr::BaseRepo(RepoErrorType::InternalServerError)),
        };

        Ok(jwt)
    }

    fn get_payload(id: String) -> JwtPayload {
        let json_payload = json!({"id": id});

        let mut jwt_payload = JwtPayload::new();
        jwt_payload.set_expires_at(&SystemTime::from(Utc::now() + Duration::minutes(2)));
        jwt_payload
            .set_claim("id", Some(json_payload["id"].clone()))
            .unwrap();

        return jwt_payload;
    }

    pub fn verify_jwt_token(token: &str) -> Result<CurrentUser, RepoErr> {
        let key = b"0123456789ABCDEF0123456789ABCDEF";

        let verifier = match HS256.verifier_from_bytes(key) {
            Ok(verifier) => verifier,
            Err(_) => return Err(RepoErr::BaseRepo(RepoErrorType::InternalServerError)),
        };

        let (payload, _) = match jwt::decode_with_verifier(&token, &verifier) {
            Ok(jwt) => jwt,
            Err(_) => return Err(RepoErr::BaseRepo(RepoErrorType::InternalServerError)),
        };

        if payload.expires_at().unwrap() < SystemTime::from(Utc::now()) {
            return Err(RepoErr::BaseRepo(RepoErrorType::InvalidRequest {
                msg: Some("Token expired".to_string()),
            }));
        }

        let id = payload.claim("id").unwrap();

        Ok(CurrentUser {
            id: Uuid::from_str(id.as_str().unwrap()).unwrap(),
        })
    }
}
