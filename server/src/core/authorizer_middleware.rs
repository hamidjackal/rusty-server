use axum::{http, middleware::Next, response::Response};
use hyper::Request;
use sea_orm::prelude::Uuid;

use crate::modules::user::service::UserService;

use super::repository::{RepoErr, RepoErrorType};

pub struct UserAuthorizer {}

impl UserAuthorizer {
    pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, RepoErr> {
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        let auth_header = if let Some(auth_header) = auth_header {
            auth_header
        } else {
            return Err(RepoErr::BaseRepo(RepoErrorType::UnauthorizedRequest {
                msg: Some("No Authorization Header".to_string()),
            }));
        };

        if let Some(current_user) = Self::authorize_current_user(auth_header).await {
            req.extensions_mut().insert(current_user);
            Ok(next.run(req).await)
        } else {
            Err(RepoErr::BaseRepo(RepoErrorType::UnauthorizedRequest {
                msg: Some("Invalid Authorization Header".to_string()),
            }))
        }
    }

    async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
        let current_user = match UserService::verify_jwt_token(auth_token) {
            Ok(current_user) => current_user,
            Err(_) => return None,
        };

        return Some(current_user);
    }
}

#[derive(Clone)]
pub struct CurrentUser {
    pub id: Uuid,
}
