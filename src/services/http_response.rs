use axum::{http::Error, response::Response, Json};

// use hyper::Body;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::{json, to_string, Value};

pub struct HttpError {}

pub struct HttpSuccess {}

#[derive(Serialize)]
pub struct ResponseStrcuture<T, J> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
    #[serde(flatten)]
    extra: Option<J>,
}

impl HttpError {
    pub fn not_found(msg: Option<String>) -> Result<Response<String>, Error> {
        let body = ResponseStrcuture {
            success: false,
            result: Some(msg.unwrap_or("Not found".to_string())),
            extra: Some(()),
        };

        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "application/json")
            .body(to_string(&body).unwrap())
            .unwrap())
    }

    // pub fn internal_server_error(msg: Option<String>) -> Result<Response<Body>, Error> {
    //     let body = ResponseStrcuture {
    //         success: false,
    //         message: Some(msg.unwrap_or("Internal server error".to_string())),
    //         extra: Some(()),
    //     };
    //     Ok(Response::builder()
    //         .status(StatusCode::INTERNAL_SERVER_ERROR)
    //         .header("Content-Type", "application/json")
    //         .body(Body::Text(to_string(&body).unwrap()))
    //         .unwrap())
    // }

    // pub fn invalid_request(msg: Option<String>) -> Result<Response<Body>, Error> {
    //     let body = ResponseStrcuture {
    //         success: false,
    //         message: Some(msg.unwrap_or("Invalid request".to_string())),
    //         extra: Some(()),
    //     };
    //     Ok(Response::builder()
    //         .status(StatusCode::BAD_REQUEST)
    //         .header("Content-Type", "application/json")
    //         .body(Body::Text(to_string(&body).unwrap()))
    //         .unwrap())
    // }
}

impl HttpSuccess {
    // pub fn success<T>(result: Option<T>) -> Result<Response<Body>, Error>
    // where
    //     T: Serialize,
    // {
    //     let body = ResponseStrcuture {
    //         success: true,
    //         result,
    //         extra: Some(()),
    //     };

    //     Ok(Response::builder()
    //         .status(StatusCode::OK)
    //         .header("Content-Type", "application/json")
    //         .body(Body::from(to_string(&body).unwrap()))
    //         .unwrap())
    // }

    pub fn success<T: Serialize>(result: Option<T>) -> Result<Json<Value>, Error> {
        Ok(Json(json!({
            "success": true,
            "result" : result,
            "extra": Some(()),
        })))
    }
}
