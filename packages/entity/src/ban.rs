use super::{deserialize_dt, serialize_dt};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "bans")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub active: bool, // if false, ban is inactive and should not be enforced, but is kept for record-keeping

    pub banner_user_id: Uuid,
    #[sea_orm(
        belongs_to,
        from = "banner_user_id",
        to = "id",
        relation_enum = "Banner"
    )]
    pub banner_user: HasOne<super::user::Entity>, // user who issued the ban, null if system ban

    pub post_id: Option<Uuid>, // post being banned.
    #[sea_orm(belongs_to, from = "post_id", to = "id", relation_enum = "Post")]
    pub post: HasOne<super::post::Entity>,

    pub file_id: Option<Uuid>, // file being banned.
    #[sea_orm(belongs_to, from = "file_id", to = "id")]
    pub file: HasOne<super::file::Entity>,

    pub user_id: Option<Uuid>, // user being banned.
    #[sea_orm(belongs_to, from = "user_id", to = "id", relation_enum = "User")]
    pub user: HasOne<super::user::Entity>,

    pub user_identifier_id: Option<Uuid>, // user being banned.
    #[sea_orm(belongs_to, from = "user_identifier_id", to = "id")]
    pub user_identifier: HasOne<super::user_identifier::Entity>,

    pub soft_ban: bool, // if true, content is only visible to staff and creator, not public.
    pub public_ban: bool, // stripped from public view except for ban info.
    #[sea_orm(column_type = "Text", nullable)]
    pub soft_reason: Option<String>, // visibile to staff and creator.
    #[sea_orm(column_type = "Text", nullable)]
    pub public_reason: Option<String>, // visibile to public, e.g. "Inappropriate content", "Spam", etc. Optional because some bans may be silent and not visible to public
    #[sea_orm(column_type = "Text", nullable)]
    pub warn_reason: Option<String>, // visibile to creator only
    #[sea_orm(column_type = "Text", nullable)]
    pub staff_note: Option<String>, // visibile to staff only

    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub additional_info: Option<Value>, // any additional info about the ban, e.g. evidence, notes, etc.
    pub expires: Option<DateTime<Utc>>,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
