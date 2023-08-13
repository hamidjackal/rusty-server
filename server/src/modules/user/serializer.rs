use axum::Json;
use pwhash::bcrypt;
use sea_orm::ActiveValue::{self, Set};
use sea_orm::{prelude::Uuid, ActiveValue::NotSet};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::model::ActiveModel;

#[derive(Deserialize, Validate, Debug, Clone, Serialize)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "Last name is required"))]
    pub last_name: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct UpdateUser {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub password: Option<String>,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct ChangePassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

pub struct UserSerializer;

impl UserSerializer {
    pub fn serialize_create(user: Json<CreateUser>) -> ActiveModel {
        let hashed_password = bcrypt::hash(user.password.clone()).unwrap();
        ActiveModel {
            id: Set(Uuid::new_v4()),
            first_name: Set(user.first_name.clone()),
            last_name: Set(user.last_name.clone()),
            email: Set(user.email.clone()),
            password: Set(hashed_password),
        }
    }

    pub fn serialize_update(user: Json<UpdateUser>) -> ActiveModel {
        ActiveModel {
            id: NotSet,
            first_name: Self::get_update_value(user.first_name.clone()),
            last_name: Self::get_update_value(user.last_name.clone()),
            email: NotSet,
            password: Self::get_updated_password_value(user.password.clone()),
        }
    }

    fn get_updated_password_value(value: Option<String>) -> ActiveValue<String> {
        match value {
            Some(value) => {
                let hashed_password = bcrypt::hash(value).unwrap();
                Set(hashed_password)
            }
            None => NotSet,
        }
    }

    fn get_update_value(value: Option<String>) -> ActiveValue<String> {
        match value {
            Some(value) => Set(value),
            None => NotSet,
        }
    }
}
