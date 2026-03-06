use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub post_id: Uuid,
    #[sea_orm(belongs_to, from = "post_id", to = "id")]
    pub post: HasOne<super::post::Entity>,

    pub user_id: Option<Uuid>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    pub user_identifier_id: Option<Uuid>, // HasUserIdentifier
    #[sea_orm(belongs_to, from = "user_identifier_id", to = "id")]
    pub user_identifier: HasOne<super::user_identifier::Entity>,

    pub approved: bool, // manually approved by staff, available on site (unless banned later)
    pub rejected: bool, // auto rejected by filters not available on site
    pub deleted: bool,  // not available on site

    // HasBans
    #[sea_orm(has_many)]
    pub bans: HasMany<super::ban::Entity>,

    pub spoiler: bool, // whether the file is marked as a spoiler, if true, the file will be blurred and require user interaction to view

    pub description: Option<String>,
    pub file_display_name: String,
    pub gallery_order: Option<u32>,
    pub file_size: u64, // size of the file after processing (resizing, compression, etc), this is the size of the file that is actually available on the site
    pub file_dimensions: String,
    pub file_original_name: String,
    pub file_original_size: u64, // size of the file uploaded to us

    pub additional_info: Option<Value>, // exif, ai analysis scores, etc.
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
