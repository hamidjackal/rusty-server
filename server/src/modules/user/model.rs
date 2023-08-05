use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use sea_orm::entity::prelude::*;

use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid", unique, indexed)]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    #[sea_orm(nullable)]
    pub first_name: String,

    #[sea_orm(nullable)]
    pub last_name: String,

    #[sea_orm(indexed, unique)]
    pub email: String,
    pub password: String,
}

impl IntoResponse for Model {
    fn into_response(self) -> Response {
        let status = StatusCode::OK;
        let body = Json(json!({
            "success": true,
            "result": self.to_owned(),
        }));

        (status, body).into_response()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}
