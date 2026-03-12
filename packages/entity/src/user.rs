use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub is_ai: bool,
    pub is_human: bool,
    // HasBans
    #[sea_orm(has_many, relation_enum = "User", via_rel = "User")]
    pub bans: HasMany<super::ban::Entity>,

    #[sea_orm(has_one)]
    pub permissions: HasOne<super::user_permission::Entity>,

    #[sea_orm(has_one)]
    pub profile: HasOne<super::user_profile::Entity>,

    #[sea_orm(has_many)]
    pub posts: HasMany<super::post::Entity>,

    #[sea_orm(has_many)]
    pub files: HasMany<super::file::Entity>,

    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub last_login: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
