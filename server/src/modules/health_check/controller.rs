use axum::response::Response;
use hyper::Body;

use crate::helpers::http_response::HttpSuccess;

pub async fn health_check() -> Response<Body> {
    HttpSuccess::success("Success").unwrap()
}
