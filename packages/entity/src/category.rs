use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String, // e.g. "Lifestyle", "Media", "Tech"
    pub sort_order: u16,

    #[sea_orm(has_many)]
    pub boards: HasMany<super::board::Entity>,

    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
