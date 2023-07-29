use axum::Json;
use serde_json::Value;

use crate::services::http_response::HttpSuccess;

pub async fn health_check() -> Json<Value> {
    HttpSuccess::success(Some("Success")).unwrap()
}
