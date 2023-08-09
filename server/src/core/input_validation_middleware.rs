use serde::de::DeserializeOwned;
use serde_json::from_slice;
use std::fmt::Debug;

use axum::{
    body::{Body, Bytes},
    middleware::Next,
    response::Response,
};
use hyper::Request;

use validator::{Validate, ValidationError, ValidationErrors};

use super::repository::{ValidationErr, ValidationErrorType};

pub struct BodyValidator {}

impl BodyValidator {
    pub async fn validate<U>(
        req: Request<Body>,
        next: Next<Body>,
    ) -> Result<Response, ValidationErr>
    where
        U: DeserializeOwned + Validate + Debug,
    {
        let (parts, body) = req.into_parts();
        let bytes_body = Self::convert_body_to_bytes(body).await?;

        let mut cloned_body = Bytes::new();
        bytes_body.clone_into(&mut cloned_body);

        let deserialized_body: U = Self::deserialize_body(cloned_body).await?;
        Self::validate_body(deserialized_body).await?;

        let body = Body::from(bytes_body);

        let req = Request::from_parts(parts, body);
        Ok(next.run(req).await)
    }

    async fn convert_body_to_bytes(body: Body) -> Result<Bytes, ValidationErr> {
        let my_body = hyper::body::to_bytes(body).await;
        match my_body {
            Ok(my_body) => Ok(my_body),
            Err(e) => {
                println!("Convert Error: {:?}", e);
                return Err(ValidationErr::BaseRepo(ValidationErrorType::InvalidInput {
                    msg: None,
                }));
            }
        }
    }

    async fn deserialize_body<U: DeserializeOwned>(body: Bytes) -> Result<U, ValidationErr> {
        let new_body = from_slice::<U>(body.as_ref());
        match new_body {
            Ok(new_body) => Ok(new_body),
            Err(e) => {
                return Err(ValidationErr::BaseRepo(ValidationErrorType::InvalidInput {
                    msg: Some(vec![e.to_string()]),
                }));
            }
        }
    }

    async fn validate_body<U: Validate>(deserialized_body: U) -> Result<(), ValidationErr> {
        match deserialized_body.validate() {
            Ok(_) => Ok(()),
            Err(e) => {
                let err_messages = Self::get_error_messages(e);
                return Err(ValidationErr::BaseRepo(ValidationErrorType::InvalidInput {
                    msg: Some(err_messages),
                }));
            }
        }
    }

    fn get_error_messages(errors: ValidationErrors) -> Vec<String> {
        errors
            .field_errors()
            .iter()
            .map(|(_, v)| Self::extract_error_message(v))
            .collect::<Vec<String>>()
    }

    fn extract_error_message(errors: &&Vec<ValidationError>) -> String {
        let messages = errors
            .iter()
            .map(|e| e.message.clone().unwrap().to_string())
            .collect::<Vec<String>>();
        messages.join(", ")
    }
}
