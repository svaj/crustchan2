use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "threads")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub subject: String,

    pub board_id: Uuid,
    #[sea_orm(belongs_to, from = "board_id", to = "id")]
    pub board: HasOne<super::board::Entity>,

    pub user_id: Option<Uuid>, // id of the user who made the post, null if not logged in
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    pub user_identifier_id: Option<Uuid>, // user being banned.
    #[sea_orm(belongs_to, from = "user_identifier_id", to = "id")]
    pub user_identifier: HasOne<super::user_identifier::Entity>,

    #[sea_orm(has_many)]
    pub posts: HasMany<super::post::Entity>,

    pub deleted: bool, // if true, post is deleted and only visible to admins, does not show for users
    pub ai_slop: bool, // if true, flags as ai-generated content
    pub approved: bool, // manually approved by mods, visible to everyone, bypasses filters
    pub rejected: bool, // auto rejected by filters or manually rejected by mods, not visible to anyone
    pub locked: bool,
    pub sticky: bool,

    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub last_bumped_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub last_replied_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
