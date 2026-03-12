use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    pub user_id: Uuid,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    pub permisions: Value,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
