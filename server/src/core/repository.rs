use axum::{
    async_trait,
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use sea_orm::prelude::Uuid;
use serde_json::{json, Value};

#[async_trait]
pub trait BaseRepo<T, J> {
    async fn create(&self, body: Json<J>) -> Result<T, RepoErr>;
    async fn update(&self, id: Uuid, body: Json<J>) -> Result<T, RepoErr>;
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
}
