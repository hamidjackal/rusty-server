use axum::{
    async_trait,
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use sea_orm::prelude::Uuid;
use serde_json::json;

#[async_trait]
pub trait BaseRepo<T, J, K> {
    async fn create(&self, body: Json<J>) -> Result<T, RepoErr>;
    async fn update(&self, id: Uuid, body: Json<K>) -> Result<T, RepoErr>;
    async fn delete(&self, id: Uuid) -> Result<(), RepoErr>;
    async fn find_by_id(&self, id: Uuid) -> Result<T, RepoErr>;
    async fn find_all(&self) -> Result<T, RepoErr>;
}

pub enum RepoErr {
    BaseRepo(RepoErrorType),
}

impl From<RepoErrorType> for RepoErr {
    fn from(err: RepoErrorType) -> Self {
        RepoErr::BaseRepo(err)
    }
}

impl IntoResponse for RepoErr {
    fn into_response(self) -> Response {
        let message: String;
        let (status, error_message) = match self {
            RepoErr::BaseRepo(RepoErrorType::NotFound) => (StatusCode::NOT_FOUND, "Not found"),
            RepoErr::BaseRepo(RepoErrorType::InternalServerError) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            RepoErr::BaseRepo(RepoErrorType::InvalidRequest { msg }) => {
                message = msg.unwrap_or("Invalid request".to_string());
                (StatusCode::BAD_REQUEST, message.as_str())
            }
            RepoErr::BaseRepo(RepoErrorType::UnauthorizedRequest { msg }) => {
                message = msg.unwrap_or("Unauthorized request".to_string());
                (StatusCode::UNAUTHORIZED, message.as_str())
            }
        };

        let body = Json(json!({
            "success": false,
            "result": error_message,
        }));

        (status, body).into_response()
    }
}

#[derive(Debug)]
pub enum RepoErrorType {
    NotFound,
    InternalServerError,
    InvalidRequest { msg: Option<String> },
    UnauthorizedRequest { msg: Option<String> },
}

pub enum ValidationErr {
    BaseRepo(ValidationErrorType),
}

impl From<ValidationErrorType> for ValidationErr {
    fn from(err: ValidationErrorType) -> Self {
        ValidationErr::BaseRepo(err)
    }
}

impl IntoResponse for ValidationErr {
    fn into_response(self) -> Response {
        let message: Vec<String>;
        let (status, error_message) = match self {
            ValidationErr::BaseRepo(ValidationErrorType::InvalidInput { msg }) => {
                message = vec!["Invalid input".to_string()];
                (StatusCode::BAD_REQUEST, msg.unwrap_or(message))
            }
        };

        let body = Json(json!({
            "success": false,
            "result": error_message,
        }));

        (status, body).into_response()
    }
}

#[derive(Debug)]
pub enum ValidationErrorType {
    InvalidInput { msg: Option<Vec<String>> },
}
