use axum::Json;
use sea_orm::ActiveValue::{self, Set};
use sea_orm::{prelude::Uuid, ActiveValue::NotSet};
use serde::Deserialize;
use validator::Validate;

use super::model::ActiveModel;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct UpdateUser {
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

pub struct UserSerializer;

impl UserSerializer {
    pub fn serialize_create(user: Json<CreateUser>) -> ActiveModel {
        ActiveModel {
            id: Set(Uuid::new_v4()),
            first_name: Set(user.first_name.clone()),
            last_name: Set(user.last_name.clone()),
            email: Set(user.email.clone()),
            password: Set(user.password.clone()),
        }
    }

    pub fn serialize_update(user: Json<CreateUser>) -> ActiveModel {
        ActiveModel {
            id: NotSet,
            first_name: Self::get_update_value(user.first_name.clone()),
            last_name: Self::get_update_value(user.last_name.clone()),
            email: NotSet,
            password: Self::get_update_value(user.password.clone()),
        }
    }

    fn get_update_value(value: String) -> ActiveValue<String> {
        match value.is_empty() {
            true => NotSet,
            false => Set(value.clone()),
        }
    }
}
